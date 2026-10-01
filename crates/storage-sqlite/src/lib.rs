use librett_application::{ApplicationError, TournamentRepository};
use librett_domain::{Category, CompetitionFormat, Discipline, Tournament};
use rusqlite::{params, Connection, OptionalExtension};
use std::{path::Path, time::Duration};
use uuid::Uuid;

pub struct SqliteTournamentRepository {
    connection: Connection,
}

impl SqliteTournamentRepository {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
        Self::initialize(Connection::open(path)?)
    }

    fn initialize(mut connection: Connection) -> Result<Self, rusqlite::Error> {
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.execute_batch("PRAGMA foreign_keys = ON;")?;
        let version: i64 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
        if version > 1 {
            return Err(rusqlite::Error::InvalidQuery);
        }
        if version == 0 {
            let transaction = connection.transaction()?;
            transaction.execute_batch(include_str!("../migrations/001_tournaments.sql"))?;
            transaction.commit()?;
        }
        Ok(Self { connection })
    }

    fn load_categories(&self, tournament_id: Uuid) -> Result<Vec<Category>, ApplicationError> {
        let mut statement = self.connection.prepare(
            "SELECT id, name, discipline, format FROM categories WHERE tournament_id = ?1 ORDER BY rowid"
        ).map_err(|_| ApplicationError::Storage)?;
        let rows = statement.query_map([tournament_id.to_string()], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?))
        }).map_err(|_| ApplicationError::Storage)?;
        rows.map(|row| {
            let (id, name, discipline, format) = row.map_err(|_| ApplicationError::Storage)?;
            Ok(Category {
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
        }).collect()
    }
}

impl TournamentRepository for SqliteTournamentRepository {
    fn list(&self) -> Result<Vec<Tournament>, ApplicationError> {
        let mut statement = self.connection.prepare("SELECT id, name FROM tournaments ORDER BY rowid DESC")
            .map_err(|_| ApplicationError::Storage)?;
        let rows = statement.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))
            .map_err(|_| ApplicationError::Storage)?;
        rows.map(|row| {
            let (id, name) = row.map_err(|_| ApplicationError::Storage)?;
            let id = Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?;
            Ok(Tournament { id, name, categories: self.load_categories(id)? })
        }).collect()
    }

    fn find(&self, id: Uuid) -> Result<Tournament, ApplicationError> {
        let name = self.connection.query_row("SELECT name FROM tournaments WHERE id = ?1", [id.to_string()], |row| row.get::<_, String>(0))
            .optional().map_err(|_| ApplicationError::Storage)?.ok_or(ApplicationError::NotFound)?;
        Ok(Tournament { id, name, categories: self.load_categories(id)? })
    }

    fn insert(&mut self, tournament: &Tournament) -> Result<(), ApplicationError> {
        self.connection.execute("INSERT INTO tournaments (id, name) VALUES (?1, ?2)", params![tournament.id.to_string(), tournament.name])
            .map_err(|_| ApplicationError::Storage)?;
        Ok(())
    }

    fn insert_category(&mut self, tournament_id: Uuid, category: &Category) -> Result<(), ApplicationError> {
        let discipline = match category.discipline { Discipline::Singles => "singles", Discipline::Doubles => "doubles" };
        let format = match category.format { CompetitionFormat::Knockout => "knockout", CompetitionFormat::GroupsKnockout => "groups_knockout" };
        self.connection.execute(
            "INSERT INTO categories (id, tournament_id, name, name_key, discipline, format) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![category.id.to_string(), tournament_id.to_string(), category.name, category.name.to_lowercase(), discipline, format],
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
            add_category(&mut repository, tournament.id, "Singl", Discipline::Singles, CompetitionFormat::GroupsKnockout).unwrap();
            expected = add_category(&mut repository, tournament.id, "Dubl", Discipline::Doubles, CompetitionFormat::Knockout).unwrap();
            assert_eq!(add_category(&mut repository, tournament.id, " SINGL ", Discipline::Singles, CompetitionFormat::Knockout), Err(ApplicationError::DuplicateCategory));
            assert_eq!(repository.find(Uuid::new_v4()), Err(ApplicationError::NotFound));
        }
        {
            let repository = SqliteTournamentRepository::open(&path).unwrap();
            assert_eq!(repository.list().unwrap(), vec![expected]);
        }
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn category_cannot_reference_a_missing_tournament() {
        let mut repository = SqliteTournamentRepository::initialize(Connection::open_in_memory().unwrap()).unwrap();
        let mut tournament = Tournament::new("Kup").unwrap();
        tournament.add_category("Dubl", Discipline::Doubles, CompetitionFormat::Knockout).unwrap();
        assert_eq!(repository.insert_category(tournament.id, &tournament.categories[0]), Err(ApplicationError::Storage));
        assert!(repository.list().unwrap().is_empty());
    }
}
