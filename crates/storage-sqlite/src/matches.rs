use super::*;
use librett_application::{CompetitionState, GroupOrderRequest, MatchPage, SaveMatchRequest};
use librett_domain::{
    group_round_matches, knockout_matches, CategoryDraw, CategoryRules, ScheduledMatch,
    StoredMatchResult,
};
use std::collections::{HashMap, HashSet};

struct Snapshot {
    draw: Option<CategoryDraw>,
    rules: CategoryRules,
    revision: u32,
    stale: bool,
    results: HashMap<String, StoredMatchResult>,
    orders: HashMap<usize, Vec<Uuid>>,
    order_revisions: HashMap<usize, u32>,
    fillers: HashMap<usize, librett_domain::FillerChoice>,
    filler_revision: u32,
}
fn snapshot(
    conn: &Connection,
    tournament: Uuid,
    category: Uuid,
    _group_round: Option<(usize, usize)>,
) -> Result<Snapshot, ApplicationError> {
    let format: Option<String> = conn
        .query_row(
            "SELECT format FROM categories WHERE id=?1 AND tournament_id=?2 AND archived=0",
            params![category.to_string(), tournament.to_string()],
            |r| r.get(0),
        )
        .optional()
        .map_err(|_| ApplicationError::Storage)?;
    let format = format.ok_or(ApplicationError::NotFound)?;
    let configuration: Option<(u32, String)> = conn
        .query_row(
            "SELECT revision,payload FROM category_configurations WHERE category_id=?1",
            [category.to_string()],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()
        .map_err(|_| ApplicationError::Storage)?;
    let (revision, rules) = match configuration {
        Some((revision, json)) => (
            revision,
            serde_json::from_str::<CategoryRules>(&json).map_err(|_| ApplicationError::Storage)?,
        ),
        None => (0, CategoryRules::default()),
    };
    let json: Option<String> = conn.query_row("SELECT payload FROM category_draws WHERE category_id=?1 ORDER BY revision DESC LIMIT 1", [category.to_string()], |r| r.get(0)).optional().map_err(|_| ApplicationError::Storage)?;
    let draw: Option<CategoryDraw> = json
        .map(|json| serde_json::from_str(&json).map_err(|_| ApplicationError::Storage))
        .transpose()?;
    let mut results = HashMap::new();
    let mut stale = false;
    if let Some(draw) = &draw {
        let mut entries = conn
            .prepare("SELECT id FROM entries WHERE category_id=?1 AND status='registered'")
            .map_err(|_| ApplicationError::Storage)?;
        let active = entries
            .query_map([category.to_string()], |r| r.get::<_, String>(0))
            .map_err(|_| ApplicationError::Storage)?
            .collect::<Result<HashSet<_>, _>>()
            .map_err(|_| ApplicationError::Storage)?;
        let assigned: HashSet<_> = draw.sections.iter().flatten().flatten().copied().collect();
        stale = active != draw.participants.iter().map(|e| e.id.to_string()).collect()
            || assigned.len() != draw.participants.len()
            || format
                != match draw.format {
                    CompetitionFormat::Knockout => "knockout",
                    CompetitionFormat::GroupsKnockout => "groups_knockout",
                }
            || draw.format == CompetitionFormat::GroupsKnockout
                && (rules.group_count != draw.settings.group_count
                    || rules.qualifiers_per_group != draw.settings.qualifiers_per_group);
        let prefix: Option<String> = None;
        let mut query = conn.prepare("SELECT m.match_key,m.revision,m.payload FROM match_results m WHERE m.draw_id=?1 AND (?2 IS NULL OR m.match_key LIKE ?2) AND m.id=(SELECT MAX(n.id) FROM match_results n WHERE n.draw_id=m.draw_id AND n.match_key=m.match_key)").map_err(|_| ApplicationError::Storage)?;
        let rows = query
            .query_map(params![draw.id.to_string(), prefix], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, u32>(1)?,
                    r.get::<_, String>(2)?,
                ))
            })
            .map_err(|_| ApplicationError::Storage)?;
        for row in rows {
            let (key, revision, payload) = row.map_err(|_| ApplicationError::Storage)?;
            let result = serde_json::from_str(&payload).map_err(|_| ApplicationError::Storage)?;
            results.insert(
                key.clone(),
                StoredMatchResult {
                    key,
                    revision,
                    result,
                },
            );
        }
    }
    let mut orders = HashMap::new();
    let mut order_revisions = HashMap::new();
    if let Some(draw) = &draw {
        let mut stmt = conn
            .prepare("SELECT group_number,revision,payload FROM group_orders WHERE draw_id=?1")
            .map_err(|_| ApplicationError::Storage)?;
        let rows = stmt
            .query_map([draw.id.to_string()], |r| {
                Ok((
                    r.get::<_, usize>(0)?,
                    r.get::<_, u32>(1)?,
                    r.get::<_, String>(2)?,
                ))
            })
            .map_err(|_| ApplicationError::Storage)?;
        for row in rows {
            let (group, revision, json) = row.map_err(|_| ApplicationError::Storage)?;
            let order: Option<Vec<Uuid>> =
                serde_json::from_str(&json).map_err(|_| ApplicationError::Storage)?;
            if let Some(order) = order {
                orders.insert(group, order);
            }
            order_revisions.insert(group, revision);
        }
    }
    let saved_fillers: Option<(u32, String)> = if let Some(draw) = &draw {
        conn.query_row(
            "SELECT revision,payload FROM knockout_fillers WHERE draw_id=?1",
            [draw.id.to_string()],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .optional()
        .map_err(|_| ApplicationError::Storage)?
    } else {
        None
    };
    let (filler_revision, fillers) = match saved_fillers {
        Some((revision, json)) => (
            revision,
            serde_json::from_str(&json).map_err(|_| ApplicationError::Storage)?,
        ),
        None => (0, HashMap::new()),
    };
    Ok(Snapshot {
        fillers,
        filler_revision,
        draw,
        rules,
        revision,
        stale,
        results,
        orders,
        order_revisions,
    })
}
fn competition(state: &Snapshot) -> CompetitionState {
    let Some(draw) = &state.draw else {
        return CompetitionState {
            match_version: state.results.values().map(|r| u64::from(r.revision)).sum(),
            filler_revision: state.filler_revision,
            rules_revision: state.revision,
            fillers: state.fillers.clone(),
            candidates: vec![],
            draw_id: None,
            stale: state.stale,
            groups: vec![],
            slots: vec![],
            matches: vec![],
            order_revisions: vec![],
            result_versions: vec![],
        };
    };
    if draw.format == CompetitionFormat::Knockout {
        return CompetitionState {
            match_version: state.results.values().map(|r| u64::from(r.revision)).sum(),
            filler_revision: state.filler_revision,
            rules_revision: state.revision,
            fillers: state.fillers.clone(),
            candidates: vec![],
            draw_id: Some(draw.id),
            stale: state.stale,
            groups: vec![],
            slots: vec![],
            matches: knockout_matches(draw, &state.results),
            order_revisions: vec![],
            result_versions: vec![],
        };
    }
    let groups = librett_domain::group_standings(draw, &state.rules, &state.results, &state.orders);
    let slots = librett_domain::filled_qualification_slots(
        draw,
        &groups,
        state.rules.knockout_filling,
        &state.fillers,
    );
    let matches = librett_domain::knockout_from_slots(
        slots
            .iter()
            .map(|s| (s.entry_id, s.bye || s.entry_id.is_some()))
            .collect(),
        &state.results,
    );
    let order_revisions = (0..groups.len())
        .map(|g| state.order_revisions.get(&g).copied().unwrap_or(0))
        .collect();
    let result_versions = (0..groups.len())
        .map(|g| {
            let prefix = format!("group:{g}:");
            state
                .results
                .values()
                .filter(|s| s.key.starts_with(&prefix))
                .map(|s| u64::from(s.revision))
                .sum()
        })
        .collect();
    CompetitionState {
        match_version: state.results.values().map(|r| u64::from(r.revision)).sum(),
        filler_revision: state.filler_revision,
        rules_revision: state.revision,
        fillers: state.fillers.clone(),
        candidates: librett_domain::lucky_loser_candidates(draw, &groups),
        draw_id: Some(draw.id),
        stale: state.stale,
        groups,
        slots,
        matches,
        order_revisions,
        result_versions,
    }
}
fn matches_for(
    state: &Snapshot,
    group: usize,
    round: usize,
    knockout: bool,
) -> Vec<ScheduledMatch> {
    let Some(draw) = &state.draw else {
        return vec![];
    };
    if draw.format == CompetitionFormat::Knockout || knockout {
        competition(state)
            .matches
            .into_iter()
            .filter(|m| m.round == round)
            .collect()
    } else {
        group_round_matches(draw, group, round, &state.results)
    }
}
fn invalidated(state: &Snapshot) -> Vec<StoredMatchResult> {
    let projection: HashMap<_, _> = competition(state)
        .matches
        .into_iter()
        .map(|m| (m.key.clone(), m))
        .collect();
    state
        .results
        .values()
        .filter(|s| {
            s.key.starts_with("ko:")
                && s.result.is_some()
                && projection.get(&s.key).map_or(true, |m| m.result.is_none())
        })
        .cloned()
        .collect()
}
fn clear_invalidated(
    conn: &Connection,
    state: &mut Snapshot,
    draw: Uuid,
    affected: Vec<StoredMatchResult>,
) -> Result<(), ApplicationError> {
    for stored in affected {
        let revision = stored
            .revision
            .checked_add(1)
            .ok_or(ApplicationError::MatchConflict)?;
        write_result(conn, draw, &stored.key, revision, &None)?;
        state.results.insert(
            stored.key.clone(),
            StoredMatchResult {
                key: stored.key,
                revision,
                result: None,
            },
        );
    }
    Ok(())
}
fn key_parts(key: &str) -> Result<(usize, usize, usize), ApplicationError> {
    let parts: Vec<_> = key.split(':').collect();
    let num = |i: usize| {
        parts
            .get(i)
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or(ApplicationError::InvalidResult)
    };
    match parts.first().copied() {
        Some("ko") if parts.len() == 3 => Ok((0, num(1)?, num(2)?)),
        Some("group") if parts.len() == 4 => Ok((num(1)?, num(2)?, num(3)?)),
        _ => Err(ApplicationError::InvalidResult),
    }
}
fn write_result(
    conn: &Connection,
    draw: Uuid,
    key: &str,
    revision: u32,
    result: &Option<librett_domain::MatchResult>,
) -> Result<(), ApplicationError> {
    let json = serde_json::to_string(result).map_err(|_| ApplicationError::Storage)?;
    conn.execute(
        "INSERT INTO match_results(draw_id,match_key,revision,payload) VALUES(?1,?2,?3,?4)",
        params![draw.to_string(), key, revision, json],
    )
    .map_err(|_| ApplicationError::Storage)?;
    Ok(())
}
impl SqliteTournamentRepository {
    pub fn competition_state(
        &mut self,
        tournament: Uuid,
        category: Uuid,
    ) -> Result<CompetitionState, ApplicationError> {
        let tx = self
            .connection
            .transaction()
            .map_err(|_| ApplicationError::Storage)?;
        Ok(competition(&snapshot(&tx, tournament, category, None)?))
    }
    pub fn save_knockout_fillers(
        &mut self,
        request: librett_application::SaveFillersRequest,
    ) -> Result<CompetitionState, ApplicationError> {
        let json = serde_json::to_string(&("knockout_fillers", &request))
            .map_err(|_| ApplicationError::Storage)?;
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        let receipt: Option<(String, String)> = tx
            .query_row(
                "SELECT request_payload,result_payload FROM match_writes WHERE id=?1",
                [request.request_id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?;
        if let Some((old, output)) = receipt {
            return if old == json {
                serde_json::from_str(&output).map_err(|_| ApplicationError::Storage)
            } else {
                Err(ApplicationError::MatchConflict)
            };
        }
        let mut state = snapshot(&tx, request.tournament_id, request.category_id, None)?;
        let current = competition(&state);
        if state.stale
            || current.draw_id != Some(request.draw_id)
            || current.match_version != request.expected_match_version
            || current.filler_revision != request.expected_revision
            || state.revision != request.rules_revision
            || current.order_revisions != request.order_revisions
            || current.result_versions != request.result_versions
        {
            return Err(ApplicationError::MatchConflict);
        }
        if state.rules.knockout_filling != librett_domain::KnockoutFilling::LuckyLoser
            || current.groups.is_empty()
            || current.groups.iter().any(|g| !g.complete || !g.resolved)
        {
            return Err(ApplicationError::InvalidResult);
        }
        let draw = state.draw.as_ref().ok_or(ApplicationError::InvalidResult)?;
        let base = librett_domain::qualification_slots(draw, &current.groups);
        let eligible: HashSet<_> = current
            .candidates
            .iter()
            .map(|c| c.standing.entry_id)
            .collect();
        let mut used = HashSet::new();
        for (index, choice) in &request.fillers {
            if !base.get(*index).is_some_and(|s| s.bye) {
                return Err(ApplicationError::InvalidResult);
            }
            if let librett_domain::FillerChoice::Entry(id) = choice {
                if !eligible.contains(id) || !used.insert(*id) {
                    return Err(ApplicationError::InvalidResult);
                }
            }
        }
        state.fillers = request.fillers.clone();
        let affected = invalidated(&state);
        if !affected.is_empty() && !request.invalidate_downstream {
            return Err(ApplicationError::ResultImpact);
        }
        clear_invalidated(&tx, &mut state, request.draw_id, affected)?;
        state.filler_revision = request
            .expected_revision
            .checked_add(1)
            .ok_or(ApplicationError::MatchConflict)?;
        let payload =
            serde_json::to_string(&state.fillers).map_err(|_| ApplicationError::Storage)?;
        tx.execute("INSERT INTO knockout_fillers(draw_id,revision,payload) VALUES(?1,?2,?3) ON CONFLICT(draw_id) DO UPDATE SET revision=excluded.revision,payload=excluded.payload", params![request.draw_id.to_string(),state.filler_revision,payload]).map_err(|_| ApplicationError::Storage)?;
        let output = competition(&state);
        tx.execute(
            "INSERT INTO match_writes(id,request_payload,result_payload) VALUES(?1,?2,?3)",
            params![
                request.request_id.to_string(),
                json,
                serde_json::to_string(&output).map_err(|_| ApplicationError::Storage)?
            ],
        )
        .map_err(|_| ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)?;
        Ok(output)
    }
    pub fn save_group_order(
        &mut self,
        request: GroupOrderRequest,
    ) -> Result<CompetitionState, ApplicationError> {
        let json = serde_json::to_string(&("group_order", &request))
            .map_err(|_| ApplicationError::Storage)?;
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        let receipt: Option<(String, String)> = tx
            .query_row(
                "SELECT request_payload,result_payload FROM match_writes WHERE id=?1",
                [request.request_id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?;
        if let Some((old, result)) = receipt {
            return if old == json {
                serde_json::from_str(&result).map_err(|_| ApplicationError::Storage)
            } else {
                Err(ApplicationError::MatchConflict)
            };
        }
        let mut state = snapshot(&tx, request.tournament_id, request.category_id, None)?;
        let current = competition(&state);
        let group = current
            .groups
            .get(request.group)
            .ok_or(ApplicationError::InvalidResult)?;
        if state.stale
            || current.draw_id != Some(request.draw_id)
            || !group.complete
            || current.order_revisions[request.group] != request.expected_revision
            || current.result_versions[request.group] != request.expected_result_version
        {
            return Err(ApplicationError::MatchConflict);
        }
        if let Some(order) = &request.order {
            if order.len() != group.rows.len()
                || order.iter().copied().collect::<HashSet<_>>()
                    != group.rows.iter().map(|r| r.entry_id).collect()
            {
                return Err(ApplicationError::InvalidResult);
            }
            state.orders.insert(request.group, order.clone());
        } else {
            state.orders.remove(&request.group);
        }
        let affected = invalidated(&state);
        if !affected.is_empty() && !request.invalidate_downstream {
            return Err(ApplicationError::ResultImpact);
        }
        clear_invalidated(&tx, &mut state, request.draw_id, affected)?;
        let revision = request
            .expected_revision
            .checked_add(1)
            .ok_or(ApplicationError::MatchConflict)?;
        let payload =
            serde_json::to_string(&request.order).map_err(|_| ApplicationError::Storage)?;
        tx.execute("INSERT INTO group_orders(draw_id,group_number,revision,payload) VALUES(?1,?2,?3,?4) ON CONFLICT(draw_id,group_number) DO UPDATE SET revision=excluded.revision,payload=excluded.payload",params![request.draw_id.to_string(),request.group,revision,payload]).map_err(|_|ApplicationError::Storage)?;
        state.order_revisions.insert(request.group, revision);
        let result = competition(&state);
        let output = serde_json::to_string(&result).map_err(|_| ApplicationError::Storage)?;
        tx.execute(
            "INSERT INTO match_writes(id,request_payload,result_payload) VALUES(?1,?2,?3)",
            params![request.request_id.to_string(), json, output],
        )
        .map_err(|_| ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)?;
        Ok(result)
    }

    pub fn match_page(
        &mut self,
        tournament: Uuid,
        category: Uuid,
        group: usize,
        round: usize,
        page: usize,
        knockout: bool,
    ) -> Result<MatchPage, ApplicationError> {
        if group > 2048 || round > 4096 || page > 4096 {
            return Err(ApplicationError::InvalidResult);
        }
        let tx = self
            .connection
            .transaction()
            .map_err(|_| ApplicationError::Storage)?;
        let state = snapshot(&tx, tournament, category, Some((group, round)))?;
        let matches = matches_for(&state, group, round, knockout);
        let round_count = state.draw.as_ref().map_or(0, |draw| {
            if draw.format == CompetitionFormat::Knockout || knockout {
                let count = if draw.format == CompetitionFormat::Knockout {
                    draw.sections[0].len()
                } else {
                    draw.settings.group_count * draw.settings.qualifiers_per_group
                };
                count.next_power_of_two().ilog2() as usize
            } else {
                draw.sections.get(group).map_or(0, |s| {
                    let n = s.iter().flatten().count();
                    if n < 2 {
                        0
                    } else {
                        n + n % 2 - 1
                    }
                })
            }
        });
        let total = matches.len();
        Ok(MatchPage {
            draw: state.draw,
            rules: state.rules,
            rules_revision: state.revision,
            stale: state.stale,
            round_count,
            total,
            matches: matches.into_iter().skip(page * 32).take(32).collect(),
        })
    }
    pub fn save_match_result(
        &mut self,
        request: SaveMatchRequest,
    ) -> Result<ScheduledMatch, ApplicationError> {
        request.result.validate()?;
        let json = serde_json::to_string(&request).map_err(|_| ApplicationError::Storage)?;
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        let receipt: Option<(String, String)> = tx
            .query_row(
                "SELECT request_payload,result_payload FROM match_writes WHERE id=?1",
                [request.request_id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?;
        if let Some((prior, result)) = receipt {
            return if prior == json {
                serde_json::from_str(&result).map_err(|_| ApplicationError::Storage)
            } else {
                Err(ApplicationError::MatchConflict)
            };
        }
        let (group, round, _position) = key_parts(&request.key)?;
        let mut state = snapshot(
            &tx,
            request.tournament_id,
            request.category_id,
            Some((group, round)),
        )?;
        let draw = state.draw.as_ref().ok_or(ApplicationError::InvalidDraw)?;
        if state.stale
            || draw.id != request.draw_id
            || state.revision != request.rules_revision
            || state.rules != request.result.rules
        {
            return Err(ApplicationError::MatchConflict);
        }
        if draw.format == CompetitionFormat::Knockout && !request.key.starts_with("ko:") {
            return Err(ApplicationError::InvalidResult);
        }
        let item = matches_for(&state, group, round, request.key.starts_with("ko:"))
            .into_iter()
            .find(|m| m.key == request.key)
            .ok_or(ApplicationError::InvalidResult)?;
        if item.revision != request.expected_revision {
            return Err(ApplicationError::MatchConflict);
        }
        if item.bye
            || item.first != Some(request.result.first)
            || item.second != Some(request.result.second)
        {
            return Err(ApplicationError::InvalidResult);
        }
        let draw_id = draw.id;
        let revision = item
            .revision
            .checked_add(1)
            .ok_or(ApplicationError::MatchConflict)?;
        state.results.insert(
            request.key.clone(),
            StoredMatchResult {
                key: request.key.clone(),
                revision,
                result: Some(request.result.clone()),
            },
        );
        if request.key.starts_with("group:") {
            state.orders.remove(&group);
        }
        let downstream = invalidated(&state);
        if !downstream.is_empty() && !request.invalidate_downstream {
            return Err(ApplicationError::ResultImpact);
        }
        clear_invalidated(&tx, &mut state, draw_id, downstream)?;
        if request.key.starts_with("group:") {
            tx.execute("UPDATE group_orders SET revision=revision+1,payload='null' WHERE draw_id=?1 AND group_number=?2",params![draw_id.to_string(),group]).map_err(|_|ApplicationError::Storage)?;
        }
        write_result(
            &tx,
            draw_id,
            &request.key,
            revision,
            &Some(request.result.clone()),
        )?;
        let mut updated = item;
        updated.result = Some(request.result);
        updated.revision = revision;
        let payload = serde_json::to_string(&updated).map_err(|_| ApplicationError::Storage)?;
        tx.execute(
            "INSERT INTO match_writes(id,request_payload,result_payload) VALUES(?1,?2,?3)",
            params![request.request_id.to_string(), json, payload],
        )
        .map_err(|_| ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)?;
        Ok(updated)
    }
}

/// Rules and downstream result revisions are changed in the same transaction.
pub(super) fn guard_rules_change(
    conn: &Connection,
    tournament: Uuid,
    category: Uuid,
    rules: &CategoryRules,
    confirm: bool,
) -> Result<(), ApplicationError> {
    let mut state = snapshot(conn, tournament, category, None)?;
    if state.rules == *rules {
        return Ok(());
    }
    state.rules = rules.clone();
    let affected = invalidated(&state);
    if !affected.is_empty() && !confirm {
        return Err(ApplicationError::ResultImpact);
    }
    if let Some(draw) = state.draw.as_ref().map(|d| d.id) {
        clear_invalidated(conn, &mut state, draw, affected)?;
    }
    Ok(())
}
