use super::*;
use librett_application::{CategoryConfiguration, CategoryRulesRepository};
use librett_domain::CategoryRules;

impl CategoryRulesRepository for SqliteTournamentRepository {
    fn find_category_rules(
        &self,
        category_id: Uuid,
    ) -> Result<CategoryConfiguration, ApplicationError> {
        let row: Option<(u32, String)> = self
            .connection
            .query_row(
                "SELECT revision,payload FROM category_configurations WHERE category_id=?1",
                [category_id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?;
        match row {
            Some((revision, json)) => Ok(CategoryConfiguration {
                category_id,
                revision,
                rules: serde_json::from_str(&json).map_err(|_| ApplicationError::Storage)?,
            }),
            None => Ok(CategoryConfiguration {
                category_id,
                revision: 0,
                rules: CategoryRules::default(),
            }),
        }
    }
    fn insert_configured_category(
        &mut self,
        tournament_id: Uuid,
        category: &Category,
        rules: &CategoryRules,
    ) -> Result<(), ApplicationError> {
        let payload = serde_json::to_string(rules).map_err(|_| ApplicationError::Storage)?;
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        let discipline = match category.discipline {
            Discipline::Singles => "singles",
            Discipline::Doubles => "doubles",
        };
        let format = match category.format {
            CompetitionFormat::Knockout => "knockout",
            CompetitionFormat::GroupsKnockout => "groups_knockout",
        };
        tx.execute("INSERT INTO categories(id,tournament_id,name,name_key,discipline,format,fee_minor) VALUES(?1,?2,?3,?4,?5,?6,?7)",params![category.id.to_string(),tournament_id.to_string(),category.name,category.name.to_lowercase(),discipline,format,category.fee_minor]).map_err(|_| ApplicationError::Storage)?;
        tx.execute(
            "INSERT INTO category_configurations(category_id,revision,payload) VALUES(?1,1,?2)",
            params![category.id.to_string(), payload],
        )
        .map_err(|_| ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)
    }
    fn save_category_rules(
        &mut self,
        tournament_id: Uuid,
        category_id: Uuid,
        rules: &CategoryRules,
        expected_revision: u32,
    ) -> Result<CategoryConfiguration, ApplicationError> {
        let payload = serde_json::to_string(rules).map_err(|_| ApplicationError::Storage)?;
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        let exists: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM categories WHERE id=?1 AND tournament_id=?2 AND archived=0)",params![category_id.to_string(),tournament_id.to_string()],|r|r.get(0)).map_err(|_| ApplicationError::Storage)?;
        if !exists {
            return Err(ApplicationError::NotFound);
        }
        let old: Option<(u32, String)> = tx
            .query_row(
                "SELECT revision,payload FROM category_configurations WHERE category_id=?1",
                [category_id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?;
        let revision = old.as_ref().map(|(revision, _)| *revision).unwrap_or(0);
        if let Some((_, json)) = &old {
            let previous: CategoryRules =
                serde_json::from_str(json).map_err(|_| ApplicationError::Storage)?;
            if previous == *rules {
                return Ok(CategoryConfiguration {
                    category_id,
                    revision,
                    rules: previous,
                });
            }
        }
        if revision != expected_revision {
            return Err(ApplicationError::DrawConflict);
        }
        let revision = revision
            .checked_add(1)
            .ok_or(ApplicationError::DrawConflict)?;
        tx.execute("INSERT INTO category_configurations(category_id,revision,payload) VALUES(?1,?2,?3) ON CONFLICT(category_id) DO UPDATE SET revision=excluded.revision,payload=excluded.payload",params![category_id.to_string(),revision,payload]).map_err(|_|ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)?;
        Ok(CategoryConfiguration {
            category_id,
            revision,
            rules: rules.clone(),
        })
    }
    fn save_configured_category(
        &mut self,
        tournament_id: Uuid,
        category: &Category,
        rules: &CategoryRules,
        expected_revision: u32,
    ) -> Result<CategoryConfiguration, ApplicationError> {
        let payload = serde_json::to_string(rules).map_err(|_| ApplicationError::Storage)?;
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        let row: Option<(String,String,String,i64,bool)> = tx.query_row("SELECT name,discipline,format,fee_minor,EXISTS(SELECT 1 FROM entries WHERE category_id=c.id) FROM categories c WHERE id=?1 AND tournament_id=?2 AND archived=0",params![category.id.to_string(),tournament_id.to_string()],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?))).optional().map_err(|_|ApplicationError::Storage)?;
        let (name, old_discipline, old_format, fee, used) =
            row.ok_or(ApplicationError::NotFound)?;
        let discipline = match category.discipline {
            Discipline::Singles => "singles",
            Discipline::Doubles => "doubles",
        };
        let format = match category.format {
            CompetitionFormat::Knockout => "knockout",
            CompetitionFormat::GroupsKnockout => "groups_knockout",
        };
        if used && (old_discipline != discipline || old_format != format) {
            return Err(ApplicationError::InvalidRules);
        }
        let old: Option<(u32, String)> = tx
            .query_row(
                "SELECT revision,payload FROM category_configurations WHERE category_id=?1",
                [category.id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?;
        let revision = old.as_ref().map(|(r, _)| *r).unwrap_or(0);
        let previous = old
            .map(|(_, json)| {
                serde_json::from_str::<CategoryRules>(&json).map_err(|_| ApplicationError::Storage)
            })
            .transpose()?
            .unwrap_or_default();
        if previous == *rules
            && name == category.name
            && fee == category.fee_minor
            && old_discipline == discipline
            && old_format == format
        {
            return Ok(CategoryConfiguration {
                category_id: category.id,
                revision,
                rules: rules.clone(),
            });
        }
        if revision != expected_revision {
            return Err(ApplicationError::DrawConflict);
        }
        let revision = revision
            .checked_add(1)
            .ok_or(ApplicationError::DrawConflict)?;
        tx.execute("UPDATE categories SET name=?2,name_key=?3,discipline=?4,format=?5,fee_minor=?6 WHERE id=?1",params![category.id.to_string(),category.name,category.name.to_lowercase(),discipline,format,category.fee_minor]).map_err(|_|ApplicationError::Storage)?;
        tx.execute("INSERT INTO category_configurations(category_id,revision,payload) VALUES(?1,?2,?3) ON CONFLICT(category_id) DO UPDATE SET revision=excluded.revision,payload=excluded.payload",params![category.id.to_string(),revision,payload]).map_err(|_|ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)?;
        Ok(CategoryConfiguration {
            category_id: category.id,
            revision,
            rules: rules.clone(),
        })
    }
}

impl librett_application::CategoryEditorRepository for SqliteTournamentRepository {
    fn category_editor_state(
        &self,
        tournament_id: Uuid,
        category_id: Uuid,
    ) -> Result<librett_application::CategoryEditorState, ApplicationError> {
        self.connection.query_row("SELECT c.name,c.discipline,c.format,c.fee_minor,COALESCE(r.revision,0),r.payload,EXISTS(SELECT 1 FROM entries WHERE category_id=c.id) FROM categories c LEFT JOIN category_configurations r ON r.category_id=c.id WHERE c.id=?1 AND c.tournament_id=?2 AND c.archived=0", params![category_id.to_string(), tournament_id.to_string()], |row| {
            let discipline: String = row.get(1)?; let format: String = row.get(2)?; let payload: Option<String> = row.get(5)?;
            let rules = payload.map(|json| serde_json::from_str(&json).map_err(|_| rusqlite::Error::InvalidQuery)).transpose()?.unwrap_or_default();
            Ok(librett_application::CategoryEditorState {
                category: Category { id: category_id, name: row.get(0)?, discipline: match discipline.as_str() { "singles" => Discipline::Singles, "doubles" => Discipline::Doubles, _ => return Err(rusqlite::Error::InvalidQuery) }, format: match format.as_str() { "knockout" => CompetitionFormat::Knockout, "groups_knockout" => CompetitionFormat::GroupsKnockout, _ => return Err(rusqlite::Error::InvalidQuery) }, fee_minor: row.get(3)?, archived: false },
                configuration: CategoryConfiguration { category_id, revision: row.get(4)?, rules }, used: row.get(6)?,
            })
        }).optional().map_err(|_| ApplicationError::Storage)?.ok_or(ApplicationError::NotFound)
    }
}
