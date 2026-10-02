use super::*;
use librett_application::{
    add_category_with_fee, create_player, create_tournament, record_cash, register_entries,
    register_entry, CashRepository, CategoryRepository, PlayerRepository,
};
use librett_domain::{CashKind, CashRecord};

#[test]
fn batch_entries_are_atomic_and_archived_categories_keep_cash_history() {
    let path = std::env::temp_dir().join(format!("librett-category-{}.sqlite", Uuid::new_v4()));
    let tid;
    let cid;
    let expected;
    {
        let mut r = SqliteTournamentRepository::open(&path).unwrap();
        tid = create_tournament(&mut r, "Cup").unwrap().id;
        let t = add_category_with_fee(
            &mut r,
            tid,
            "Singles",
            Discipline::Singles,
            CompetitionFormat::Knockout,
            100_000,
        )
        .unwrap();
        cid = t.categories[0].id;
        let players = (0..4)
            .map(|i| {
                create_player(&mut r, &format!("Player {i}"), "")
                    .unwrap()
                    .id
            })
            .collect::<Vec<_>>();
        assert_eq!(
            register_entries(&mut r, tid, cid, vec![vec![players[0]], vec![players[0]]]),
            Err(ApplicationError::AlreadyRegistered)
        );
        assert!(r.list_entries(cid).unwrap().is_empty());
        assert!(r.list_cash(tid).unwrap().is_empty());
        // Failure on a later entry must roll back earlier entries and fees too.
        r.connection.execute_batch("CREATE TEMP TRIGGER fail_second BEFORE INSERT ON cash_records WHEN (SELECT count(*) FROM cash_records)>0 BEGIN SELECT RAISE(ABORT,'later failure'); END;").unwrap();
        assert_eq!(
            register_entries(&mut r, tid, cid, vec![vec![players[0]], vec![players[1]]]),
            Err(ApplicationError::Storage)
        );
        assert!(r.list_entries(cid).unwrap().is_empty());
        assert!(r.list_cash(tid).unwrap().is_empty());
        r.connection
            .execute_batch("DROP TRIGGER fail_second;")
            .unwrap();
        let entries =
            register_entries(&mut r, tid, cid, vec![vec![players[0]], vec![players[1]]]).unwrap();
        assert_eq!(entries.len(), 2);
        let note = CashRecord {
            id: Uuid::new_v4(),
            entry_id: entries[0].id,
            kind: CashKind::Payment,
            amount_minor: 40_000,
            note: "  ".into(),
            created_at: String::new(),
        };
        record_cash(&mut r, tid, note.clone()).unwrap();
        assert_eq!(r.list_cash(tid).unwrap().last().unwrap().note, "");
        assert_eq!(
            record_cash(
                &mut r,
                tid,
                CashRecord {
                    id: Uuid::new_v4(),
                    note: "x".repeat(501),
                    ..note
                }
            ),
            Err(ApplicationError::InvalidCash)
        );
        assert_eq!(
            register_entries(&mut r, Uuid::new_v4(), cid, vec![vec![players[2]]]),
            Err(ApplicationError::NotFound)
        );
        assert_eq!(
            r.delete_category(Uuid::new_v4(), cid),
            Err(ApplicationError::NotFound)
        );
        r.delete_category(tid, cid).unwrap();
        assert!(r.find(tid).unwrap().categories[0].archived);
        expected = r.list_cash(tid).unwrap();
        assert_eq!(expected.len(), 3);
        assert_eq!(
            register_entry(&mut r, tid, cid, vec![players[2]]),
            Err(ApplicationError::NotFound)
        );
        assert_eq!(
            register_entries(&mut r, tid, cid, vec![vec![players[2]]]),
            Err(ApplicationError::NotFound)
        );
        let t = add_category_with_fee(
            &mut r,
            tid,
            "Empty",
            Discipline::Singles,
            CompetitionFormat::Knockout,
            0,
        )
        .unwrap();
        let empty = t.categories[1].id;
        r.delete_category(tid, empty).unwrap();
        assert_eq!(r.find(tid).unwrap().categories.len(), 1);
        let t = add_category_with_fee(
            &mut r,
            tid,
            "Pairs",
            Discipline::Doubles,
            CompetitionFormat::Knockout,
            200_000,
        )
        .unwrap();
        let pair = t.categories[1].id;
        assert_eq!(
            register_entries(&mut r, tid, pair, vec![vec![players[0]]]),
            Err(ApplicationError::InvalidMembers)
        );
        let entries = register_entries(
            &mut r,
            tid,
            pair,
            vec![vec![players[0], players[1]], vec![players[2], players[3]]],
        )
        .unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(r.list_cash(tid).unwrap().len(), 5);
    }
    let r = SqliteTournamentRepository::open(&path).unwrap();
    assert!(r.find(tid).unwrap().categories[0].archived);
    assert_eq!(r.list_entries(cid).unwrap().len(), 2);
    assert_eq!(&r.list_cash(tid).unwrap()[..3], expected.as_slice());
    drop(r);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn v6_migration_preserves_fee_and_creates_one_pre_v8_backup() {
    let dir = std::env::temp_dir().join(format!("librett-archive-migration-{}", Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let path = dir.join("data.sqlite");
    let tid = Uuid::new_v4();
    let cid = Uuid::new_v4();
    let c = Connection::open(&path).unwrap();
    for sql in [
        include_str!("../migrations/001_tournaments.sql"),
        include_str!("../migrations/002_players.sql"),
        include_str!("../migrations/003_player_profiles.sql"),
        include_str!("../migrations/004_registration_status.sql"),
        include_str!("../migrations/005_cash.sql"),
        include_str!("../migrations/006_category_fees.sql"),
    ] {
        c.execute_batch(sql).unwrap();
    }
    c.execute(
        "INSERT INTO tournaments(id,name) VALUES(?1,'Old cup')",
        [tid.to_string()],
    )
    .unwrap();
    c.execute("INSERT INTO categories(id,tournament_id,name,name_key,discipline,format,fee_minor) VALUES(?1,?2,'Old','old','singles','knockout',12345)",params![cid.to_string(),tid.to_string()]).unwrap();
    drop(c);
    for _ in 0..2 {
        let r = SqliteTournamentRepository::open(&path).unwrap();
        let t = r.find(tid).unwrap();
        assert_eq!(t.categories[0].fee_minor, 12345);
        assert!(!t.categories[0].archived);
        assert_eq!(
            r.connection
                .query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            8
        );
    }
    let files = std::fs::read_dir(&dir)
        .unwrap()
        .map(|f| f.unwrap().path())
        .collect::<Vec<_>>();
    assert_eq!(files.len(), 2);
    let backup = files.iter().find(|p| **p != path).unwrap();
    assert!(backup.to_string_lossy().contains("pre-v8"));
    let c = Connection::open(backup).unwrap();
    assert_eq!(
        c.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        6
    );
    drop(c);
    for f in files {
        std::fs::remove_file(f).unwrap();
    }
    std::fs::remove_dir(dir).unwrap();
}

#[test]
fn v7_category_names_allow_different_disciplines_and_preserve_references() {
    let dir = std::env::temp_dir().join(format!("librett-names-{}", Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let path = dir.join("data.sqlite");
    let tid = Uuid::new_v4();
    let cid = Uuid::new_v4();
    let pid = Uuid::new_v4();
    let eid = Uuid::new_v4();
    let rid = Uuid::new_v4();
    let c = Connection::open(&path).unwrap();
    for sql in [
        include_str!("../migrations/001_tournaments.sql"),
        include_str!("../migrations/002_players.sql"),
        include_str!("../migrations/003_player_profiles.sql"),
        include_str!("../migrations/004_registration_status.sql"),
        include_str!("../migrations/005_cash.sql"),
        include_str!("../migrations/006_category_fees.sql"),
        include_str!("../migrations/007_category_archive.sql"),
    ] {
        c.execute_batch(sql).unwrap();
    }
    c.execute(
        "INSERT INTO tournaments(id,name) VALUES(?1,'Cup')",
        [tid.to_string()],
    )
    .unwrap();
    c.execute("INSERT INTO categories(id,tournament_id,name,name_key,discipline,format,fee_minor) VALUES(?1,?2,'Apsolutna','apsolutna','singles','knockout',12345)",params![cid.to_string(),tid.to_string()]).unwrap();
    c.execute(
        "INSERT INTO players(id,name,club) VALUES(?1,'Current name','Club')",
        [pid.to_string()],
    )
    .unwrap();
    c.execute(
        "INSERT INTO entries(id,category_id) VALUES(?1,?2)",
        params![eid.to_string(), cid.to_string()],
    )
    .unwrap();
    c.execute("INSERT INTO entry_members(entry_id,category_id,player_id,position,name_snapshot,club_snapshot) VALUES(?1,?2,?3,1,'Snapshot','Old Club')",params![eid.to_string(),cid.to_string(),pid.to_string()]).unwrap();
    c.execute(
        "INSERT INTO player_attendance(tournament_id,player_id,checked_in) VALUES(?1,?2,1)",
        params![tid.to_string(), pid.to_string()],
    )
    .unwrap();
    c.execute("INSERT INTO cash_records(id,entry_id,kind,amount_minor,note) VALUES(?1,?2,'charge',12345,'Historical fee')",params![rid.to_string(),eid.to_string()]).unwrap();
    drop(c);
    {
        let mut r = SqliteTournamentRepository::open(&path).unwrap();
        let t = add_category_with_fee(
            &mut r,
            tid,
            " APSOLUTNA ",
            Discipline::Doubles,
            CompetitionFormat::Knockout,
            45600,
        )
        .unwrap();
        assert_eq!(t.categories.len(), 2);
        assert_eq!(
            add_category_with_fee(
                &mut r,
                tid,
                "apsolutna",
                Discipline::Singles,
                CompetitionFormat::Knockout,
                0
            ),
            Err(ApplicationError::DuplicateCategory)
        );
        assert_eq!(
            add_category_with_fee(
                &mut r,
                tid,
                "apsolutna",
                Discipline::Doubles,
                CompetitionFormat::Knockout,
                0
            ),
            Err(ApplicationError::DuplicateCategory)
        );
        // The DB also protects against callers bypassing application validation.
        let mut duplicate = t.categories[1].clone();
        duplicate.id = Uuid::new_v4();
        assert!(r.insert_category(tid, &duplicate).is_err());
        let entry = r.list_entries(cid).unwrap().remove(0);
        assert_eq!(entry.id, eid);
        assert_eq!(entry.members[0].name, "Snapshot");
        assert!(entry.members[0].checked_in);
        let cash = r.list_cash(tid).unwrap();
        assert_eq!(cash.len(), 1);
        assert_eq!(cash[0].id, rid);
        assert_eq!(cash[0].amount_minor, 12345);
        assert_eq!(
            r.connection
                .query_row("PRAGMA foreign_keys", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            1
        );
        assert!(r
            .connection
            .execute("DELETE FROM categories WHERE id=?1", [cid.to_string()])
            .is_err());
        assert!(r
            .connection
            .execute("DELETE FROM cash_records", [])
            .is_err());
    }
    let r = SqliteTournamentRepository::open(&path).unwrap();
    assert_eq!(r.find(tid).unwrap().categories.len(), 2);
    drop(r);
    let files = std::fs::read_dir(&dir)
        .unwrap()
        .map(|f| f.unwrap().path())
        .collect::<Vec<_>>();
    assert_eq!(files.len(), 2);
    let backup = files.iter().find(|p| **p != path).unwrap();
    assert!(backup.to_string_lossy().contains("pre-v8"));
    let c = Connection::open(backup).unwrap();
    assert_eq!(
        c.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        7
    );
    drop(c);
    for f in files {
        std::fs::remove_file(f).unwrap();
    }
    std::fs::remove_dir(dir).unwrap();
}
