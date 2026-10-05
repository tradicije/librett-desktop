CREATE TABLE knockout_fillers (
 draw_id TEXT PRIMARY KEY REFERENCES category_draws(id),
 revision INTEGER NOT NULL CHECK(revision > 0),
 payload TEXT NOT NULL
);
PRAGMA user_version = 16;
