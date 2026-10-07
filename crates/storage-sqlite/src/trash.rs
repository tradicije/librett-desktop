use super::*;

pub(super) fn ensure_active(conn: &Connection, id: Uuid) -> Result<(), ApplicationError> {
    let active: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM tournaments WHERE id=?1 AND id NOT IN (SELECT tournament_id FROM tournament_trash))",
        [id.to_string()], |r| r.get(0),
    ).map_err(|_| ApplicationError::Storage)?;
    if active {
        Ok(())
    } else {
        Err(ApplicationError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beta_schema_18_migrates_once_and_its_backup_can_be_imported() {
        let directory =
            std::env::temp_dir().join(format!("librett-trash-migration-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("live.sqlite");
        let mut repo = SqliteTournamentRepository::open(&path).unwrap();
        let tournament = librett_application::create_tournament(&mut repo, "Beta kup").unwrap();
        // Remove migrations 19 and 20 to reproduce the schema shipped in beta.1.
        let triggers: Vec<String> = repo.connection.prepare("SELECT name FROM sqlite_schema WHERE type='trigger' AND (sql LIKE '%tournament_in_trash%' OR name GLOB 'history_*')").unwrap().query_map([], |r| r.get(0)).unwrap().map(Result::unwrap).collect();
        for trigger in triggers {
            repo.connection
                .execute_batch(&format!("DROP TRIGGER {trigger};"))
                .unwrap();
        }
        repo.connection
            .execute_batch(
                "DROP TABLE action_history; DROP TABLE tournament_trash; PRAGMA user_version=18;",
            )
            .unwrap();
        // Older installs shipped this table without cascading deletion, while
        // fresh installs used a changed migration 11 under the same app version.
        let legacy = include_str!("../migrations/011_category_rules.sql")
            .split(';')
            .next()
            .unwrap()
            .replace(" ON DELETE CASCADE", "");
        let retained: Vec<String> = repo.connection.prepare("SELECT sql FROM sqlite_schema WHERE type='trigger' AND tbl_name='category_configurations'").unwrap().query_map([], |r| r.get(0)).unwrap().map(Result::unwrap).collect();
        repo.connection.execute_batch("CREATE TEMP TABLE saved_rules AS SELECT * FROM category_configurations; DROP TABLE category_configurations;").unwrap();
        repo.connection.execute_batch(&legacy).unwrap();
        repo.connection.execute_batch("INSERT INTO category_configurations SELECT * FROM saved_rules; DROP TABLE temp.saved_rules;").unwrap();
        for trigger in retained {
            repo.connection.execute_batch(&trigger).unwrap();
        }
        drop(repo);
        for _ in 0..2 {
            let repo = SqliteTournamentRepository::open(&path).unwrap();
            assert_eq!(repo.find(tournament.id).unwrap(), tournament);
            assert!(repo.list_trashed_tournaments().unwrap().is_empty());
        }
        let backups = std::fs::read_dir(&directory)
            .unwrap()
            .map(|p| p.unwrap().path())
            .filter(|p| p.to_string_lossy().contains("pre-v21-"))
            .collect::<Vec<_>>();
        assert_eq!(backups.len(), 1);
        let old_backup = std::fs::read(&backups[0]).unwrap();
        let mut repo = SqliteTournamentRepository::open(&path).unwrap();
        repo.trash_tournament(tournament.id).unwrap();
        repo.import_backup(&directory, &old_backup).unwrap();
        assert_eq!(repo.find(tournament.id).unwrap(), tournament);
        repo.trash_tournament(tournament.id).unwrap();
        assert_eq!(repo.restore_tournament(tournament.id).unwrap(), tournament);
        drop(repo);
        std::fs::remove_dir_all(directory).unwrap();
    }
}

impl SqliteTournamentRepository {
    pub fn trash_tournament(&mut self, id: Uuid) -> Result<(), ApplicationError> {
        let changed = self.connection.execute(
            "INSERT INTO tournament_trash(tournament_id) SELECT id FROM tournaments WHERE id=?1 ON CONFLICT(tournament_id) DO NOTHING",
            [id.to_string()],
        ).map_err(|_| ApplicationError::Storage)?;
        if changed == 0 {
            let exists: bool = self
                .connection
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM tournament_trash WHERE tournament_id=?1)",
                    [id.to_string()],
                    |r| r.get(0),
                )
                .map_err(|_| ApplicationError::Storage)?;
            if !exists {
                return Err(ApplicationError::NotFound);
            }
        }
        Ok(())
    }

    pub fn restore_tournament(&mut self, id: Uuid) -> Result<Tournament, ApplicationError> {
        self.connection
            .execute(
                "DELETE FROM tournament_trash WHERE tournament_id=?1",
                [id.to_string()],
            )
            .map_err(|_| ApplicationError::Storage)?;
        self.find(id)
    }

    pub fn list_trashed_tournaments(&self) -> Result<Vec<Tournament>, ApplicationError> {
        let mut query = self.connection.prepare("SELECT t.id,t.name,t.cover FROM tournaments t JOIN tournament_trash x ON x.tournament_id=t.id ORDER BY x.deleted_at DESC,x.rowid DESC").map_err(|_| ApplicationError::Storage)?;
        let rows = query
            .query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, Option<String>>(2)?,
                ))
            })
            .map_err(|_| ApplicationError::Storage)?;
        rows.map(|row| {
            let (id, name, cover) = row.map_err(|_| ApplicationError::Storage)?;
            let id = Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?;
            Ok(Tournament {
                id,
                name,
                cover,
                completed: completion::state(&self.connection, "tournaments", id)?
                    .completed_at
                    .is_some(),
                registered_count: self.registered_count(id)?,
                categories: self.load_categories(id)?,
            })
        })
        .collect()
    }
}
