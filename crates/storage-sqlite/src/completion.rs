use super::*;
use librett_application::{
    CategoryResults, CompletionRequest, CompletionState, TournamentProgress,
};
use std::collections::HashSet;

pub(super) fn state(
    conn: &Connection,
    table: &str,
    id: Uuid,
) -> Result<CompletionState, ApplicationError> {
    let query = match table {
        "categories" => "SELECT completion_revision,completed_at FROM categories WHERE id=?1",
        "tournaments" => "SELECT completion_revision,completed_at FROM tournaments WHERE id=?1",
        _ => return Err(ApplicationError::Storage),
    };
    conn.query_row(query, [id.to_string()], |r| {
        Ok(CompletionState {
            revision: r.get(0)?,
            completed_at: r.get(1)?,
        })
    })
    .optional()
    .map_err(|_| ApplicationError::Storage)?
    .ok_or(ApplicationError::NotFound)
}
pub(super) fn ensure_category_open(conn: &Connection, id: Uuid) -> Result<(), ApplicationError> {
    let closed: Option<bool> = conn.query_row("SELECT c.completed_at IS NOT NULL OR t.completed_at IS NOT NULL FROM categories c JOIN tournaments t ON t.id=c.tournament_id WHERE c.id=?1", [id.to_string()], |r|r.get(0)).optional().map_err(|_|ApplicationError::Storage)?;
    match closed {
        Some(false) => Ok(()),
        Some(true) => Err(ApplicationError::CompetitionClosed),
        None => Err(ApplicationError::NotFound),
    }
}
pub(super) fn ensure_tournament_open(conn: &Connection, id: Uuid) -> Result<(), ApplicationError> {
    if state(conn, "tournaments", id)?.completed_at.is_some() {
        Err(ApplicationError::CompetitionClosed)
    } else {
        Ok(())
    }
}
fn category_results(
    conn: &Connection,
    tournament: Uuid,
    category: Uuid,
) -> Result<CategoryResults, ApplicationError> {
    let row: Option<(String,Option<String>)> = conn.query_row("SELECT name,completion_snapshot FROM categories WHERE id=?1 AND tournament_id=?2 AND archived=0",params![category.to_string(),tournament.to_string()],|r|Ok((r.get(0)?,r.get(1)?))).optional().map_err(|_|ApplicationError::Storage)?;
    let (name, saved) = row.ok_or(ApplicationError::NotFound)?;
    let completion = state(conn, "categories", category)?;
    let tournament_completion = state(conn, "tournaments", tournament)?;
    if completion.completed_at.is_some() {
        let mut frozen: CategoryResults =
            serde_json::from_str(&saved.ok_or(ApplicationError::Storage)?)
                .map_err(|_| ApplicationError::Storage)?;
        frozen.completion = completion;
        frozen.tournament_completion = tournament_completion;
        frozen.version = serde_json::to_string(&(
            frozen.version.as_str(),
            frozen.completion.revision,
            frozen.tournament_completion.revision,
        ))
        .map_err(|_| ApplicationError::Storage)?;
        return Ok(frozen);
    }
    let source = super::matches::snapshot(conn, tournament, category, None)?;
    let projection = super::matches::competition(&source);
    let version = serde_json::to_string(&(
        projection.draw_id,
        projection.stale,
        projection.rules_revision,
        projection.filler_revision,
        projection.match_version,
        &projection.order_revisions,
        completion.revision,
        tournament_completion.revision,
    ))
    .map_err(|_| ApplicationError::Storage)?;
    let mut blockers = Vec::new();
    if source.draw.is_none() {
        blockers.push("no_draw".into());
    }
    if projection.stale {
        blockers.push("stale_draw".into());
    }
    if projection.groups.iter().any(|g| !g.complete) {
        blockers.push("group_matches".into());
    }
    if projection.groups.iter().any(|g| g.complete && !g.resolved) {
        blockers.push("group_ranking".into());
    }
    if projection
        .slots
        .iter()
        .any(|s| !s.bye && s.entry_id.is_none())
    {
        blockers.push("qualification".into());
    }
    let qualifying: HashSet<_> = if projection.slots.is_empty() {
        source
            .draw
            .as_ref()
            .map(|d| d.sections.iter().flatten().flatten().copied().collect())
            .unwrap_or_default()
    } else {
        projection.slots.iter().filter_map(|s| s.entry_id).collect()
    };
    let expected_entries = if projection.slots.is_empty() {
        source.draw.as_ref().map_or(0, |d| d.participants.len())
    } else {
        projection.slots.iter().filter(|s| !s.bye).count()
    };
    let knockout_total = expected_entries.saturating_sub(1)
        + usize::from(projection.matches.iter().any(|m| {
            m.position == 1
                && projection
                    .matches
                    .iter()
                    .filter(|m| m.position == 0)
                    .map(|m| m.round)
                    .max()
                    == Some(m.round)
        }));
    let knockout_completed = projection
        .matches
        .iter()
        .filter(|m| m.result.is_some())
        .count();
    let placements = source
        .draw
        .as_ref()
        .map(|draw| {
            librett_domain::final_placements_with_rule(
                draw,
                &projection.matches,
                &qualifying,
                source.rules.third_place,
            )
        })
        .unwrap_or_default();
    if source.draw.is_some() && (placements.is_empty() || knockout_completed != knockout_total) {
        blockers.push("knockout_matches".into());
    }
    let ready = blockers.is_empty() && !placements.is_empty();
    Ok(CategoryResults {
        third_place: source.rules.third_place,
        category_id: category,
        category_name: name,
        completion,
        tournament_completion,
        version,
        draw: source.draw,
        ready,
        blockers,
        group_completed: projection.groups.iter().map(|g| g.completed).sum(),
        group_total: projection.groups.iter().map(|g| g.total).sum(),
        knockout_completed,
        knockout_total,
        placements: if ready { placements } else { vec![] },
    })
}
fn tournament_progress(
    conn: &Connection,
    tournament: Uuid,
) -> Result<TournamentProgress, ApplicationError> {
    let completion = state(conn, "tournaments", tournament)?;
    let mut stmt = conn
        .prepare("SELECT id FROM categories WHERE tournament_id=?1 AND archived=0 ORDER BY rowid")
        .map_err(|_| ApplicationError::Storage)?;
    let ids = stmt
        .query_map([tournament.to_string()], |r| r.get::<_, String>(0))
        .map_err(|_| ApplicationError::Storage)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ApplicationError::Storage)?;
    drop(stmt);
    let categories = ids
        .into_iter()
        .map(|id| {
            category_results(
                conn,
                tournament,
                Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let ready = !categories.is_empty()
        && categories
            .iter()
            .all(|c| c.completion.completed_at.is_some());
    let version = serde_json::to_string(&(
        completion.revision,
        categories
            .iter()
            .map(|c| (c.category_id, c.version.as_str()))
            .collect::<Vec<_>>(),
    ))
    .map_err(|_| ApplicationError::Storage)?;
    Ok(TournamentProgress {
        tournament_id: tournament,
        completion,
        version,
        ready,
        categories,
    })
}
impl SqliteTournamentRepository {
    pub fn category_results(
        &mut self,
        tournament: Uuid,
        category: Uuid,
    ) -> Result<CategoryResults, ApplicationError> {
        let tx = self
            .connection
            .transaction()
            .map_err(|_| ApplicationError::Storage)?;
        category_results(&tx, tournament, category)
    }
    pub fn tournament_progress(
        &mut self,
        tournament: Uuid,
    ) -> Result<TournamentProgress, ApplicationError> {
        let tx = self
            .connection
            .transaction()
            .map_err(|_| ApplicationError::Storage)?;
        tournament_progress(&tx, tournament)
    }
    pub fn change_completion(
        &mut self,
        request: CompletionRequest,
    ) -> Result<TournamentProgress, ApplicationError> {
        let json = serde_json::to_string(&("completion", &request))
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
                Err(ApplicationError::CompletionConflict)
            };
        }
        let (table, kind, id, current, version, ready) = if let Some(category) = request.category_id
        {
            ensure_tournament_open(&tx, request.tournament_id)?;
            let results = category_results(&tx, request.tournament_id, category)?;
            (
                "categories",
                "category",
                category,
                results.completion,
                results.version,
                results.ready,
            )
        } else {
            let progress = tournament_progress(&tx, request.tournament_id)?;
            (
                "tournaments",
                "tournament",
                request.tournament_id,
                progress.completion,
                progress.version,
                progress.ready,
            )
        };
        if current.revision != request.expected_revision
            || version != request.expected_version
            || current.completed_at.is_some() == request.complete
        {
            return Err(ApplicationError::CompletionConflict);
        }
        if request.complete && !ready {
            return Err(ApplicationError::CompetitionIncomplete);
        }
        let revision = current
            .revision
            .checked_add(1)
            .ok_or(ApplicationError::CompletionConflict)?;
        let completed_at: Option<String> = if request.complete {
            Some(
                tx.query_row("SELECT strftime('%Y-%m-%dT%H:%M:%fZ','now')", [], |r| {
                    r.get(0)
                })
                .map_err(|_| ApplicationError::Storage)?,
            )
        } else {
            None
        };
        // Freeze the exact results being confirmed; reopening retains that snapshot in history.
        let payload = if let Some(category) = request.category_id {
            let mut results = category_results(&tx, request.tournament_id, category)?;
            results.completion = CompletionState {
                revision,
                completed_at: completed_at.clone(),
            };
            serde_json::to_string(&results).map_err(|_| ApplicationError::Storage)?
        } else {
            let mut progress = tournament_progress(&tx, request.tournament_id)?;
            progress.completion = CompletionState {
                revision,
                completed_at: completed_at.clone(),
            };
            serde_json::to_string(&progress).map_err(|_| ApplicationError::Storage)?
        };
        let query=match table{
            "categories"=>"UPDATE categories SET completion_revision=?2,completed_at=?3,completion_snapshot=?4 WHERE id=?1",
            _=>"UPDATE tournaments SET completion_revision=?2,completed_at=?3,completion_snapshot=?4 WHERE id=?1",
        };
        tx.execute(
            query,
            params![
                id.to_string(),
                revision,
                completed_at,
                if request.complete {
                    Some(payload.clone())
                } else {
                    None
                }
            ],
        )
        .map_err(|_| ApplicationError::Storage)?;
        tx.execute("INSERT INTO completion_history(entity_kind,entity_id,revision,payload) VALUES(?1,?2,?3,?4)",params![kind,id.to_string(),revision,payload]).map_err(|_|ApplicationError::Storage)?;
        let output = tournament_progress(&tx, request.tournament_id)?;
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
}
