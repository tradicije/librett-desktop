CREATE TABLE cash_allocations (
    record_id TEXT NOT NULL REFERENCES cash_records(id),
    player_id TEXT NOT NULL REFERENCES players(id),
    amount_minor INTEGER NOT NULL CHECK(amount_minor BETWEEN 1 AND 1000000000),
    PRIMARY KEY(record_id, player_id)
);
CREATE TABLE cash_settlements (
    id TEXT PRIMARY KEY NOT NULL,
    tournament_id TEXT NOT NULL REFERENCES tournaments(id),
    player_id TEXT NOT NULL REFERENCES players(id),
    entry_keys TEXT NOT NULL,
    paid INTEGER NOT NULL CHECK(paid IN (0,1))
);
CREATE TRIGGER allocation_no_update BEFORE UPDATE ON cash_allocations BEGIN SELECT RAISE(ABORT,'Cash allocations are immutable'); END;
CREATE TRIGGER allocation_no_delete BEFORE DELETE ON cash_allocations BEGIN SELECT RAISE(ABORT,'Cash allocations are immutable'); END;
CREATE TRIGGER settlement_no_update BEFORE UPDATE ON cash_settlements BEGIN SELECT RAISE(ABORT,'Settlements are immutable'); END;
CREATE TRIGGER settlement_no_delete BEFORE DELETE ON cash_settlements BEGIN SELECT RAISE(ABORT,'Settlements are immutable'); END;
PRAGMA user_version = 9;
