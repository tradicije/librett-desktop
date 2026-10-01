use super::*;
use librett_application::PlayerRepository;
use librett_domain::{Entry, EntryMember, Player, PlayerProfile};

fn read_player(row: &rusqlite::Row<'_>) -> rusqlite::Result<Player> {
    let id: String = row.get(0)?;
    Ok(Player {
        id: Uuid::parse_str(&id).map_err(|error| {
            rusqlite::Error::FromSqlConversionFailure(
                0,
                rusqlite::types::Type::Text,
                Box::new(error),
            )
        })?,
        name: row.get(1)?,
        club: row.get(2)?,
        profile: PlayerProfile {
            birth_year: row.get(3)?,
            city: row.get(4)?,
            country: row.get(5)?,
            email: row.get(6)?,
            phone: row.get(7)?,
            notes: row.get(8)?,
            photo: row.get(9)?,
        },
    })
}

impl PlayerRepository for SqliteTournamentRepository {
    fn list_players(&self) -> Result<Vec<Player>, ApplicationError> {
        let mut statement = self.connection.prepare("SELECT id, name, club, birth_year, city, country, email, phone, notes, photo FROM players ORDER BY name COLLATE NOCASE, id").map_err(|_| ApplicationError::Storage)?;
        let rows = statement
            .query_map([], read_player)
            .map_err(|_| ApplicationError::Storage)?;
        rows.map(|row| row.map_err(|_| ApplicationError::Storage))
            .collect()
    }

    fn find_player(&self, id: Uuid) -> Result<Player, ApplicationError> {
        self.connection.query_row("SELECT id, name, club, birth_year, city, country, email, phone, notes, photo FROM players WHERE id = ?1", [id.to_string()], read_player)
            .optional().map_err(|_| ApplicationError::Storage)?.ok_or(ApplicationError::NotFound)
    }

