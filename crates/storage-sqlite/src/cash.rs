use super::*;
use librett_application::CashRepository;
use librett_domain::{CashBalance, CashKind, CashRecord};

fn records(
    connection: &Connection,
    tournament_id: Uuid,
) -> Result<Vec<CashRecord>, ApplicationError> {
    let mut statement = connection.prepare("SELECT r.id,r.entry_id,r.kind,r.amount_minor,r.note,r.created_at FROM cash_records r JOIN entries e ON e.id=r.entry_id JOIN categories c ON c.id=e.category_id WHERE c.tournament_id=?1 ORDER BY r.rowid").map_err(|_| ApplicationError::Storage)?;
    let rows = statement
        .query_map([tournament_id.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })
        .map_err(|_| ApplicationError::Storage)?;
    rows.map(|row| {
        let (id, entry, kind, amount_minor, note, created_at) =
            row.map_err(|_| ApplicationError::Storage)?;
        Ok(CashRecord {
            id: Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?,
            entry_id: Uuid::parse_str(&entry).map_err(|_| ApplicationError::Storage)?,
            kind: match kind.as_str() {
                "charge" => CashKind::Charge,
                "discount" => CashKind::Discount,
                "payment" => CashKind::Payment,
                "refund" => CashKind::Refund,
                _ => return Err(ApplicationError::Storage),
            },
            amount_minor,
            note,
            created_at,
        })
    })
    .collect()
}

// The cash ledger is immutable. Eligibility is evaluated against current registrations
// and attendance under the same write lock as the payment.
fn ensure_payment(
    conn: &Connection,
    tournament: Uuid,
    entry: Uuid,
    player: Option<Uuid>,
) -> Result<(), ApplicationError> {
    super::trash::ensure_active(conn, tournament)?;
    let active: Option<bool> = conn.query_row("SELECT e.status='registered' AND c.archived=0 FROM entries e JOIN categories c ON c.id=e.category_id WHERE e.id=?1 AND c.tournament_id=?2", params![entry.to_string(),tournament.to_string()], |r|r.get(0)).optional().map_err(|_|ApplicationError::Storage)?;
    match active {
        None => return Err(ApplicationError::NotFound),
        Some(false) => return Err(ApplicationError::RegistrationInactive),
        Some(true) => {}
    }
    let missing: bool = conn.query_row("SELECT EXISTS(SELECT 1 FROM entry_members m LEFT JOIN player_attendance a ON a.tournament_id=?1 AND a.player_id=m.player_id WHERE m.entry_id=?2 AND (?3 IS NULL OR m.player_id=?3) AND COALESCE(a.checked_in,0)=0)", params![tournament.to_string(),entry.to_string(),player.map(|p|p.to_string())], |r|r.get(0)).map_err(|_|ApplicationError::Storage)?;
    if missing {
        Err(ApplicationError::AttendanceRequired)
    } else {
        Ok(())
    }
}

