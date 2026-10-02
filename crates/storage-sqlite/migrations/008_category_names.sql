-- Executed with foreign keys disabled outside the migration transaction.
CREATE TABLE categories_new (
    id TEXT PRIMARY KEY NOT NULL,
    tournament_id TEXT NOT NULL REFERENCES tournaments(id),
    name TEXT NOT NULL CHECK(length(trim(name)) BETWEEN 1 AND 120),
    name_key TEXT NOT NULL,
    discipline TEXT NOT NULL CHECK(discipline IN ('singles','doubles')),
    format TEXT NOT NULL CHECK(format IN ('knockout','groups_knockout')),
    fee_minor INTEGER NOT NULL DEFAULT 0 CHECK(fee_minor BETWEEN 0 AND 1000000000),
    archived INTEGER NOT NULL DEFAULT 0 CHECK(archived IN (0,1)),
    UNIQUE(tournament_id, discipline, name_key)
);
INSERT INTO categories_new (id,tournament_id,name,name_key,discipline,format,fee_minor,archived)
SELECT id,tournament_id,name,name_key,discipline,format,fee_minor,archived FROM categories ORDER BY rowid;
DROP TABLE categories;
ALTER TABLE categories_new RENAME TO categories;
PRAGMA user_version = 8;
