use super::*;
use librett_application::{
    add_category_with_fee, create_player, create_tournament, record_cash, register_entry,
    PlayerCashRepository,
};
use librett_domain::{cash_share, CashKind, CashRecord};

#[test]
fn player_settlement_splits_pairs_and_is_atomic_idempotent_and_persistent() {
    let path = std::env::temp_dir().join(format!("librett-player-cash-{}.sqlite", Uuid::new_v4()));
    let tid;
    let player;
    let partner;
    let singles;
    let doubles;
    {
        let mut r = SqliteTournamentRepository::open(&path).unwrap();
        tid = create_tournament(&mut r, "Cup").unwrap().id;
        player = create_player(&mut r, "Aleksa", "").unwrap().id;
        partner = create_player(&mut r, "Marko", "").unwrap().id;
        let t = add_category_with_fee(
            &mut r,
            tid,
            "Apsolutna",
            Discipline::Singles,
            CompetitionFormat::Knockout,
            100_000,
        )
        .unwrap();
        singles = register_entry(&mut r, tid, t.categories[0].id, vec![player])
            .unwrap()
            .id;
        let t = add_category_with_fee(
            &mut r,
            tid,
            "Apsolutna",
            Discipline::Doubles,
            CompetitionFormat::Knockout,
            50_000,
        )
        .unwrap();
        doubles = register_entry(&mut r, tid, t.categories[1].id, vec![player, partner])
            .unwrap()
            .id;
        let record = CashRecord {
            id: Uuid::new_v4(),
            entry_id: singles,
            kind: CashKind::Payment,
            amount_minor: 40_000,
            note: String::new(),
            created_at: String::new(),
        };
        record_cash(&mut r, tid, record).unwrap();
        let request = Uuid::new_v4();
        r.connection.execute_batch("CREATE TEMP TRIGGER fail_second_settlement BEFORE INSERT ON cash_allocations WHEN (SELECT count(*) FROM cash_allocations)>0 BEGIN SELECT RAISE(ABORT,'test later failure'); END;").unwrap();
        assert_eq!(
            r.settle_player_cash(request, tid, player, vec![singles, doubles], true),
            Err(ApplicationError::Storage)
        );
        assert_eq!(r.cash_ledger(tid).unwrap().records.len(), 3);
        assert!(r.cash_ledger(tid).unwrap().allocations.is_empty());
        r.connection
            .execute_batch("DROP TRIGGER fail_second_settlement;")
            .unwrap();
        r.settle_player_cash(request, tid, player, vec![singles, doubles], true)
            .unwrap();
        let saved = r.cash_ledger(tid).unwrap();
        assert_eq!(saved.allocations.len(), 2);
        assert_eq!(
            saved
                .allocations
                .iter()
                .map(|a| a.amount_minor)
                .sum::<i64>(),
            85_000
        );
        assert_eq!(
            saved
                .allocations
                .iter()
                .find(|a| saved
                    .records
                    .iter()
                    .any(|rec| rec.id == a.record_id && rec.entry_id == doubles))
                .unwrap()
                .amount_minor,
            25_000
        );
        r.settle_player_cash(request, tid, player, vec![doubles, singles], true)
            .unwrap();
        assert_eq!(r.cash_ledger(tid).unwrap(), saved);
        assert_eq!(
            r.settle_player_cash(request, tid, player, vec![singles, doubles], false),
            Err(ApplicationError::InvalidCash)
        );
        assert_eq!(
            r.settle_player_cash(Uuid::new_v4(), tid, partner, vec![singles], true),
            Err(ApplicationError::NotFound)
        );
        assert_eq!(
            r.settle_player_cash(Uuid::new_v4(), Uuid::new_v4(), player, vec![singles], true),
            Err(ApplicationError::NotFound)
        );
        assert_eq!(
            r.settle_player_cash(Uuid::new_v4(), tid, player, vec![singles, singles], true),
            Err(ApplicationError::InvalidCash)
        );
        r.settle_player_cash(Uuid::new_v4(), tid, partner, vec![doubles], true)
            .unwrap();
        assert_eq!(
            r.cash_ledger(tid)
                .unwrap()
                .allocations
                .last()
                .unwrap()
                .amount_minor,
            25_000
        );
        // A fresh click on an already-paid account also adds no payment.
        let before = r.cash_ledger(tid).unwrap();
        r.settle_player_cash(Uuid::new_v4(), tid, partner, vec![doubles], true)
            .unwrap();
        assert_eq!(r.cash_ledger(tid).unwrap(), before);
        // Refunding one player does not refund the partner's allocation.
        r.settle_player_cash(Uuid::new_v4(), tid, player, vec![doubles], false)
            .unwrap();
        let refund = r.cash_ledger(tid).unwrap();
        assert_eq!(refund.records.last().unwrap().kind, CashKind::Refund);
        assert_eq!(refund.allocations.last().unwrap().player_id, player);
        assert_eq!(refund.allocations.last().unwrap().amount_minor, 25_000);
        r.settle_player_cash(Uuid::new_v4(), tid, player, vec![doubles], true)
            .unwrap();
        assert_eq!(
            r.cash_ledger(tid)
                .unwrap()
                .allocations
                .last()
                .unwrap()
                .amount_minor,
            25_000
        );
        assert!(r
            .connection
            .execute("DELETE FROM cash_allocations", [])
            .is_err());
        assert!(r
            .connection
            .execute("UPDATE cash_settlements SET paid=0", [])
            .is_err());
    }
    let r = SqliteTournamentRepository::open(&path).unwrap();
    let ledger = r.cash_ledger(tid).unwrap();
    let net = ledger
        .records
        .iter()
        .map(|r| match r.kind {
            CashKind::Payment => r.amount_minor,
            CashKind::Refund => -r.amount_minor,
            _ => 0,
        })
        .sum::<i64>();
    assert_eq!(net, 150_000);
    drop(r);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn historical_pair_payments_and_discounts_split_without_rounding_loss() {
    let mut r =
        SqliteTournamentRepository::initialize(Connection::open_in_memory().unwrap()).unwrap();
    let tid = create_tournament(&mut r, "Cup").unwrap().id;
    let a = create_player(&mut r, "A", "").unwrap().id;
    let b = create_player(&mut r, "B", "").unwrap().id;
    let t = add_category_with_fee(
        &mut r,
        tid,
        "Doubles",
        Discipline::Doubles,
        CompetitionFormat::Knockout,
        50_001,
    )
    .unwrap();
    let e = register_entry(&mut r, tid, t.categories[0].id, vec![a, b])
        .unwrap()
        .id;
    for (kind, amount) in [(CashKind::Discount, 1), (CashKind::Payment, 10_001)] {
        record_cash(
            &mut r,
            tid,
            CashRecord {
                id: Uuid::new_v4(),
                entry_id: e,
                kind,
                amount_minor: amount,
                note: String::new(),
                created_at: String::new(),
            },
        )
        .unwrap();
    }
    r.settle_player_cash(Uuid::new_v4(), tid, a, vec![e], true)
        .unwrap();
    assert_eq!(
        r.cash_ledger(tid).unwrap().allocations[0].amount_minor,
        19_999
    );
    r.settle_player_cash(Uuid::new_v4(), tid, b, vec![e], true)
        .unwrap();
    assert_eq!(
        r.cash_ledger(tid).unwrap().allocations[1].amount_minor,
        20_000
    );
    assert_eq!(cash_share(50_001, 0, 2), 25_001);
    assert_eq!(cash_share(50_001, 1, 2), 25_000);
    assert_eq!(cash_share(-1, 0, 2), -1);
    assert_eq!(cash_share(-1, 1, 2), 0);
}

#[test]
fn v8_migration_preserves_historical_payments_and_backs_up_once() {
    let dir = std::env::temp_dir().join(format!("librett-player-ledger-{}", Uuid::new_v4()));
    std::fs::create_dir(&dir).unwrap();
    let path = dir.join("data.sqlite");
    let tid = Uuid::new_v4();
    let cid = Uuid::new_v4();
    let eid = Uuid::new_v4();
    let c = Connection::open(&path).unwrap();
    for sql in [
        include_str!("../migrations/001_tournaments.sql"),
        include_str!("../migrations/002_players.sql"),
        include_str!("../migrations/003_player_profiles.sql"),
        include_str!("../migrations/004_registration_status.sql"),
        include_str!("../migrations/005_cash.sql"),
        include_str!("../migrations/006_category_fees.sql"),
        include_str!("../migrations/007_category_archive.sql"),
        include_str!("../migrations/008_category_names.sql"),
    ] {
        c.execute_batch(sql).unwrap();
    }
    c.execute(
        "INSERT INTO tournaments(id,name) VALUES(?1,'Cup')",
        [tid.to_string()],
    )
    .unwrap();
    c.execute("INSERT INTO categories(id,tournament_id,name,name_key,discipline,format) VALUES(?1,?2,'Singles','singles','singles','knockout')",params![cid.to_string(),tid.to_string()]).unwrap();
    c.execute(
        "INSERT INTO entries(id,category_id) VALUES(?1,?2)",
        params![eid.to_string(), cid.to_string()],
    )
    .unwrap();
    for (kind, amount) in [("charge", 4_000_000), ("payment", 3_500_000)] {
        c.execute("INSERT INTO cash_records(id,entry_id,kind,amount_minor,note) VALUES(?1,?2,?3,?4,'Historical')",params![Uuid::new_v4().to_string(),eid.to_string(),kind,amount]).unwrap();
    }
    drop(c);
    for _ in 0..2 {
        let r = SqliteTournamentRepository::open(&path).unwrap();
        let cash = r.cash_ledger(tid).unwrap();
        assert_eq!(cash.records.len(), 2);
        assert!(cash.allocations.is_empty());
        assert_eq!(
            cash.records[0].amount_minor - cash.records[1].amount_minor,
            500_000
        );
    }
    let files = std::fs::read_dir(&dir)
        .unwrap()
        .map(|f| f.unwrap().path())
        .collect::<Vec<_>>();
    assert_eq!(files.len(), 2);
    let backup = files.iter().find(|p| **p != path).unwrap();
    assert!(backup.to_string_lossy().contains("pre-v9"));
    let c = Connection::open(backup).unwrap();
    assert_eq!(
        c.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
            .unwrap(),
        8
    );
    drop(c);
    for f in files {
        std::fs::remove_file(f).unwrap();
    }
    std::fs::remove_dir(dir).unwrap();
}
