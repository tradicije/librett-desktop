CREATE TABLE cash_records (
    id TEXT PRIMARY KEY NOT NULL,
    entry_id TEXT NOT NULL REFERENCES entries(id),
    kind TEXT NOT NULL CHECK(kind IN ('charge','discount','payment','refund')),
    amount_minor INTEGER NOT NULL CHECK(amount_minor BETWEEN 1 AND 1000000000),
    note TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
);
CREATE INDEX cash_records_entry ON cash_records(entry_id);
CREATE TRIGGER cash_no_update BEFORE UPDATE ON cash_records BEGIN SELECT RAISE(ABORT,'Cash records are immutable'); END;
CREATE TRIGGER cash_no_delete BEFORE DELETE ON cash_records BEGIN SELECT RAISE(ABORT,'Cash records are immutable'); END;
PRAGMA user_version = 5;