impl CashRepository for SqliteTournamentRepository {
    fn list_cash(&self, tournament_id: Uuid) -> Result<Vec<CashRecord>, ApplicationError> {
        self.find(tournament_id)?;
        records(&self.connection, tournament_id)
    }
    fn record_cash(
        &mut self,
        tournament_id: Uuid,
        record: CashRecord,
    ) -> Result<(), ApplicationError> {
        // Immediate locking prevents concurrent balance checks from both succeeding.
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        let exists:bool=tx.query_row("SELECT EXISTS(SELECT 1 FROM entries e JOIN categories c ON c.id=e.category_id WHERE e.id=?1 AND c.tournament_id=?2)",params![record.entry_id.to_string(),tournament_id.to_string()],|r|r.get(0)).map_err(|_|ApplicationError::Storage)?;
        if !exists {
            return Err(ApplicationError::NotFound);
        }
        let current = records(&tx, tournament_id)?;
        if let Some(old) = current.iter().find(|r| r.id == record.id) {
            return if old.entry_id == record.entry_id
                && old.kind == record.kind
                && old.amount_minor == record.amount_minor
                && old.note == record.note
            {
                Ok(())
            } else {
                Err(ApplicationError::InvalidCash)
            };
        }
        if record.kind == CashKind::Payment {
            ensure_payment(&tx, tournament_id, record.entry_id, None)?;
        }
        let mut balance = CashBalance::default();
        for r in current.iter().filter(|r| r.entry_id == record.entry_id) {
            balance
                .apply(r.kind, r.amount_minor)
                .map_err(|_| ApplicationError::Storage)?;
        }
        if matches!(record.kind, CashKind::Payment | CashKind::Refund) {
            let allocated: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM cash_allocations a JOIN cash_records r ON r.id=a.record_id WHERE r.entry_id=?1)", [record.entry_id.to_string()], |r| r.get(0)).map_err(|_| ApplicationError::Storage)?;
            if allocated {
                return Err(ApplicationError::InvalidCash);
            }
        }
        balance.apply(record.kind, record.amount_minor)?;
        tx.execute(
            "INSERT INTO cash_records (id,entry_id,kind,amount_minor,note) VALUES (?1,?2,?3,?4,?5)",
            params![
                record.id.to_string(),
                record.entry_id.to_string(),
                record.kind.as_str(),
                record.amount_minor,
                record.note
            ],
        )
        .map_err(|_| ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use librett_application::{
        add_category, create_player, create_tournament, record_cash, register_entry,
        set_entry_status,
    };
    #[test]
    fn ledger_scopes_retries_refunds_withdrawal_restart_and_immutability() {
        let path = std::env::temp_dir().join(format!("librett-cash-{}.sqlite", Uuid::new_v4()));
        let tournament;
        let entry;
        {
            let mut repo = SqliteTournamentRepository::open(&path).unwrap();
            let t = create_tournament(&mut repo, "Cup").unwrap();
            tournament = t.id;
            let t = add_category(
                &mut repo,
                t.id,
                "Singles",
                Discipline::Singles,
                CompetitionFormat::Knockout,
            )
            .unwrap();
            let p = create_player(&mut repo, "Player", "").unwrap();
            entry = register_entry(&mut repo, t.id, t.categories[0].id, vec![p.id])
                .unwrap()
                .id;
            librett_application::set_player_attendance(&mut repo, tournament, p.id, true).unwrap();
            let charge = CashRecord {
                id: Uuid::new_v4(),
                entry_id: entry,
                kind: CashKind::Charge,
                amount_minor: 100_000,
                note: "Fee".into(),
                created_at: String::new(),
            };
            assert_eq!(
                record_cash(&mut repo, Uuid::new_v4(), charge.clone()),
                Err(ApplicationError::NotFound)
            );
            record_cash(&mut repo, tournament, charge.clone()).unwrap();
            record_cash(&mut repo, tournament, charge.clone()).unwrap();
            let mut conflict = charge.clone();
            conflict.amount_minor += 1;
            assert_eq!(
                record_cash(&mut repo, tournament, conflict),
                Err(ApplicationError::InvalidCash)
            );
            for (kind, amount) in [
                (CashKind::Discount, 20_000),
                (CashKind::Payment, 30_000),
                (CashKind::Payment, 60_000),
                (CashKind::Refund, 10_000),
            ] {
                record_cash(
                    &mut repo,
                    tournament,
                    CashRecord {
                        id: Uuid::new_v4(),
                        kind,
                        amount_minor: amount,
                        ..charge.clone()
                    },
                )
                .unwrap();
            }
            let invalid = CashRecord {
                id: Uuid::new_v4(),
                kind: CashKind::Refund,
                amount_minor: 80_001,
                ..charge
            };
            assert_eq!(
                record_cash(&mut repo, tournament, invalid),
                Err(ApplicationError::InvalidCash)
            );
            set_entry_status(
                &mut repo,
                tournament,
                entry,
                librett_domain::EntryStatus::Withdrawn,
            )
            .unwrap();
            assert!(repo
                .connection
                .execute("DELETE FROM cash_records", [])
                .is_err());
            assert!(repo
                .connection
                .execute("UPDATE cash_records SET amount_minor=1", [])
                .is_err());
        }
        let repo = SqliteTournamentRepository::open(&path).unwrap();
        let saved = repo.list_cash(tournament).unwrap();
        assert_eq!(saved.len(), 5);
        let mut b = CashBalance::default();
        for r in saved {
            assert_eq!(r.entry_id, entry);
            assert!(!r.created_at.is_empty());
            b.apply(r.kind, r.amount_minor).unwrap();
        }
        assert_eq!(b.charges - b.discounts - b.payments + b.refunds, 0);
        drop(repo);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn v4_backup_is_created_once_before_cash_migration() {
        let dir = std::env::temp_dir().join(format!("librett-cash-migration-{}", Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        let path = dir.join("data.sqlite");
        let c = Connection::open(&path).unwrap();
        for migration in [
            include_str!("../migrations/001_tournaments.sql"),
            include_str!("../migrations/002_players.sql"),
            include_str!("../migrations/003_player_profiles.sql"),
            include_str!("../migrations/004_registration_status.sql"),
        ] {
            c.execute_batch(migration).unwrap();
        }
        drop(c);
        for _ in 0..2 {
            let r = SqliteTournamentRepository::open(&path).unwrap();
            assert_eq!(
                r.connection
                    .query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
                    .unwrap(),
                22
            );
        }
        let files = std::fs::read_dir(&dir)
            .unwrap()
            .map(|r| r.unwrap().path())
            .collect::<Vec<_>>();
        assert_eq!(files.len(), 2);
        let backup = files.iter().find(|p| **p != path).unwrap();
        assert!(backup.to_string_lossy().contains("pre-v22"));
        let c = Connection::open(backup).unwrap();
        assert_eq!(
            c.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            4
        );
        drop(c);
        for f in files {
            std::fs::remove_file(f).unwrap();
        }
        std::fs::remove_dir(dir).unwrap();
    }
}

#[cfg(test)]
mod category_fee_tests {
    use super::*;
    use librett_application::{
        add_category_with_fee, create_player, create_tournament, register_entry, PlayerRepository,
    };

    #[test]
    fn category_fees_charge_once_per_entry_and_roll_back_atomically() {
        let path = std::env::temp_dir().join(format!("librett-fees-{}.sqlite", Uuid::new_v4()));
        let tid;
        let paid_category;
        let paid_entry;
        {
            let mut r = SqliteTournamentRepository::open(&path).unwrap();
            tid = create_tournament(&mut r, "Cup").unwrap().id;
            assert_eq!(
                add_category_with_fee(
                    &mut r,
                    tid,
                    "Invalid",
                    Discipline::Singles,
                    CompetitionFormat::Knockout,
                    -1
                ),
                Err(ApplicationError::InvalidCash)
            );
            assert!(r.find(tid).unwrap().categories.is_empty());
            let t = add_category_with_fee(
                &mut r,
                tid,
                "Singles",
                Discipline::Singles,
                CompetitionFormat::Knockout,
                100_001,
            )
            .unwrap();
            paid_category = t.categories[0].id;
            let p = create_player(&mut r, "One", "").unwrap();
            let q = create_player(&mut r, "Two", "").unwrap();
            paid_entry = register_entry(&mut r, tid, paid_category, vec![p.id])
                .unwrap()
                .id;
            assert_eq!(
                register_entry(&mut r, tid, paid_category, vec![p.id]),
                Err(ApplicationError::AlreadyRegistered)
            );
            let records = r.list_cash(tid).unwrap();
            assert_eq!(records.len(), 1);
            assert_eq!(records[0].amount_minor, 100_001);
            assert_eq!(records[0].kind, CashKind::Charge);
            let t = add_category_with_fee(
                &mut r,
                tid,
                "Doubles",
                Discipline::Doubles,
                CompetitionFormat::Knockout,
                50_050,
            )
            .unwrap();
            register_entry(&mut r, tid, t.categories[1].id, vec![p.id, q.id]).unwrap();
            let records = r.list_cash(tid).unwrap();
            assert_eq!(records.len(), 2);
            assert_eq!(records[1].amount_minor, 50_050);
            let t = add_category_with_fee(
                &mut r,
                tid,
                "Free",
                Discipline::Singles,
                CompetitionFormat::Knockout,
                0,
            )
            .unwrap();
            register_entry(&mut r, tid, t.categories[2].id, vec![p.id]).unwrap();
            assert_eq!(r.list_cash(tid).unwrap().len(), 2);
            // Simulate an insertion failure after members have been written.
            r.connection.execute_batch("CREATE TEMP TRIGGER fail_cash BEFORE INSERT ON cash_records BEGIN SELECT RAISE(ABORT,'test failure'); END;").unwrap();
            assert_eq!(
                register_entry(&mut r, tid, paid_category, vec![q.id]),
                Err(ApplicationError::Storage)
            );
            assert_eq!(r.list_entries(paid_category).unwrap().len(), 1);
            assert_eq!(r.list_cash(tid).unwrap().len(), 2);
            r.connection
                .execute_batch("DROP TRIGGER fail_cash;")
                .unwrap();
            register_entry(&mut r, tid, paid_category, vec![q.id]).unwrap();
            assert_eq!(r.list_cash(tid).unwrap().len(), 3);
        }
        let r = SqliteTournamentRepository::open(&path).unwrap();
        assert_eq!(r.find(tid).unwrap().categories[0].fee_minor, 100_001);
        let records = r.list_cash(tid).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].entry_id, paid_entry);
        drop(r);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn v5_migration_defaults_fees_and_preserves_existing_cash_without_backfill() {
        let dir = std::env::temp_dir().join(format!("librett-fee-migration-{}", Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        let path = dir.join("data.sqlite");
        let tid = Uuid::new_v4();
        let cid = Uuid::new_v4();
        let eid = Uuid::new_v4();
        let rid = Uuid::new_v4();
        let c = Connection::open(&path).unwrap();
        for migration in [
            include_str!("../migrations/001_tournaments.sql"),
            include_str!("../migrations/002_players.sql"),
            include_str!("../migrations/003_player_profiles.sql"),
            include_str!("../migrations/004_registration_status.sql"),
            include_str!("../migrations/005_cash.sql"),
        ] {
            c.execute_batch(migration).unwrap();
        }
        c.execute(
            "INSERT INTO tournaments(id,name) VALUES(?1,'Old cup')",
            [tid.to_string()],
        )
        .unwrap();
        c.execute("INSERT INTO categories(id,tournament_id,name,name_key,discipline,format) VALUES(?1,?2,'Old','old','singles','knockout')",params![cid.to_string(),tid.to_string()]).unwrap();
        c.execute(
            "INSERT INTO entries(id,category_id) VALUES(?1,?2)",
            params![eid.to_string(), cid.to_string()],
        )
        .unwrap();
        c.execute("INSERT INTO cash_records(id,entry_id,kind,amount_minor,note) VALUES(?1,?2,'charge',25000,'Manual fee')",params![rid.to_string(),eid.to_string()]).unwrap();
        drop(c);
        for _ in 0..2 {
            let r = SqliteTournamentRepository::open(&path).unwrap();
            assert_eq!(r.find(tid).unwrap().categories[0].fee_minor, 0);
            let records = r.list_cash(tid).unwrap();
            assert_eq!(records.len(), 1);
            assert_eq!(records[0].id, rid);
            assert_eq!(records[0].amount_minor, 25000);
        }
        let files = std::fs::read_dir(&dir)
            .unwrap()
            .map(|f| f.unwrap().path())
            .collect::<Vec<_>>();
        assert_eq!(files.len(), 2);
        let backup = files.iter().find(|p| **p != path).unwrap();
        assert!(backup.to_string_lossy().contains("pre-v22"));
        let c = Connection::open(backup).unwrap();
        assert_eq!(
            c.query_row("PRAGMA user_version", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            5
        );
        drop(c);
        for f in files {
            std::fs::remove_file(f).unwrap();
        }
        std::fs::remove_dir(dir).unwrap();
    }
}

fn allocations(
    connection: &Connection,
    tournament_id: Uuid,
) -> Result<Vec<librett_domain::CashAllocation>, ApplicationError> {
    let mut statement = connection.prepare("SELECT a.record_id,a.player_id,a.amount_minor FROM cash_allocations a JOIN cash_records r ON r.id=a.record_id JOIN entries e ON e.id=r.entry_id JOIN categories c ON c.id=e.category_id WHERE c.tournament_id=?1 ORDER BY r.rowid,a.player_id").map_err(|_|ApplicationError::Storage)?;
    let rows = statement
        .query_map([tournament_id.to_string()], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)?,
            ))
        })
        .map_err(|_| ApplicationError::Storage)?;
    rows.map(|r| {
        let (id, player, amount_minor) = r.map_err(|_| ApplicationError::Storage)?;
        Ok(librett_domain::CashAllocation {
            record_id: Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?,
            player_id: Uuid::parse_str(&player).map_err(|_| ApplicationError::Storage)?,
            amount_minor,
        })
    })
    .collect()
}

impl librett_application::PlayerCashRepository for SqliteTournamentRepository {
    fn cash_ledger(
        &self,
        tournament_id: Uuid,
    ) -> Result<librett_domain::CashLedger, ApplicationError> {
        self.find(tournament_id)?;
        Ok(librett_domain::CashLedger {
            records: records(&self.connection, tournament_id)?,
            allocations: allocations(&self.connection, tournament_id)?,
        })
    }
    fn settle_player_cash(
        &mut self,
        request_id: Uuid,
        tournament_id: Uuid,
        player_id: Uuid,
        entry_ids: Vec<Uuid>,
        paid: bool,
    ) -> Result<(), ApplicationError> {
        self.settle_cash(request_id, tournament_id, player_id, entry_ids, paid, None)
    }
    fn settle_player_cash_checked(
        &mut self,
        request_id: Uuid,
        tournament_id: Uuid,
        player_id: Uuid,
        entry_ids: Vec<Uuid>,
        paid: bool,
        expected: Vec<librett_application::ExpectedCashAmount>,
    ) -> Result<(), ApplicationError> {
        self.settle_cash(
            request_id,
            tournament_id,
            player_id,
            entry_ids,
            paid,
            Some(expected),
        )
    }
}
impl SqliteTournamentRepository {
    fn settle_cash(
        &mut self,
        request_id: Uuid,
        tournament_id: Uuid,
        player_id: Uuid,
        mut entry_ids: Vec<Uuid>,
        paid: bool,
        mut expected: Option<Vec<librett_application::ExpectedCashAmount>>,
    ) -> Result<(), ApplicationError> {
        if entry_ids.is_empty() || entry_ids.len() > 1000 {
            return Err(ApplicationError::InvalidCash);
        }
        entry_ids.sort();
        if entry_ids.windows(2).any(|ids| ids[0] == ids[1]) {
            return Err(ApplicationError::InvalidCash);
        }
        if let Some(amounts) = &mut expected {
            amounts.sort_by_key(|a| a.entry_id);
            if amounts.len() != entry_ids.len()
                || amounts.iter().zip(&entry_ids).any(|(amount, id)| {
                    amount.entry_id != *id
                        || !(0..=librett_domain::MAX_CASH_MINOR).contains(&amount.amount_minor)
                })
            {
                return Err(ApplicationError::InvalidCash);
            }
        }
        let expected_payload = expected
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|_| ApplicationError::Storage)?
            .unwrap_or_default();
        let keys = entry_ids
            .iter()
            .map(Uuid::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        let old: Option<(String, String, String, bool, String)> = tx
            .query_row(
                "SELECT tournament_id,player_id,entry_keys,paid,expected_payload FROM cash_settlements WHERE id=?1",
                [request_id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?;
        if let Some((t, p, k, state, previous_expected)) = old {
            return if t == tournament_id.to_string()
                && p == player_id.to_string()
                && k == keys
                && state == paid
                && previous_expected == expected_payload
            {
                Ok(())
            } else {
                Err(ApplicationError::InvalidCash)
            };
        }
        let current = records(&tx, tournament_id)?;
        let allocated = allocations(&tx, tournament_id)?;
        for (index, entry_id) in entry_ids.iter().enumerate() {
            let exists:bool=tx.query_row("SELECT EXISTS(SELECT 1 FROM entries e JOIN categories c ON c.id=e.category_id JOIN entry_members m ON m.entry_id=e.id WHERE e.id=?1 AND c.tournament_id=?2 AND m.player_id=?3)",params![entry_id.to_string(),tournament_id.to_string(),player_id.to_string()],|r|r.get(0)).map_err(|_|ApplicationError::Storage)?;
            if !exists {
                return Err(ApplicationError::NotFound);
            }
            if paid {
                ensure_payment(&tx, tournament_id, *entry_id, Some(player_id))?;
            }
            let (position,count):(i64,i64)=tx.query_row("SELECT m.position,(SELECT count(*) FROM entry_members WHERE entry_id=?1) FROM entry_members m WHERE m.entry_id=?1 AND m.player_id=?2",params![entry_id.to_string(),player_id.to_string()],|r|Ok((r.get(0)?,r.get(1)?))).map_err(|_|ApplicationError::Storage)?;
            let mut whole = CashBalance::default();
            let mut unallocated_net = 0;
            let mut allocated_net = 0;
            for record in current.iter().filter(|r| r.entry_id == *entry_id) {
                whole
                    .apply(record.kind, record.amount_minor)
                    .map_err(|_| ApplicationError::Storage)?;
                let sign = match record.kind {
                    CashKind::Payment => 1,
                    CashKind::Refund => -1,
                    _ => 0,
                };
                if allocated.iter().any(|a| a.record_id == record.id) {
                    allocated_net += sign
                        * allocated
                            .iter()
                            .filter(|a| a.record_id == record.id && a.player_id == player_id)
                            .map(|a| a.amount_minor)
                            .sum::<i64>();
                } else {
                    unallocated_net += sign * record.amount_minor;
                }
            }
            let due = librett_domain::cash_share(
                whole.charges - whole.discounts,
                (position - 1) as usize,
                count as usize,
            );
            let net = librett_domain::cash_share(
                unallocated_net,
                (position - 1) as usize,
                count as usize,
            ) + allocated_net;
            if net < 0 {
                return Err(ApplicationError::InvalidCash);
            }
            let amount = if paid { (due - net).max(0) } else { net.max(0) };
            if expected
                .as_ref()
                .is_some_and(|amounts| amounts[index].amount_minor != amount)
            {
                return Err(ApplicationError::CashConflict);
            }
            if amount == 0 {
                continue;
            }
            let kind = if paid {
                CashKind::Payment
            } else {
                CashKind::Refund
            };
            whole.apply(kind, amount)?;
            let id = Uuid::new_v4();
            tx.execute("INSERT INTO cash_records(id,entry_id,kind,amount_minor,note) VALUES(?1,?2,?3,?4,'')",params![id.to_string(),entry_id.to_string(),kind.as_str(),amount]).map_err(|_|ApplicationError::Storage)?;
            tx.execute(
                "INSERT INTO cash_allocations(record_id,player_id,amount_minor) VALUES(?1,?2,?3)",
                params![id.to_string(), player_id.to_string(), amount],
            )
            .map_err(|_| ApplicationError::Storage)?;
        }
        tx.execute("INSERT INTO cash_settlements(id,tournament_id,player_id,entry_keys,paid,expected_payload) VALUES(?1,?2,?3,?4,?5,?6)",params![request_id.to_string(),tournament_id.to_string(),player_id.to_string(),keys,paid,expected_payload]).map_err(|_|ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)
    }
}
