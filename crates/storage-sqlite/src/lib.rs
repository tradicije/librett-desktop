use librett_application::{ApplicationError, TournamentRepository};
use librett_domain::{Category, CompetitionFormat, Discipline, Tournament};
use rusqlite::{params, Connection, OptionalExtension};
use std::{path::Path, time::Duration};
use uuid::Uuid;

mod backup;
mod cash;
mod category_rules;
#[cfg(test)]
mod category_tests;
mod completion;
mod draw;
mod history;
mod matches;
#[cfg(test)]
mod player_cash_tests;
mod players;
mod registration;
mod registry;
mod scheduling;
mod trash;
pub use history::{HistoryItem, HistoryOption, HistoryPage};
#[cfg(test)]
mod workflow_tests;

pub struct SqliteTournamentRepository {
    connection: Connection,
}

impl SqliteTournamentRepository {
    pub fn update_tournament_details(
        &mut self,
        id: Uuid,
        name: &str,
        cover: Option<String>,
        expected_name: &str,
        expected_cover: Option<String>,
    ) -> Result<Tournament, ApplicationError> {
        self.find(id)?;
        let normalized = Tournament::new(name)?.name;
        if let Some(image) = &cover {
            librett_domain::validate_jpeg(image, true)?;
        }
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        let current: Option<(String, Option<String>)> = tx
            .query_row(
                "SELECT name, cover FROM tournaments WHERE id = ?1",
                [id.to_string()],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?;
        let (old_name, old_cover) = current.ok_or(ApplicationError::NotFound)?;
        if old_name != normalized || old_cover != cover {
            if old_name != expected_name || old_cover != expected_cover {
                return Err(ApplicationError::DrawConflict);
            }
            tx.execute(
                "UPDATE tournaments SET name = ?1, cover = ?2 WHERE id = ?3",
                rusqlite::params![normalized, cover, id.to_string()],
            )
            .map_err(|_| ApplicationError::Storage)?;
        }
        tx.commit().map_err(|_| ApplicationError::Storage)?;
        self.find(id)
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
        let path = path.as_ref();
        let connection = Connection::open(path)?;
        let version: i64 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
        if (1..22).contains(&version) {
            // VACUUM INTO creates a consistent SQLite snapshot before changing an existing schema.
            let backup = path.with_extension(format!("pre-v22-{}.sqlite", Uuid::new_v4()));
            connection.execute("VACUUM INTO ?1", [backup.to_string_lossy().as_ref()])?;
        }
        Self::initialize(connection)
    }

    fn initialize(mut connection: Connection) -> Result<Self, rusqlite::Error> {
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.execute_batch("PRAGMA foreign_keys = ON;")?;
        let version: i64 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
        if version > 22 {
            return Err(rusqlite::Error::InvalidQuery);
        }
        if version == 0 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/001_tournaments.sql"))?;
            transaction.commit()?;
        }
        if version < 2 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/002_players.sql"))?;
            transaction.commit()?;
        }
        if version < 3 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/003_player_profiles.sql"))?;
            transaction.commit()?;
        }
        if version < 4 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/004_registration_status.sql"))?;
            transaction.commit()?;
        }
        if version < 5 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/005_cash.sql"))?;
            transaction.commit()?;
        }
        if version < 6 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/006_category_fees.sql"))?;
            transaction.commit()?;
        }
        if version < 7 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/007_category_archive.sql"))?;
            transaction.commit()?;
        }
        if version < 8 {
            // SQLite requires FK enforcement off outside a transaction when
            // rebuilding a referenced table. Verify all references before commit.
            connection.execute_batch("PRAGMA foreign_keys = OFF;")?;
            let result = (|| -> Result<(), rusqlite::Error> {
                let transaction = connection.transaction()?;
                transaction.execute_batch(include_str!("../migrations/008_category_names.sql"))?;
                let broken: i64 = transaction.query_row(
                    "SELECT count(*) FROM pragma_foreign_key_check",
                    [],
                    |row| row.get(0),
                )?;
                if broken != 0 {
                    return Err(rusqlite::Error::InvalidQuery);
                }
                transaction.commit()
            })();
            connection.execute_batch("PRAGMA foreign_keys = ON;")?;
            result?;
        }
        if version < 9 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/009_player_cash.sql"))?;
            transaction.commit()?;
        }
        if version < 10 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/010_category_draws.sql"))?;
            transaction.commit()?;
        }
        if version < 11 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/011_category_rules.sql"))?;
            transaction.commit()?;
        }
        if version < 12 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/012_guarded_writes.sql"))?;
            transaction.commit()?;
        }
        if version < 13 {
            // Some development checkouts added cover while schema 12 was live.
            // Accept both forms without retrying an already applied ALTER TABLE.
            let has_cover: bool = connection.query_row(
                "SELECT EXISTS(SELECT 1 FROM pragma_table_info('tournaments') WHERE name='cover')",
                [],
                |row| row.get(0),
            )?;
            let transaction = connection.transaction()?;
            if has_cover {
                transaction.execute_batch("PRAGMA user_version = 13;")?;
            } else {
                transaction
                    .execute_batch(include_str!("../migrations/013_tournament_covers.sql"))?;
            }
            transaction.commit()?;
        }
        if version < 14 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/014_match_results.sql"))?;
            transaction.commit()?;
        }
        if version < 15 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/015_group_orders.sql"))?;
            transaction.commit()?;
        }
        if version < 16 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/016_knockout_fillers.sql"))?;
            transaction.commit()?;
        }
        if version < 17 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/017_completion.sql"))?;
            transaction.commit()?;
        }
        if version < 18 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/018_scheduling.sql"))?;
            transaction.commit()?;
        }
        if version < 19 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/019_tournament_trash.sql"))?;
            transaction.commit()?;
        }
        if version < 20 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/020_action_history.sql"))?;
            transaction.commit()?;
        }
        if version < 21 {
            let transaction = connection.transaction()?;
            // Two definitions shipped under migration 11. Accept only those known
            // definitions; do not silently repair arbitrary imported schema changes.
            let definition: String = transaction.query_row(
                "SELECT sql FROM sqlite_schema WHERE type='table' AND name='category_configurations'",
                [], |row| row.get(0),
            )?;
            let canonical = include_str!("../migrations/011_category_rules.sql")
                .split(';')
                .next()
                .unwrap()
                .trim();
            let legacy = canonical.replace(" ON DELETE CASCADE", "");
            if definition.trim() != canonical && definition.trim() != legacy {
                return Err(rusqlite::Error::InvalidQuery);
            }
            // Preserve triggers verbatim so later backup validation still detects
            // unexpected triggers or altered audit/guard definitions.
            let triggers: Vec<String> = {
                let mut statement = transaction.prepare(
                    "SELECT sql FROM sqlite_schema WHERE type='trigger' AND tbl_name='category_configurations' ORDER BY name",
                )?;
                let rows = statement.query_map([], |row| row.get(0))?;
                rows.collect::<Result<_, _>>()?
            };
            transaction.execute_batch("CREATE TEMP TABLE category_rules_upgrade AS SELECT * FROM category_configurations; DROP TABLE category_configurations;")?;
            transaction.execute_batch(canonical)?;
            transaction.execute_batch("INSERT INTO category_configurations SELECT * FROM category_rules_upgrade; DROP TABLE temp.category_rules_upgrade;")?;
            for trigger in triggers {
                transaction.execute_batch(&trigger)?;
            }
            transaction.execute_batch("PRAGMA user_version = 21;")?;
            transaction.commit()?;
        }
        if version < 22 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/022_registry.sql"))?;
            transaction.commit()?;
        }
        Ok(Self { connection })
    }

    fn registered_count(&self, tournament_id: Uuid) -> Result<usize, ApplicationError> {
        self.connection.query_row("SELECT count(DISTINCT m.player_id) FROM entry_members m JOIN entries e ON e.id=m.entry_id JOIN categories c ON c.id=e.category_id WHERE c.tournament_id=?1 AND c.archived=0 AND e.status='registered'", [tournament_id.to_string()], |r| r.get(0)).map_err(|_| ApplicationError::Storage)
    }

    fn load_categories(&self, tournament_id: Uuid) -> Result<Vec<Category>, ApplicationError> {
        let mut statement = self.connection.prepare(
            "SELECT id, name, discipline, format, fee_minor, archived FROM categories WHERE tournament_id = ?1 ORDER BY rowid"
        ).map_err(|_| ApplicationError::Storage)?;
        let rows = statement
            .query_map([tournament_id.to_string()], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, bool>(5)?,
                ))
            })
            .map_err(|_| ApplicationError::Storage)?;
        rows.map(|row| {
            let (id, name, discipline, format, fee_minor, archived) =
                row.map_err(|_| ApplicationError::Storage)?;
            Ok(Category {
                completed: completion::state(
                    &self.connection,
                    "categories",
                    Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?,
                )?
                .completed_at
                .is_some(),
                archived,
                fee_minor,
                id: Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?,
                name,
                discipline: match discipline.as_str() {
                    "singles" => Discipline::Singles,
                    "doubles" => Discipline::Doubles,
                    _ => return Err(ApplicationError::Storage),
                },
                format: match format.as_str() {
                    "knockout" => CompetitionFormat::Knockout,
                    "groups_knockout" => CompetitionFormat::GroupsKnockout,
                    _ => return Err(ApplicationError::Storage),
                },
            })
        })
        .collect()
    }
}

