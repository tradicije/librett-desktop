CREATE TABLE tournaments (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL CHECK (length(trim(name)) BETWEEN 1 AND 120)
);
CREATE TABLE categories (
    id TEXT PRIMARY KEY NOT NULL,
    tournament_id TEXT NOT NULL REFERENCES tournaments(id),
    name TEXT NOT NULL CHECK (length(trim(name)) BETWEEN 1 AND 120),
    name_key TEXT NOT NULL,
    discipline TEXT NOT NULL CHECK (discipline IN ('singles', 'doubles')),
    format TEXT NOT NULL CHECK (format IN ('knockout', 'groups_knockout')),
    UNIQUE (tournament_id, name_key)
);
PRAGMA user_version = 1;
