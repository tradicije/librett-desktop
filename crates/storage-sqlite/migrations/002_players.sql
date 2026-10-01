CREATE TABLE players (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL CHECK (length(trim(name)) BETWEEN 1 AND 120),
    club TEXT NOT NULL CHECK (length(club) <= 120)
);
CREATE TABLE entries (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL REFERENCES categories(id),
    UNIQUE (id, category_id)
);
CREATE TABLE entry_members (
    entry_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    player_id TEXT NOT NULL REFERENCES players(id),
    position INTEGER NOT NULL CHECK (position IN (1, 2)),
    name_snapshot TEXT NOT NULL,
    club_snapshot TEXT NOT NULL,
    FOREIGN KEY (entry_id, category_id) REFERENCES entries(id, category_id),
    PRIMARY KEY (entry_id, position),
    UNIQUE (category_id, player_id)
);
PRAGMA user_version = 2;
