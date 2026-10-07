use super::*;
use librett_application::DrawRepository;
use librett_domain::CategoryDraw;
use std::collections::HashSet;

impl DrawRepository for SqliteTournamentRepository {
    fn find_draw(&self, category_id: Uuid) -> Result<Option<CategoryDraw>, ApplicationError> {
        let payload: Option<String> = self.connection.query_row("SELECT payload FROM category_draws WHERE category_id=?1 ORDER BY revision DESC LIMIT 1", [category_id.to_string()], |r| r.get(0)).optional().map_err(|_| ApplicationError::Storage)?;
        payload
            .map(|json| serde_json::from_str(&json).map_err(|_| ApplicationError::Storage))
            .transpose()
    }

    fn save_draw(
        &mut self,
        mut draw: CategoryDraw,
        expected_revision: u32,
        confirm_restart: bool,
    ) -> Result<CategoryDraw, ApplicationError> {
        let request = serde_json::to_string(&(expected_revision, &draw, confirm_restart))
            .map_err(|_| ApplicationError::Storage)?;
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        let old: Option<(String, String)> = tx
            .query_row(
                "SELECT request_payload,payload FROM category_draws WHERE id=?1",
                [draw.id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?;
        if let Some((previous_request, payload)) = old {
            return if previous_request == request {
                serde_json::from_str(&payload).map_err(|_| ApplicationError::Storage)
            } else {
                Err(ApplicationError::DrawConflict)
            };
        }
        super::completion::ensure_category_open(&tx, draw.category_id)?;
        let revision: u32 = tx
            .query_row(
                "SELECT COALESCE(MAX(revision),0) FROM category_draws WHERE category_id=?1",
                [draw.category_id.to_string()],
                |r| r.get(0),
            )
            .map_err(|_| ApplicationError::Storage)?;
        if revision != expected_revision {
            return Err(ApplicationError::DrawConflict);
        }
        if super::registration::category_started(&tx, draw.category_id)? && !confirm_restart {
            return Err(ApplicationError::ResultImpact);
        }
        if draw.format == librett_domain::CompetitionFormat::GroupsKnockout {
            let payload: Option<String> = tx
                .query_row(
                    "SELECT payload FROM category_configurations WHERE category_id=?1",
                    [draw.category_id.to_string()],
                    |r| r.get(0),
                )
                .optional()
                .map_err(|_| ApplicationError::Storage)?;
            let configured = payload
                .map(|json| {
                    serde_json::from_str::<librett_domain::CategoryRules>(&json)
                        .map_err(|_| ApplicationError::Storage)
                })
                .transpose()?
                .unwrap_or_default();
            if configured.group_count != draw.settings.group_count
                || configured.qualifiers_per_group != draw.settings.qualifiers_per_group
            {
                return Err(ApplicationError::InvalidRules);
            }
        }
        // Recheck registration membership while holding the write transaction.
        let mut statement = tx.prepare("SELECT e.id FROM entries e JOIN categories c ON c.id=e.category_id WHERE e.category_id=?1 AND e.status='registered' AND c.archived=0").map_err(|_| ApplicationError::Storage)?;
        let active = statement
            .query_map([draw.category_id.to_string()], |r| r.get::<_, String>(0))
            .map_err(|_| ApplicationError::Storage)?
            .collect::<Result<HashSet<_>, _>>()
            .map_err(|_| ApplicationError::Storage)?;
        if active != draw.participants.iter().map(|e| e.id.to_string()).collect() {
            return Err(ApplicationError::InvalidDraw);
        }
        drop(statement);
        draw.revision = revision
            .checked_add(1)
            .ok_or(ApplicationError::DrawConflict)?;
        let payload = serde_json::to_string(&draw).map_err(|_| ApplicationError::Storage)?;
        tx.execute("INSERT INTO category_draws(id,category_id,revision,request_payload,payload) VALUES(?1,?2,?3,?4,?5)", params![draw.id.to_string(),draw.category_id.to_string(),draw.revision,request,payload]).map_err(|_| ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)?;
        Ok(draw)
    }
}
