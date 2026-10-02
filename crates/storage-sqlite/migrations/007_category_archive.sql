ALTER TABLE categories ADD COLUMN archived INTEGER NOT NULL DEFAULT 0 CHECK(archived IN (0,1));
PRAGMA user_version = 7;
