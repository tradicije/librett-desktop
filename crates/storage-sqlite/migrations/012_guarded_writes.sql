CREATE TABLE player_writes (
    id TEXT PRIMARY KEY NOT NULL,
    player_id TEXT NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    request_payload TEXT NOT NULL,
    result_payload TEXT NOT NULL
);
ALTER TABLE cash_settlements ADD COLUMN expected_payload TEXT NOT NULL DEFAULT '';
PRAGMA user_version = 12;
