CREATE TABLE tournament_scheduling (
 tournament_id TEXT PRIMARY KEY REFERENCES tournaments(id),
 table_count INTEGER NOT NULL DEFAULT 0 CHECK(table_count BETWEEN 0 AND 128),
 revision INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE match_assignments (
 tournament_id TEXT NOT NULL REFERENCES tournaments(id),
 draw_id TEXT NOT NULL REFERENCES category_draws(id),
 match_key TEXT NOT NULL,
 table_number INTEGER NOT NULL CHECK(table_number BETWEEN 1 AND 128),
 status TEXT NOT NULL CHECK(status IN ('queued','running','released')),
 payload TEXT NOT NULL,
 started_at TEXT,
 PRIMARY KEY(draw_id,match_key)
);
CREATE UNIQUE INDEX occupied_table ON match_assignments(tournament_id,table_number) WHERE status != 'released';
CREATE TABLE schedule_writes (id TEXT PRIMARY KEY, request_payload TEXT NOT NULL, result_payload TEXT NOT NULL);
PRAGMA application_id = 1279415380;
PRAGMA user_version = 18;
