use super::*;
use librett_application::{
    ReadyMatch, ScheduleAction, ScheduleRequest, ScheduleState, TableAssignment,
};
use librett_domain::{Entry, ScheduledMatch};
use std::collections::HashSet;

fn named(entry: &Entry) -> String {
    entry
        .members
        .iter()
        .map(|member| {
            let club: String = member
                .club
                .chars()
                .filter(|c| c.is_alphanumeric())
                .take(3)
                .flat_map(char::to_uppercase)
                .collect();
            if club.is_empty() {
                member.name.clone()
            } else {
                format!("{} ({club})", member.name)
            }
        })
        .collect::<Vec<_>>()
        .join(" / ")
}
fn ready_match(
    draw: &librett_domain::CategoryDraw,
    category_name: &str,
    item: &ScheduledMatch,
) -> Option<ReadyMatch> {
    if item.bye || item.result.is_some() {
        return None;
    }
    match_details(draw, category_name, item)
}
fn match_details(
    draw: &librett_domain::CategoryDraw,
    category_name: &str,
    item: &ScheduledMatch,
) -> Option<ReadyMatch> {
    let (first, second) = (item.first?, item.second?);
    let a = draw.participants.iter().find(|e| e.id == first)?;
    let b = draw.participants.iter().find(|e| e.id == second)?;
    Some(ReadyMatch {
        category_id: draw.category_id,
        category_name: category_name.into(),
        draw_id: draw.id,
        key: item.key.clone(),
        phase: if item.key.starts_with("group:") {
            "groups"
        } else {
            "knockout"
        }
        .into(),
        round: item.round,
        first,
        second,
        first_name: named(a),
        second_name: named(b),
        players: a.members.iter().chain(&b.members).map(|m| m.id).collect(),
    })
}
fn snapshot(conn: &Connection, tournament: Uuid) -> Result<ScheduleState, ApplicationError> {
    let parent = super::completion::state(conn, "tournaments", tournament)?;
    let config: Option<(usize, u32)> = conn
        .query_row(
            "SELECT table_count,revision FROM tournament_scheduling WHERE tournament_id=?1",
            [tournament.to_string()],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()
        .map_err(|_| ApplicationError::Storage)?;
    let (table_count, revision) = config.unwrap_or((0, 0));
    let mut query=conn.prepare("SELECT table_number,status,payload,started_at FROM match_assignments WHERE tournament_id=?1 AND status!='released' ORDER BY table_number").map_err(|_|ApplicationError::Storage)?;
    let rows = query
        .query_map([tournament.to_string()], |r| {
            Ok((
                r.get::<_, usize>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, Option<String>>(3)?,
            ))
        })
        .map_err(|_| ApplicationError::Storage)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ApplicationError::Storage)?;
    let mut stored = vec![];
    for (table, status, json, started_at) in rows {
        let scheduled =
            serde_json::from_str::<ReadyMatch>(&json).map_err(|_| ApplicationError::Storage)?;
        stored.push(TableAssignment {
            table,
            status,
            scheduled,
            started_at,
        });
    }
    let mut query=conn.prepare("SELECT id,name,completed_at IS NOT NULL FROM categories WHERE tournament_id=?1 AND archived=0 ORDER BY rowid").map_err(|_|ApplicationError::Storage)?;
    let categories = query
        .query_map([tournament.to_string()], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, bool>(2)?,
            ))
        })
        .map_err(|_| ApplicationError::Storage)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ApplicationError::Storage)?;
    let mut ready = vec![];
    let mut versions = vec![];
    let mut truncated = false;
    for (id, name, closed) in categories {
        let id = Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?;
        let state = super::matches::snapshot(conn, tournament, id, None)?;
        let competition = super::matches::competition(&state);
        versions.push(
            serde_json::to_string(&(
                id,
                closed,
                competition.draw_id,
                competition.stale,
                competition.match_version,
                competition.rules_revision,
                competition.filler_revision,
                &competition.order_revisions,
            ))
            .map_err(|_| ApplicationError::Storage)?,
        );
        if closed || parent.completed_at.is_some() || competition.stale {
            continue;
        }
        let Some(draw) = state.draw.as_ref() else {
            continue;
        };
        // Always project occupied matches, even outside the bounded waiting list.
        for assignment in stored.iter().filter(|a| a.scheduled.draw_id == draw.id) {
            let (group, round, _) = super::matches::key_parts(&assignment.scheduled.key)?;
            for item in super::matches::matches_for(
                &state,
                group,
                round,
                assignment.scheduled.key.starts_with("ko:"),
            ) {
                if item.key == assignment.scheduled.key {
                    if let Some(item) = ready_match(draw, &name, &item) {
                        ready.push(item);
                    }
                }
            }
        }
        let mut added = 0;
        if draw.format == CompetitionFormat::GroupsKnockout {
            'groups: for (group, section) in draw.sections.iter().enumerate() {
                let n = section.iter().flatten().count();
                let rounds = if n < 2 { 0 } else { n + n % 2 - 1 };
                for round in 0..rounds {
                    for item in super::matches::matches_for(&state, group, round, false) {
                        if let Some(item) = ready_match(draw, &name, &item) {
                            if !ready
                                .iter()
                                .any(|r| r.draw_id == item.draw_id && r.key == item.key)
                            {
                                ready.push(item);
                                added += 1;
                            }
                            if added >= 100 || ready.len() >= 1000 {
                                truncated = true;
                                break 'groups;
                            }
                        }
                    }
                }
            }
        }
        for item in &competition.matches {
            if let Some(item) = ready_match(draw, &name, item) {
                if !ready
                    .iter()
                    .any(|r| r.draw_id == item.draw_id && r.key == item.key)
                {
                    if added >= 100 || ready.len() >= 1000 {
                        truncated = true;
                        break;
                    }
                    ready.push(item);
                    added += 1;
                }
            }
        }
    }
    let assignments: Vec<_> = stored
        .into_iter()
        .filter(|a| {
            ready.iter().any(|r| {
                r.draw_id == a.scheduled.draw_id
                    && r.key == a.scheduled.key
                    && r.first == a.scheduled.first
                    && r.second == a.scheduled.second
            })
        })
        .collect();
    let waiting = ready
        .into_iter()
        .filter(|r| {
            !assignments
                .iter()
                .any(|a| a.scheduled.draw_id == r.draw_id && a.scheduled.key == r.key)
        })
        .collect();
    let version = serde_json::to_string(&(revision, table_count, parent.revision, versions))
        .map_err(|_| ApplicationError::Storage)?;
    Ok(ScheduleState {
        tournament_id: tournament,
        table_count,
        version,
        assignments,
        waiting,
        truncated,
    })
}
fn assign(
    conn: &Connection,
    tournament: Uuid,
    ready: &ReadyMatch,
    table: usize,
    completed: bool,
) -> Result<(), ApplicationError> {
    conn.execute("INSERT INTO match_assignments(tournament_id,draw_id,match_key,table_number,status,payload,started_at) VALUES(?1,?2,?3,?4,?6,?5,NULL) ON CONFLICT(draw_id,match_key) DO UPDATE SET table_number=excluded.table_number,status=excluded.status,payload=excluded.payload,started_at=NULL",params![tournament.to_string(),ready.draw_id.to_string(),ready.key,table,serde_json::to_string(ready).map_err(|_|ApplicationError::Storage)?,if completed{"released"}else{"queued"}]).map_err(|_|ApplicationError::Storage)?;
    Ok(())
}
impl SqliteTournamentRepository {
    pub fn match_tables(
        &self,
        tournament: Uuid,
        draw: Uuid,
    ) -> Result<std::collections::HashMap<String, usize>, ApplicationError> {
        let mut query=self.connection.prepare("SELECT match_key,table_number FROM match_assignments WHERE tournament_id=?1 AND draw_id=?2").map_err(|_|ApplicationError::Storage)?;
        let rows = query
            .query_map(params![tournament.to_string(), draw.to_string()], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, usize>(1)?))
            })
            .map_err(|_| ApplicationError::Storage)?;
        rows.collect::<Result<std::collections::HashMap<_, _>, _>>()
            .map_err(|_| ApplicationError::Storage)
    }
    pub fn schedule(&mut self, tournament: Uuid) -> Result<ScheduleState, ApplicationError> {
        let tx = self
            .connection
            .transaction()
            .map_err(|_| ApplicationError::Storage)?;
        snapshot(&tx, tournament)
    }
    pub fn change_schedule(
        &mut self,
        request: ScheduleRequest,
    ) -> Result<ScheduleState, ApplicationError> {
        let json = serde_json::to_string(&request).map_err(|_| ApplicationError::Storage)?;
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        let old: Option<(String, String)> = tx
            .query_row(
                "SELECT request_payload,result_payload FROM schedule_writes WHERE id=?1",
                [request.request_id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?;
        if let Some((prior, result)) = old {
            return if prior == json {
                serde_json::from_str(&result).map_err(|_| ApplicationError::Storage)
            } else {
                Err(ApplicationError::ScheduleConflict)
            };
        }
        super::completion::ensure_tournament_open(&tx, request.tournament_id)?;
        let current = snapshot(&tx, request.tournament_id)?;
        if current.version != request.expected_version {
            return Err(ApplicationError::ScheduleConflict);
        }
        // Finished or invalidated matches release their table before a new assignment.
        let keep: HashSet<_> = current
            .assignments
            .iter()
            .map(|a| (a.scheduled.draw_id.to_string(), a.scheduled.key.clone()))
            .collect();
        let mut query=tx.prepare("SELECT draw_id,match_key FROM match_assignments WHERE tournament_id=?1 AND status!='released'").map_err(|_|ApplicationError::Storage)?;
        let occupied = query
            .query_map([request.tournament_id.to_string()], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
            })
            .map_err(|_| ApplicationError::Storage)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ApplicationError::Storage)?;
        drop(query);
        for (draw, key) in occupied {
            if !keep.contains(&(draw.clone(), key.clone())) {
                tx.execute("UPDATE match_assignments SET status='released' WHERE draw_id=?1 AND match_key=?2",params![draw,key]).map_err(|_|ApplicationError::Storage)?;
            }
        }
        match request.action {
            ScheduleAction::Configure { table_count } => {
                if table_count > 128 || current.assignments.iter().any(|a| a.table > table_count) {
                    return Err(ApplicationError::TableBusy);
                }
                tx.execute("INSERT INTO tournament_scheduling(tournament_id,table_count,revision) VALUES(?1,?2,0) ON CONFLICT(tournament_id) DO UPDATE SET table_count=excluded.table_count",params![request.tournament_id.to_string(),table_count]).map_err(|_|ApplicationError::Storage)?;
            }
            ScheduleAction::Assign {
                draw_id,
                ref key,
                table,
            } => {
                if table == 0 || table > 128 {
                    return Err(ApplicationError::TableBusy);
                }
                let category: String = tx
                    .query_row(
                        "SELECT category_id FROM category_draws WHERE id=?1",
                        [draw_id.to_string()],
                        |r| r.get(0),
                    )
                    .optional()
                    .map_err(|_| ApplicationError::Storage)?
                    .ok_or(ApplicationError::ScheduleConflict)?;
                let category = Uuid::parse_str(&category).map_err(|_| ApplicationError::Storage)?;
                super::completion::ensure_category_open(&tx, category)?;
                let state = super::matches::snapshot(&tx, request.tournament_id, category, None)?;
                let competition = super::matches::competition(&state);
                if competition.stale || competition.draw_id != Some(draw_id) {
                    return Err(ApplicationError::ScheduleConflict);
                }
                let draw = state
                    .draw
                    .as_ref()
                    .ok_or(ApplicationError::ScheduleConflict)?;
                let category_name: String = tx
                    .query_row(
                        "SELECT name FROM categories WHERE id=?1",
                        [category.to_string()],
                        |r| r.get(0),
                    )
                    .map_err(|_| ApplicationError::Storage)?;
                let (group, round, _) = super::matches::key_parts(key)?;
                let scheduled =
                    super::matches::matches_for(&state, group, round, key.starts_with("ko:"))
                        .into_iter()
                        .find(|m| m.key == *key)
                        .ok_or(ApplicationError::ScheduleConflict)?;
                let completed = scheduled.result.is_some();
                let item = match_details(draw, &category_name, &scheduled)
                    .ok_or(ApplicationError::ScheduleConflict)?;
                if !completed {
                    if current.assignments.iter().any(|a| {
                        a.table == table
                            && (a.scheduled.draw_id != draw_id || a.scheduled.key != *key)
                    }) {
                        return Err(ApplicationError::TableBusy);
                    }
                    if current
                        .assignments
                        .iter()
                        .filter(|a| a.scheduled.draw_id != draw_id || a.scheduled.key != *key)
                        .any(|a| {
                            a.scheduled
                                .players
                                .iter()
                                .any(|id| item.players.contains(id))
                        })
                    {
                        return Err(ApplicationError::PlayerBusy);
                    }
                }
                tx.execute("INSERT INTO tournament_scheduling(tournament_id,table_count,revision) VALUES(?1,?2,0) ON CONFLICT(tournament_id) DO UPDATE SET table_count=MAX(table_count,excluded.table_count)",params![request.tournament_id.to_string(),table]).map_err(|_|ApplicationError::Storage)?;
                assign(&tx, request.tournament_id, &item, table, completed)?;
            }
            ScheduleAction::Remove { draw_id, ref key } => {
                let category:String=tx.query_row("SELECT category_id FROM category_draws WHERE id=?1 AND category_id IN(SELECT id FROM categories WHERE tournament_id=?2)",params![draw_id.to_string(),request.tournament_id.to_string()],|r|r.get(0)).optional().map_err(|_|ApplicationError::Storage)?.ok_or(ApplicationError::NotFound)?;
                super::completion::ensure_category_open(
                    &tx,
                    Uuid::parse_str(&category).map_err(|_| ApplicationError::Storage)?,
                )?;
                tx.execute("DELETE FROM match_assignments WHERE tournament_id=?1 AND draw_id=?2 AND match_key=?3",params![request.tournament_id.to_string(),draw_id.to_string(),key]).map_err(|_|ApplicationError::Storage)?;
            }
            ScheduleAction::Start { table } | ScheduleAction::Clear { table } => {
                if !current.assignments.iter().any(|a| a.table == table) {
                    return Err(ApplicationError::ScheduleConflict);
                }
                if matches!(request.action, ScheduleAction::Start { .. }) {
                    let assignment = current
                        .assignments
                        .iter()
                        .find(|a| a.table == table)
                        .ok_or(ApplicationError::ScheduleConflict)?;
                    super::registration::ensure_start_attendance(
                        &tx,
                        request.tournament_id,
                        assignment.scheduled.category_id,
                    )?;
                }
                let query = if matches!(request.action, ScheduleAction::Start { .. }) {
                    "UPDATE match_assignments SET status='running',started_at=strftime('%Y-%m-%dT%H:%M:%fZ','now') WHERE tournament_id=?1 AND table_number=?2 AND status='queued'"
                } else {
                    "UPDATE match_assignments SET status='released' WHERE tournament_id=?1 AND table_number=?2 AND status!='released'"
                };
                if tx
                    .execute(query, params![request.tournament_id.to_string(), table])
                    .map_err(|_| ApplicationError::Storage)?
                    == 0
                {
                    return Err(ApplicationError::ScheduleConflict);
                }
            }
        }
        tx.execute(
            "UPDATE tournament_scheduling SET revision=revision+1 WHERE tournament_id=?1",
            [request.tournament_id.to_string()],
        )
        .map_err(|_| ApplicationError::Storage)?;
        let result = snapshot(&tx, request.tournament_id)?;
        tx.execute(
            "INSERT INTO schedule_writes(id,request_payload,result_payload) VALUES(?1,?2,?3)",
            params![
                request.request_id.to_string(),
                json,
                serde_json::to_string(&result).map_err(|_| ApplicationError::Storage)?
            ],
        )
        .map_err(|_| ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)?;
        Ok(result)
    }
}
