ALTER TABLE entries ADD COLUMN status TEXT NOT NULL DEFAULT 'registered' CHECK (status IN ('registered', 'withdrawn'));
CREATE TABLE player_attendance (
    tournament_id TEXT NOT NULL REFERENCES tournaments(id),
    player_id TEXT NOT NULL REFERENCES players(id),
    checked_in INTEGER NOT NULL CHECK (checked_in IN (0, 1)),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    PRIMARY KEY (tournament_id, player_id)
);
PRAGMA user_version = 4;
