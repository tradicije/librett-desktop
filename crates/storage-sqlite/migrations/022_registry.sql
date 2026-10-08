-- Copyright (C) 2026 Aleksa Dimitrijević. AGPL-3.0-or-later.
CREATE TABLE registry_sources(registry_id TEXT PRIMARY KEY,name TEXT NOT NULL,checkpoint TEXT NOT NULL,source_url TEXT,payload TEXT NOT NULL,semantic_hash TEXT NOT NULL);
CREATE TABLE registry_links(registry_id TEXT NOT NULL REFERENCES registry_sources(registry_id),remote_id TEXT NOT NULL,local_id TEXT UNIQUE REFERENCES players(id) ON DELETE SET NULL,remote_revision TEXT NOT NULL,last_remote TEXT NOT NULL,overrides TEXT NOT NULL CHECK(json_valid(overrides)),upstream_state TEXT NOT NULL CHECK(upstream_state IN('live','withdrawn')),PRIMARY KEY(registry_id,remote_id));
CREATE TABLE registry_clubs(registry_id TEXT NOT NULL REFERENCES registry_sources(registry_id),remote_id TEXT NOT NULL,local_id TEXT NOT NULL UNIQUE,name TEXT NOT NULL,revision TEXT NOT NULL,payload TEXT NOT NULL,upstream_state TEXT NOT NULL CHECK(upstream_state IN('live','withdrawn')),PRIMARY KEY(registry_id,remote_id));
CREATE TABLE registry_memberships(registry_id TEXT NOT NULL,player_remote_id TEXT NOT NULL,club_remote_id TEXT NOT NULL,PRIMARY KEY(registry_id,player_remote_id,club_remote_id),FOREIGN KEY(registry_id,player_remote_id) REFERENCES registry_links(registry_id,remote_id) ON DELETE CASCADE,FOREIGN KEY(registry_id,club_remote_id) REFERENCES registry_clubs(registry_id,remote_id) ON DELETE CASCADE);
CREATE TABLE registry_jobs(id TEXT PRIMARY KEY,created_at INTEGER NOT NULL,payload TEXT NOT NULL,preview TEXT NOT NULL);
CREATE TABLE registry_receipts(id TEXT PRIMARY KEY,result TEXT NOT NULL);
CREATE TABLE registry_import_context(singleton INTEGER PRIMARY KEY CHECK(singleton=1),active INTEGER NOT NULL CHECK(active IN(0,1)));
INSERT INTO registry_import_context VALUES(1,0);
CREATE TRIGGER registry_local_overrides AFTER UPDATE ON players WHEN (SELECT active FROM registry_import_context WHERE singleton=1)=0
BEGIN
 UPDATE registry_links SET overrides=json_set(overrides,'$.name',json('true')) WHERE local_id=NEW.id AND OLD.name IS NOT NEW.name;
 UPDATE registry_links SET overrides=json_set(overrides,'$.club',json('true')) WHERE local_id=NEW.id AND OLD.club IS NOT NEW.club;
 UPDATE registry_links SET overrides=json_set(overrides,'$.birth_year',json('true')) WHERE local_id=NEW.id AND OLD.birth_year IS NOT NEW.birth_year;
 UPDATE registry_links SET overrides=json_set(overrides,'$.country',json('true')) WHERE local_id=NEW.id AND OLD.country IS NOT NEW.country;
 UPDATE registry_links SET overrides=json_set(overrides,'$.photo',json('true')) WHERE local_id=NEW.id AND OLD.photo IS NOT NEW.photo;
END;
PRAGMA user_version=22;