impl TournamentRepository for SqliteTournamentRepository {
    fn list(&self) -> Result<Vec<Tournament>, ApplicationError> {
        let mut statement = self
            .connection
            .prepare("SELECT id, name, cover FROM tournaments WHERE id NOT IN (SELECT tournament_id FROM tournament_trash) ORDER BY rowid DESC")
            .map_err(|_| ApplicationError::Storage)?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                ))
            })
            .map_err(|_| ApplicationError::Storage)?;
        rows.map(|row| {
            let (id, name, cover) = row.map_err(|_| ApplicationError::Storage)?;
            let id = Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?;
            Ok(Tournament {
                completed: completion::state(&self.connection, "tournaments", id)?
                    .completed_at
                    .is_some(),
                registered_count: self.registered_count(id)?,
                cover,
                id,
                name,
                categories: self.load_categories(id)?,
            })
        })
        .collect()
    }

    fn find(&self, id: Uuid) -> Result<Tournament, ApplicationError> {
        let (name, cover) = self
            .connection
            .query_row(
                "SELECT name,cover FROM tournaments WHERE id = ?1 AND id NOT IN (SELECT tournament_id FROM tournament_trash)",
                [id.to_string()],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?)),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?
            .ok_or(ApplicationError::NotFound)?;
        Ok(Tournament {
            completed: completion::state(&self.connection, "tournaments", id)?
                .completed_at
                .is_some(),
            registered_count: self.registered_count(id)?,
            cover,
            id,
            name,
            categories: self.load_categories(id)?,
        })
    }

    fn insert(&mut self, tournament: &Tournament) -> Result<(), ApplicationError> {
        self.connection
            .execute(
                "INSERT INTO tournaments (id, name, cover) VALUES (?1, ?2, ?3)",
                params![tournament.id.to_string(), tournament.name, tournament.cover],
            )
            .map_err(|_| ApplicationError::Storage)?;
        Ok(())
    }

    fn insert_category(
        &mut self,
        tournament_id: Uuid,
        category: &Category,
    ) -> Result<(), ApplicationError> {
        completion::ensure_tournament_open(&self.connection, tournament_id)?;
        let discipline = match category.discipline {
            Discipline::Singles => "singles",
            Discipline::Doubles => "doubles",
        };
        let format = match category.format {
            CompetitionFormat::Knockout => "knockout",
            CompetitionFormat::GroupsKnockout => "groups_knockout",
        };
        self.connection.execute(
            "INSERT INTO categories (id, tournament_id, name, name_key, discipline, format, fee_minor) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![category.id.to_string(), tournament_id.to_string(), category.name, category.name.to_lowercase(), discipline, format, category.fee_minor],
        ).map_err(|_| ApplicationError::Storage)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use librett_application::{add_category, create_tournament};

    #[test]
    fn persists_categories_across_restarts_and_rejects_duplicate_without_damage() {
        let path = std::env::temp_dir().join(format!("librett-test-{}.sqlite", Uuid::new_v4()));
        let expected;
        {
            let mut repository = SqliteTournamentRepository::open(&path).unwrap();
            let tournament = create_tournament(&mut repository, "Bubušinac").unwrap();
            add_category(
                &mut repository,
                tournament.id,
                "Singl",
                Discipline::Singles,
                CompetitionFormat::GroupsKnockout,
            )
            .unwrap();
            expected = add_category(
                &mut repository,
                tournament.id,
                "Dubl",
                Discipline::Doubles,
                CompetitionFormat::Knockout,
            )
            .unwrap();
            assert_eq!(
                add_category(
                    &mut repository,
                    tournament.id,
                    " SINGL ",
                    Discipline::Singles,
                    CompetitionFormat::Knockout
                ),
                Err(ApplicationError::DuplicateCategory)
            );
            assert_eq!(
                repository.find(Uuid::new_v4()),
                Err(ApplicationError::NotFound)
            );
        }
        {
            let repository = SqliteTournamentRepository::open(&path).unwrap();
            assert_eq!(repository.list().unwrap(), vec![expected]);
        }
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn category_cannot_reference_a_missing_tournament() {
        let mut repository =
            SqliteTournamentRepository::initialize(Connection::open_in_memory().unwrap()).unwrap();
        let mut tournament = Tournament::new("Kup").unwrap();
        tournament
            .add_category("Dubl", Discipline::Doubles, CompetitionFormat::Knockout)
            .unwrap();
        assert_eq!(
            repository.insert_category(tournament.id, &tournament.categories[0]),
            Err(ApplicationError::NotFound)
        );
        assert!(repository.list().unwrap().is_empty());
    }
}

impl librett_application::CategoryRepository for SqliteTournamentRepository {
    fn delete_category(
        &mut self,
        tournament_id: Uuid,
        category_id: Uuid,
    ) -> Result<(), ApplicationError> {
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        completion::ensure_category_open(&tx, category_id)?;
        if registration::category_started(&tx, category_id)? {
            return Err(ApplicationError::CompetitionStarted);
        }
        let used: Option<bool> = tx.query_row("SELECT EXISTS(SELECT 1 FROM entries WHERE category_id=c.id) FROM categories c WHERE c.id=?1 AND c.tournament_id=?2 AND c.archived=0",params![category_id.to_string(),tournament_id.to_string()],|r|r.get(0)).optional().map_err(|_|ApplicationError::Storage)?;
        match used {
            Some(true) => {
                tx.execute(
                    "UPDATE categories SET archived=1 WHERE id=?1",
                    [category_id.to_string()],
                )
                .map_err(|_| ApplicationError::Storage)?;
            }
            Some(false) => {
                tx.execute(
                    "DELETE FROM categories WHERE id=?1",
                    [category_id.to_string()],
                )
                .map_err(|_| ApplicationError::Storage)?;
            }
            None => return Err(ApplicationError::NotFound),
        }
        tx.commit().map_err(|_| ApplicationError::Storage)
    }
}
