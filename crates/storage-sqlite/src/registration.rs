use super::*;
use librett_application::RegistrationRepository;
use librett_domain::EntryStatus;

impl RegistrationRepository for SqliteTournamentRepository {
    fn set_entry_status(
        &mut self,
        tournament_id: Uuid,
        entry_id: Uuid,
        status: EntryStatus,
    ) -> Result<(), ApplicationError> {
        let status = match status {
            EntryStatus::Registered => "registered",
            EntryStatus::Withdrawn => "withdrawn",
        };
        let changed = self.connection.execute("UPDATE entries SET status = ?3 WHERE id = ?1 AND category_id IN (SELECT id FROM categories WHERE tournament_id = ?2)", params![entry_id.to_string(), tournament_id.to_string(), status]).map_err(|_| ApplicationError::Storage)?;
        if changed == 0 {
            return Err(ApplicationError::NotFound);
        }
        Ok(())
    }

    fn set_player_attendance(
        &mut self,
        tournament_id: Uuid,
        player_id: Uuid,
        checked_in: bool,
    ) -> Result<(), ApplicationError> {
        // INSERT ... SELECT checks membership and writes in one statement.
        let changed = self.connection.execute("INSERT INTO player_attendance (tournament_id, player_id, checked_in) SELECT ?1, ?2, ?3 WHERE EXISTS (SELECT 1 FROM entry_members em JOIN categories c ON c.id = em.category_id WHERE c.tournament_id = ?1 AND em.player_id = ?2) ON CONFLICT (tournament_id, player_id) DO UPDATE SET checked_in = excluded.checked_in, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')", params![tournament_id.to_string(), player_id.to_string(), checked_in]).map_err(|_| ApplicationError::Storage)?;
        if changed == 0 {
            return Err(ApplicationError::NotFound);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use librett_application::{
        add_category, create_player, create_tournament, register_entry, set_entry_status,
        set_player_attendance, PlayerRepository,
    };

    #[test]
    fn attendance_is_tournament_wide_and_withdrawal_preserves_membership() {
        let path =
            std::env::temp_dir().join(format!("librett-attendance-{}.sqlite", Uuid::new_v4()));
        let first_category;
        let doubles_category;
        let other_category;
        let tournament_id;
        let other_tournament_id;
        let player_id;
        let partner_id;
        let singles_id;
        {
            let mut repository = SqliteTournamentRepository::open(&path).unwrap();
            let tournament = create_tournament(&mut repository, "Cup").unwrap();
            tournament_id = tournament.id;
            add_category(
                &mut repository,
                tournament.id,
                "Singles",
                Discipline::Singles,
                CompetitionFormat::Knockout,
            )
            .unwrap();
            let tournament = add_category(
                &mut repository,
                tournament.id,
                "Doubles",
                Discipline::Doubles,
                CompetitionFormat::Knockout,
            )
            .unwrap();
            first_category = tournament.categories[0].id;
            doubles_category = tournament.categories[1].id;
            let other = create_tournament(&mut repository, "Other cup").unwrap();
            other_tournament_id = other.id;
            let other = add_category(
                &mut repository,
                other.id,
                "Singles",
                Discipline::Singles,
                CompetitionFormat::Knockout,
            )
            .unwrap();
            other_category = other.categories[0].id;
            let player = create_player(&mut repository, "Aleksa", "Club").unwrap();
            player_id = player.id;
            let partner = create_player(&mut repository, "Partner", "").unwrap();
            partner_id = partner.id;
            let unregistered = create_player(&mut repository, "Unregistered", "").unwrap();
            let singles = register_entry(
                &mut repository,
                tournament_id,
                first_category,
                vec![player_id],
            )
            .unwrap();
            singles_id = singles.id;
            register_entry(
                &mut repository,
                tournament_id,
                doubles_category,
                vec![player_id, partner_id],
            )
            .unwrap();
            register_entry(
                &mut repository,
                other_tournament_id,
                other_category,
                vec![player_id],
            )
            .unwrap();
            assert!(!repository.list_entries(first_category).unwrap()[0].members[0].checked_in);
            assert_eq!(
                set_player_attendance(&mut repository, tournament_id, unregistered.id, true),
                Err(ApplicationError::NotFound)
            );
            assert_eq!(
                set_entry_status(
                    &mut repository,
                    other_tournament_id,
                    singles_id,
                    EntryStatus::Withdrawn
                ),
                Err(ApplicationError::NotFound)
            );
            assert_eq!(
                set_player_attendance(&mut repository, other_tournament_id, partner_id, true),
                Err(ApplicationError::NotFound)
            );
            set_player_attendance(&mut repository, tournament_id, player_id, true).unwrap();
            let extra = add_category(
                &mut repository,
                tournament_id,
                "Veterans",
                Discipline::Singles,
                CompetitionFormat::Knockout,
            )
            .unwrap();
            let extra_entry = register_entry(
                &mut repository,
                tournament_id,
                extra.categories.last().unwrap().id,
                vec![player_id],
            )
            .unwrap();
            assert!(extra_entry.members[0].checked_in);
            let doubles = &repository.list_entries(doubles_category).unwrap()[0];
            assert!(doubles.members[0].checked_in);
            assert!(!doubles.members[1].checked_in);
            assert!(!repository.list_entries(other_category).unwrap()[0].members[0].checked_in);
            set_entry_status(
                &mut repository,
                tournament_id,
                singles_id,
                EntryStatus::Withdrawn,
            )
            .unwrap();
            assert_eq!(
                register_entry(
                    &mut repository,
                    tournament_id,
                    first_category,
                    vec![player_id]
                ),
                Err(ApplicationError::AlreadyRegistered)
            );
            assert_eq!(
                repository.delete_player(player_id),
                Err(ApplicationError::PlayerInUse)
            );
        }
        {
            let mut repository = SqliteTournamentRepository::open(&path).unwrap();
            let singles = &repository.list_entries(first_category).unwrap()[0];
            assert_eq!(singles.id, singles_id);
            assert_eq!(singles.status, EntryStatus::Withdrawn);
            assert_eq!(singles.members[0].name, "Aleksa");
            assert!(singles.members[0].checked_in);
            set_entry_status(
                &mut repository,
                tournament_id,
                singles_id,
                EntryStatus::Registered,
            )
            .unwrap();
            set_entry_status(
                &mut repository,
                tournament_id,
                singles_id,
                EntryStatus::Registered,
            )
            .unwrap();
            set_player_attendance(&mut repository, tournament_id, player_id, false).unwrap();
            set_player_attendance(&mut repository, tournament_id, partner_id, true).unwrap();
            let doubles = &repository.list_entries(doubles_category).unwrap()[0];
            assert!(!doubles.members[0].checked_in);
            assert!(doubles.members[1].checked_in);
            assert_eq!(
                repository.list_entries(first_category).unwrap()[0].status,
                EntryStatus::Registered
            );
            assert!(!repository.list_entries(other_category).unwrap()[0].members[0].checked_in);
        }
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn v3_migration_preserves_entries_defaults_status_and_creates_one_backup() {
        let dir = std::env::temp_dir().join(format!("librett-v4-{}", Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        let path = dir.join("data.sqlite");
        let tournament_id = Uuid::new_v4();
        let category_id = Uuid::new_v4();
        let player_id = Uuid::new_v4();
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
                .execute_batch(include_str!("../migrations/003_player_profiles.sql"))
                .unwrap();
            connection
                .execute(
                    "INSERT INTO tournaments (id,name) VALUES (?1,'Cup')",
                    [tournament_id.to_string()],
                )
                .unwrap();
            connection.execute("INSERT INTO categories (id,tournament_id,name,name_key,discipline,format) VALUES (?1,?2,'Singles','singles','singles','knockout')", params![category_id.to_string(), tournament_id.to_string()]).unwrap();
            connection
                .execute(
                    "INSERT INTO players (id,name,club) VALUES (?1,'Name','Club')",
                    [player_id.to_string()],
                )
                .unwrap();
            connection
                .execute(
                    "INSERT INTO entries (id,category_id) VALUES (?1,?2)",
                    params![entry_id.to_string(), category_id.to_string()],
                )
                .unwrap();
            connection.execute("INSERT INTO entry_members (entry_id,category_id,player_id,position,name_snapshot,club_snapshot) VALUES (?1,?2,?3,1,'Historical name','Historical club')",params![entry_id.to_string(),category_id.to_string(),player_id.to_string()]).unwrap();
        }
        for _ in 0..2 {
            let repository = SqliteTournamentRepository::open(&path).unwrap();
            let entry = &repository.list_entries(category_id).unwrap()[0];
            assert_eq!(entry.id, entry_id);
            assert_eq!(entry.status, EntryStatus::Registered);
            assert!(!entry.members[0].checked_in);
            assert_eq!(entry.members[0].name, "Historical name");
        }
        let files = std::fs::read_dir(&dir)
            .unwrap()
            .map(|item| item.unwrap().path())
            .collect::<Vec<_>>();
        assert_eq!(files.len(), 2);
        let backup = files.iter().find(|p| **p != path).unwrap();
        assert!(backup.to_string_lossy().contains("pre-v9"));
        let connection = Connection::open(backup).unwrap();
        assert_eq!(
            connection
                .query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            3
        );
        drop(connection);
        for file in files {
            std::fs::remove_file(file).unwrap();
        }
        std::fs::remove_dir(dir).unwrap();
    }
}
