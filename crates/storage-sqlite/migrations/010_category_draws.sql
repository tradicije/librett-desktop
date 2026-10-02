CREATE TABLE category_draws (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL REFERENCES categories(id),
    revision INTEGER NOT NULL CHECK(revision > 0),
    request_payload TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    UNIQUE(category_id, revision)
);
CREATE TRIGGER draw_no_update BEFORE UPDATE ON category_draws BEGIN SELECT RAISE(ABORT,'Draw revisions are immutable'); END;
CREATE TRIGGER draw_no_delete BEFORE DELETE ON category_draws BEGIN SELECT RAISE(ABORT,'Draw revisions are immutable'); END;
PRAGMA user_version = 10;
