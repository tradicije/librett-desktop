CREATE TABLE match_results (
    id INTEGER PRIMARY KEY,
    draw_id TEXT NOT NULL REFERENCES category_draws(id),
    match_key TEXT NOT NULL,
    revision INTEGER NOT NULL,
    payload TEXT NOT NULL,
    recorded_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    UNIQUE(draw_id, match_key, revision)
);
CREATE TABLE match_writes (
    id TEXT PRIMARY KEY,
    request_payload TEXT NOT NULL,
    result_payload TEXT NOT NULL
);
PRAGMA user_version = 14;
