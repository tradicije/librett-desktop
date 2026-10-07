use super::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct HistoryItem {
    pub id: i64,
    pub occurred_at: String,
    pub kind: String,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub entity_name: String,
    pub tournament_id: Option<String>,
    pub tournament_name: Option<String>,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub before_data: Option<serde_json::Value>,
    pub after_data: Option<serde_json::Value>,
    pub historical: bool,
}
#[derive(Serialize)]
pub struct HistoryOption {
    pub id: String,
    pub name: String,
    pub tournament_id: Option<String>,
}
#[derive(Serialize)]
pub struct HistoryPage {
    pub items: Vec<HistoryItem>,
    pub has_more: bool,
    pub tournaments: Vec<HistoryOption>,
    pub categories: Vec<HistoryOption>,
}

impl SqliteTournamentRepository {
    pub fn action_history(
        &mut self,
        kinds: Vec<String>,
        tournament_id: Option<Uuid>,
        category_id: Option<Uuid>,
        action: Option<String>,
        before_at: Option<String>,
        before_id: Option<i64>,
    ) -> Result<HistoryPage, ApplicationError> {
        if kinds.len() > 20
            || action.as_ref().is_some_and(|s| s.len() > 40)
            || before_at.as_ref().is_some_and(|s| s.len() > 40)
            || before_at.is_some() != before_id.is_some()
        {
            return Err(ApplicationError::Storage);
        }
        let tx = self
            .connection
            .transaction()
            .map_err(|_| ApplicationError::Storage)?;
        let mut query = tx.prepare("SELECT id,occurred_at,kind,action,entity_type,entity_id,entity_name,tournament_id,tournament_name,category_id,category_name,before_data,after_data,historical FROM action_history WHERE (?1='[]' OR kind IN (SELECT value FROM json_each(?1))) AND (?2 IS NULL OR tournament_id=?2) AND (?3 IS NULL OR category_id=?3) AND (?4 IS NULL OR action=?4) AND (?5 IS NULL OR occurred_at<?5 OR (occurred_at=?5 AND id<?6)) ORDER BY occurred_at DESC,id DESC LIMIT 51").map_err(|_|ApplicationError::Storage)?;
        let rows = query
            .query_map(
                params![
                    serde_json::to_string(&kinds).map_err(|_| ApplicationError::Storage)?,
                    tournament_id.map(|id| id.to_string()),
                    category_id.map(|id| id.to_string()),
                    action,
                    before_at,
                    before_id
                ],
                |r| {
                    let parse = |index| -> rusqlite::Result<Option<serde_json::Value>> {
                        let value: Option<String> = r.get(index)?;
                        value
                            .map(|s| {
                                serde_json::from_str(&s).map_err(|error| {
                                    rusqlite::Error::FromSqlConversionFailure(
                                        index,
                                        rusqlite::types::Type::Text,
                                        Box::new(error),
                                    )
                                })
                            })
                            .transpose()
                    };
                    Ok(HistoryItem {
                        id: r.get(0)?,
                        occurred_at: r.get(1)?,
                        kind: r.get(2)?,
                        action: r.get(3)?,
                        entity_type: r.get(4)?,
                        entity_id: r.get(5)?,
                        entity_name: r.get(6)?,
                        tournament_id: r.get(7)?,
                        tournament_name: r.get(8)?,
                        category_id: r.get(9)?,
                        category_name: r.get(10)?,
                        before_data: parse(11)?,
                        after_data: parse(12)?,
                        historical: r.get(13)?,
                    })
                },
            )
            .map_err(|_| ApplicationError::Storage)?;
        let mut items = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ApplicationError::Storage)?;
        drop(query);
        let has_more = items.len() > 50;
        items.truncate(50);
        let options = |sql| -> Result<Vec<HistoryOption>, ApplicationError> {
            let mut query = tx.prepare(sql).map_err(|_| ApplicationError::Storage)?;
            let rows = query
                .query_map([], |r| {
                    Ok(HistoryOption {
                        id: r.get(0)?,
                        name: r.get(1)?,
                        tournament_id: r.get(2)?,
                    })
                })
                .map_err(|_| ApplicationError::Storage)?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|_| ApplicationError::Storage)
        };
        let tournaments = options("SELECT tournament_id,tournament_name,NULL FROM action_history WHERE id IN (SELECT MAX(id) FROM action_history WHERE tournament_id IS NOT NULL GROUP BY tournament_id) ORDER BY tournament_name")?;
        let categories = options("SELECT category_id,category_name,tournament_id FROM action_history WHERE id IN (SELECT MAX(id) FROM action_history WHERE category_id IS NOT NULL GROUP BY category_id) ORDER BY category_name")?;
        tx.commit().map_err(|_| ApplicationError::Storage)?;
        Ok(HistoryPage {
            items,
            has_more,
            tournaments,
            categories,
        })
    }

    pub(super) fn record_file_action(
        &self,
        action: &str,
        name: &str,
    ) -> Result<(), ApplicationError> {
        self.connection.execute("INSERT INTO action_history(kind,action,entity_type,entity_id,entity_name) VALUES('backups',?1,'backup',?2,?2)",params![action,name]).map_err(|_|ApplicationError::Storage)?;
        Ok(())
    }

    pub fn record_export(&self, name: &str) -> Result<(), ApplicationError> {
        self.connection.execute("INSERT INTO action_history(kind,action,entity_type,entity_id,entity_name) VALUES('exports','exported','export',?1,?1)", [name]).map_err(|_|ApplicationError::Storage)?;
        Ok(())
    }
}