    fn insert_player(&mut self, player: &Player) -> Result<(), ApplicationError> {
        self.connection.execute("INSERT INTO players (id, name, club, birth_year, city, country, email, phone, notes, photo) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![player.id.to_string(), player.name, player.club, player.profile.birth_year, player.profile.city, player.profile.country, player.profile.email, player.profile.phone, player.profile.notes, player.profile.photo]).map_err(|_| ApplicationError::Storage)?;
        Ok(())
    }

    fn update_player(&mut self, player: &Player) -> Result<(), ApplicationError> {
        let changed = self.connection.execute("UPDATE players SET name=?2, club=?3, birth_year=?4, city=?5, country=?6, email=?7, phone=?8, notes=?9, photo=?10 WHERE id=?1",
            params![player.id.to_string(), player.name, player.club, player.profile.birth_year, player.profile.city, player.profile.country, player.profile.email, player.profile.phone, player.profile.notes, player.profile.photo]).map_err(|_| ApplicationError::Storage)?;
        if changed == 0 {
            return Err(ApplicationError::NotFound);
        }
        Ok(())
    }

    fn list_entries(&self, category_id: Uuid) -> Result<Vec<Entry>, ApplicationError> {
        // Validate category existence rather than treating a bad ID as an empty category.
        let exists: bool = self
            .connection
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM categories WHERE id = ?1)",
                [category_id.to_string()],
                |row| row.get(0),
            )
            .map_err(|_| ApplicationError::Storage)?;
        if !exists {
            return Err(ApplicationError::NotFound);
        }
        let mut statement = self
            .connection
            .prepare("SELECT id FROM entries WHERE category_id = ?1 ORDER BY rowid")
            .map_err(|_| ApplicationError::Storage)?;
        let ids = statement
            .query_map([category_id.to_string()], |row| row.get::<_, String>(0))
            .map_err(|_| ApplicationError::Storage)?;
        ids.map(|id| {
            let id = id.map_err(|_| ApplicationError::Storage)?;
            let mut members_statement = self.connection.prepare("SELECT player_id, name_snapshot, club_snapshot FROM entry_members WHERE entry_id = ?1 ORDER BY position").map_err(|_| ApplicationError::Storage)?;
            let rows = members_statement.query_map([&id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))).map_err(|_| ApplicationError::Storage)?;
            let members = rows.map(|row| {
                let (id, name, club) = row.map_err(|_| ApplicationError::Storage)?;
                Ok(EntryMember { id: Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?, name, club })
            }).collect::<Result<Vec<_>, ApplicationError>>()?;
            Ok(Entry { id: Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?, category_id, members })
        }).collect()
    }

    fn insert_entry(&mut self, entry: &Entry) -> Result<(), ApplicationError> {
        let transaction = self
            .connection
            .transaction()
            .map_err(|_| ApplicationError::Storage)?;
        transaction
            .execute(
                "INSERT INTO entries (id, category_id) VALUES (?1, ?2)",
                params![entry.id.to_string(), entry.category_id.to_string()],
            )
            .map_err(|_| ApplicationError::Storage)?;
        for (index, member) in entry.members.iter().enumerate() {
            let claimed: bool = transaction.query_row("SELECT EXISTS(SELECT 1 FROM entry_members WHERE category_id = ?1 AND player_id = ?2)", params![entry.category_id.to_string(), member.id.to_string()], |row| row.get(0)).map_err(|_| ApplicationError::Storage)?;
            if claimed {
                return Err(ApplicationError::AlreadyRegistered);
            }
            transaction.execute("INSERT INTO entry_members (entry_id, category_id, player_id, position, name_snapshot, club_snapshot) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", params![entry.id.to_string(), entry.category_id.to_string(), member.id.to_string(), index + 1, member.name, member.club]).map_err(|_| ApplicationError::Storage)?;
        }
        transaction.commit().map_err(|_| ApplicationError::Storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use librett_application::{add_category, create_player, create_tournament, register_entry};

    #[test]
    fn profiles_persist_and_edits_leave_registered_names_unchanged() {
        use librett_application::save_player_profile;
        let path = std::env::temp_dir().join(format!("librett-profile-{}.sqlite", Uuid::new_v4()));
        let expected;
        let category_id;
        let snapshot;
        {
            let mut repository = SqliteTournamentRepository::open(&path).unwrap();
            let t = create_tournament(&mut repository, "Kup").unwrap();
            let t = add_category(
                &mut repository,
                t.id,
                "Singl",
                Discipline::Singles,
                CompetitionFormat::Knockout,
            )
            .unwrap();
            category_id = t.categories[0].id;
            let player = save_player_profile(
                &mut repository,
                None,
                "Željko",
                "Klub",
                PlayerProfile {
                    birth_year: Some(1980),
                    city: "  Niš  ".into(),
                    notes: "Beleške".into(),
                    photo: Some(
                        include_str!("../tests/fixtures/player-photo.txt")
                            .trim()
                            .into(),
                    ),
                    ..Default::default()
                },
            )
            .unwrap();
            snapshot = register_entry(&mut repository, t.id, category_id, vec![player.id]).unwrap();
            expected = save_player_profile(
                &mut repository,
                Some(player.id),
                "Novo ime",
                "Novi klub",
                player.profile,
            )
            .unwrap();
            assert_eq!(expected.profile.city, "Niš");
            assert_eq!(
                save_player_profile(
                    &mut repository,
                    Some(player.id),
                    "Bad",
                    "",
                    PlayerProfile {
                        birth_year: Some(1800),
                        ..Default::default()
                    }
                ),
                Err(ApplicationError::InvalidProfile)
            );
            assert_eq!(repository.find_player(player.id).unwrap(), expected);
        }
        let repository = SqliteTournamentRepository::open(&path).unwrap();
        assert_eq!(repository.list_players().unwrap(), vec![expected]);
        assert_eq!(
            repository.list_entries(category_id).unwrap(),
            vec![snapshot]
        );
        drop(repository);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn v2_migration_preserves_players_entries_and_backs_up_once() {
        let dir = std::env::temp_dir().join(format!("librett-v3-{}", Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        let path = dir.join("data.sqlite");
        let player_id = Uuid::new_v4();
        let tournament_id = Uuid::new_v4();
        let category_id = Uuid::new_v4();
        let entry_id = Uuid::new_v4();
        {
            let connection = Connection::open(&path).unwrap();
            connection
                .execute_batch(include_str!("../migrations/001_tournaments.sql"))
                .unwrap();
            connection
                .execute_batch(include_str!("../migrations/002_players.sql"))
                .unwrap();
            connection
                .execute(
                    "INSERT INTO tournaments (id,name) VALUES (?1,'Kup')",
                    [tournament_id.to_string()],
                )
                .unwrap();
            connection.execute("INSERT INTO categories (id,tournament_id,name,name_key,discipline,format) VALUES (?1,?2,'Singl','singl','singles','knockout')", params![category_id.to_string(),tournament_id.to_string()]).unwrap();
            connection
                .execute(
                    "INSERT INTO players (id,name,club) VALUES (?1,'Old name','Klub')",
                    [player_id.to_string()],
                )
                .unwrap();
            connection
                .execute(
                    "INSERT INTO entries (id,category_id) VALUES (?1,?2)",
                    params![entry_id.to_string(), category_id.to_string()],
                )
                .unwrap();
            connection.execute("INSERT INTO entry_members (entry_id,category_id,player_id,position,name_snapshot,club_snapshot) VALUES (?1,?2,?3,1,'Snapshot','Klub')", params![entry_id.to_string(),category_id.to_string(),player_id.to_string()]).unwrap();
        }
        for _ in 0..2 {
            let repository = SqliteTournamentRepository::open(&path).unwrap();
            assert_eq!(
                repository.find_player(player_id).unwrap().profile,
                PlayerProfile::default()
            );
            assert_eq!(
                repository.list_entries(category_id).unwrap()[0].members[0].name,
                "Snapshot"
            );
        }
        let files = std::fs::read_dir(&dir)
            .unwrap()
            .map(|item| item.unwrap().path())
            .collect::<Vec<_>>();
        assert_eq!(files.len(), 2);
        let backup = files.iter().find(|file| **file != path).unwrap();
        let backup_connection = Connection::open(backup).unwrap();
        assert_eq!(
            backup_connection
                .query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            2
        );
        drop(backup_connection);
        for file in files {
            std::fs::remove_file(file).unwrap();
        }
        std::fs::remove_dir(dir).unwrap();
    }

    #[test]
    fn registrations_preserve_snapshots_and_duplicate_pairs_roll_back() {
        let mut repository =
            SqliteTournamentRepository::initialize(Connection::open_in_memory().unwrap()).unwrap();
        let t = create_tournament(&mut repository, "Kup").unwrap();
        add_category(
            &mut repository,
            t.id,
            "Singl",
            Discipline::Singles,
            CompetitionFormat::Knockout,
        )
        .unwrap();
        let t = add_category(
            &mut repository,
            t.id,
            "Dubl",
            Discipline::Doubles,
            CompetitionFormat::GroupsKnockout,
        )
        .unwrap();
        let a = create_player(&mut repository, "Aleksa", "Bubušinac").unwrap();
        let b = create_player(&mut repository, "Marko", "").unwrap();
        let c = create_player(&mut repository, "Petar", "").unwrap();
        let singles =
            register_entry(&mut repository, t.id, t.categories[0].id, vec![a.id]).unwrap();
        register_entry(&mut repository, t.id, t.categories[1].id, vec![a.id, b.id]).unwrap();
        assert_eq!(
            register_entry(&mut repository, t.id, t.categories[1].id, vec![c.id, a.id]),
            Err(ApplicationError::AlreadyRegistered)
        );
        // Exercise transactional rollback directly: the first member would be inserted before the collision.
        let conflicting =
            Entry::new(t.categories[1].id, Discipline::Doubles, vec![c, a.clone()]).unwrap();
        assert_eq!(
            repository.insert_entry(&conflicting),
            Err(ApplicationError::AlreadyRegistered)
        );
        assert_eq!(
            repository.list_entries(t.categories[1].id).unwrap().len(),
            1
        );
        repository
            .connection
            .execute(
                "UPDATE players SET name = 'Changed', club = 'Changed' WHERE id = ?1",
                [a.id.to_string()],
            )
            .unwrap();
        assert_eq!(
            repository.list_entries(t.categories[0].id).unwrap(),
            vec![singles]
        );
        assert_eq!(repository.list_players().unwrap().len(), 3);
        assert_eq!(
            register_entry(
                &mut repository,
                Uuid::new_v4(),
                t.categories[0].id,
                vec![b.id]
            ),
            Err(ApplicationError::NotFound)
        );
        assert_eq!(
            register_entry(
                &mut repository,
                t.id,
                t.categories[0].id,
                vec![Uuid::new_v4()]
            ),
            Err(ApplicationError::NotFound)
        );
    }

    #[test]
    fn migration_preserves_existing_tournaments_and_creates_backup() {
        let dir = std::env::temp_dir().join(format!("librett-migration-{}", Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        let path = dir.join("data.sqlite");
        let id = Uuid::new_v4();
        {
            let connection = Connection::open(&path).unwrap();
            connection
                .execute_batch(include_str!("../migrations/001_tournaments.sql"))
                .unwrap();
            connection
                .execute(
                    "INSERT INTO tournaments (id, name) VALUES (?1, 'Existing')",
                    [id.to_string()],
                )
                .unwrap();
        }
        {
            let mut repository = SqliteTournamentRepository::open(&path).unwrap();
            assert_eq!(repository.find(id).unwrap().name, "Existing");
            create_player(&mut repository, "Željko", "").unwrap();
        }
        {
            let repository = SqliteTournamentRepository::open(&path).unwrap();
            assert_eq!(repository.list_players().unwrap()[0].name, "Željko");
        }
        let files = std::fs::read_dir(&dir)
            .unwrap()
            .map(|item| item.unwrap().path())
            .collect::<Vec<_>>();
        assert_eq!(files.len(), 2);
        let backup = files.iter().find(|p| **p != path).unwrap();
        let connection = Connection::open(backup).unwrap();
        assert_eq!(
            connection
                .query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            1
        );
        drop(connection);
        for file in files {
            std::fs::remove_file(file).unwrap();
        }
        std::fs::remove_dir(dir).unwrap();
    }
}
