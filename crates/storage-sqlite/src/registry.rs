// Copyright (C) 2026 Aleksa Dimitrijević. AGPL-3.0-or-later.
use super::*;
use crate::players::read_player;
use librett_application::registry::*;
use librett_domain::{Player, PlayerProfile};
use std::collections::{HashMap, HashSet};

fn player_on(connection: &Connection, id: Uuid) -> Result<Option<Player>, ApplicationError> {
    connection.query_row("SELECT id,name,club,birth_year,city,country,email,phone,notes,photo FROM players WHERE id=?1",[id.to_string()],read_player).optional().map_err(|_|ApplicationError::Storage)
}
fn source_on(
    connection: &Connection,
    id: Uuid,
) -> Result<Option<(String, String)>, ApplicationError> {
    connection
        .query_row(
            "SELECT checkpoint,semantic_hash FROM registry_sources WHERE registry_id=?1",
            [id.to_string()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|_| ApplicationError::Storage)
}
fn write_player(tx: &Connection, player: &Player) -> Result<(), ApplicationError> {
    tx.execute("INSERT INTO players(id,name,club,birth_year,city,country,email,phone,notes,photo) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10) ON CONFLICT(id) DO UPDATE SET name=excluded.name,club=excluded.club,birth_year=excluded.birth_year,city=excluded.city,country=excluded.country,email=excluded.email,phone=excluded.phone,notes=excluded.notes,photo=excluded.photo",params![player.id.to_string(),player.name,player.club,player.profile.birth_year,player.profile.city,player.profile.country,player.profile.email,player.profile.phone,player.profile.notes,player.profile.photo]).map_err(|_|ApplicationError::Storage)?;
    Ok(())
}
type PublicEntityHistory = HashMap<(String, Uuid), (i64, bool, serde_json::Value)>;

// Compare all known public entities, including persistent withdrawal markers.
fn validate_source_history(
    connection: &Connection,
    next: &RegistrySnapshot,
) -> Result<(), ApplicationError> {
    let old: Option<String> = connection.query_row("SELECT CASE WHEN length(payload)<=33554432 THEN payload END FROM registry_sources WHERE registry_id=?1", [next.registry_id.to_string()], |row| row.get(0)).optional().map_err(|_| ApplicationError::Storage)?;
    let Some(old) = old else {
        return Ok(());
    };
    let old = parse_snapshot(old.as_bytes()).map_err(|_| ApplicationError::Storage)?;
    let entities = |snapshot: &RegistrySnapshot| -> Result<PublicEntityHistory, ApplicationError> {
        let mut result = HashMap::new();
        for (kind, id, revision, value) in snapshot
            .players
            .iter()
            .map(|p| ("player", p.id, &p.revision, serde_json::to_value(p)))
            .chain(
                snapshot
                    .clubs
                    .iter()
                    .map(|p| ("club", p.id, &p.revision, serde_json::to_value(p))),
            )
            .chain(
                snapshot
                    .media
                    .iter()
                    .map(|p| ("media", p.id, &p.revision, serde_json::to_value(p))),
            )
        {
            result.insert(
                (kind.into(), id),
                (
                    counter(revision)?,
                    false,
                    value.map_err(|_| ApplicationError::Storage)?,
                ),
            );
        }
        for p in &snapshot.tombstones {
            result.insert(
                (p.entity_type.clone(), p.entity_id),
                (
                    counter(&p.revision)?,
                    true,
                    serde_json::to_value(p).map_err(|_| ApplicationError::Storage)?,
                ),
            );
        }
        Ok(result)
    };
    let next_entities = entities(next)?;
    for (id, (revision, withdrawn, value)) in entities(&old)? {
        let Some((new_revision, new_withdrawn, new_value)) = next_entities.get(&id) else {
            return Err(ApplicationError::RegistryConflict);
        };
        if *new_revision < revision
            || (*new_revision == revision && (withdrawn != *new_withdrawn || value != *new_value))
        {
            return Err(ApplicationError::RegistryConflict);
        }
    }
    Ok(())
}
impl RegistryRepository for SqliteTournamentRepository {
    fn stage_registry(
        &mut self,
        payload: &str,
        source_url: Option<String>,
        decisions: &[RegistryDecision],
    ) -> Result<RegistryPreview, ApplicationError> {
        let snapshot = parse_snapshot(payload.as_bytes())?;
        let digest = semantic_digest(payload)?;
        validate_source_history(&self.connection, &snapshot)?;
        let source = source_on(&self.connection, snapshot.registry_id)?;
        if let Some((old_checkpoint, old_digest)) = &source {
            if counter(&snapshot.checkpoint)? < counter(old_checkpoint)?
                || (old_checkpoint == &snapshot.checkpoint && old_digest != &digest)
            {
                return Err(ApplicationError::RegistryConflict);
            }
        }
        let decisions: HashMap<_, _> = decisions.iter().map(|d| (d.remote_id, d)).collect();
        let ids: HashSet<_> = snapshot.players.iter().map(|p| p.id).collect();
        if decisions.keys().any(|id| !ids.contains(id)) {
            return Err(ApplicationError::InvalidRegistry);
        }
        let clubs: HashMap<_, _> = snapshot
            .clubs
            .iter()
            .map(|club| (club.id, &club.name))
            .collect();
        let mut links_by_player: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        for membership in &snapshot.memberships {
            links_by_player
                .entry(membership.player_id)
                .or_default()
                .push(membership.club_id);
        }
        let mut rows = vec![];
        let mut local_ids = HashSet::new();
        for remote in &snapshot.players {
            let decision = decisions.get(&remote.id).copied();
            let link:Option<(Option<String>,String,String,String,String)>=self.connection.query_row("SELECT local_id,overrides,upstream_state,remote_revision,last_remote FROM registry_links WHERE registry_id=?1 AND remote_id=?2",params![snapshot.registry_id.to_string(),remote.id.to_string()],|r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?,r.get(4)?))).optional().map_err(|_|ApplicationError::Storage)?;
            if let Some((_, _, state, old_revision, last)) = &link {
                if counter(&remote.revision)? < counter(old_revision)?
                    || (state == "withdrawn"
                        && counter(&remote.revision)? <= counter(old_revision)?)
                {
                    return Err(ApplicationError::RegistryConflict);
                }
                if counter(&remote.revision)? == counter(old_revision)?
                    && serde_json::to_value(remote).map_err(|_| ApplicationError::Storage)?
                        != serde_json::from_str::<serde_json::Value>(last)
                            .map_err(|_| ApplicationError::Storage)?
                {
                    return Err(ApplicationError::RegistryConflict);
                }
            }
            let pinned = link
                .as_ref()
                .and_then(|v| v.0.as_ref())
                .map(|id| Uuid::parse_str(id).map_err(|_| ApplicationError::Storage))
                .transpose()?;
            let local_id = decision
                .and_then(|d| d.local_id)
                .or(pinned)
                .unwrap_or_else(Uuid::new_v4);
            if !local_ids.insert(local_id) {
                return Err(ApplicationError::RegistryConflict);
            }
            let current = player_on(&self.connection, local_id)?;
            if decision.is_some_and(|d| d.local_id.is_some()) && current.is_none() {
                return Err(ApplicationError::NotFound);
            }
            let other:bool=self.connection.query_row("SELECT EXISTS(SELECT 1 FROM registry_links WHERE local_id=?1 AND NOT(registry_id=?2 AND remote_id=?3))",params![local_id.to_string(),snapshot.registry_id.to_string(),remote.id.to_string()],|r|r.get(0)).map_err(|_|ApplicationError::Storage)?;
            if other {
                return Err(ApplicationError::RegistryConflict);
            }
            let mut overrides: HashMap<String, bool> = link
                .as_ref()
                .map(|v| serde_json::from_str(&v.1).map_err(|_| ApplicationError::Storage))
                .transpose()?
                .unwrap_or_default();
            if pinned != Some(local_id) && current.is_some() {
                for field in ["name", "club", "birth_year", "country", "photo"] {
                    overrides.insert(field.into(), true);
                }
            }
            let mut remote_clubs: Vec<_> =
                links_by_player.get(&remote.id).cloned().unwrap_or_default();
            remote_clubs.sort();
            let club = if remote_clubs.is_empty() {
                current.as_ref().map(|p| p.club.clone()).unwrap_or_default()
            } else {
                remote_clubs
                    .iter()
                    .map(|id| clubs[id].as_str())
                    .collect::<Vec<_>>()
                    .join(" / ")
            };
            let (candidate, overrides, mut conflicts) = merge_player(
                current.as_ref(),
                remote,
                &club,
                overrides,
                decision,
                local_id,
            )?;
            if remote.photo_id.is_some()
                && current.as_ref().is_some_and(|p| p.profile.photo.is_some())
                && decision.and_then(|d| d.photo.as_ref()).is_none()
            {
                conflicts.push("photo".into());
            }
            rows.push(RegistryPreviewRow {
                remote_id: remote.id,
                local_id,
                revision: remote.revision.clone(),
                source_name: remote.display_name.clone(),
                current: current.as_ref().map(RegistryFields::from),
                proposed: RegistryFields::from(&candidate),
                conflicts,
                overrides,
                skip: decision.map_or(link.as_ref().is_some_and(|l| l.0.is_none()), |d| d.skip),
                valid: validate_imported_player(&candidate).is_ok(),
                expected_hash: current.as_ref().map(fingerprint).transpose()?,
                remote: remote.clone(),
                remote_clubs,
                photo: decision.and_then(|d| d.photo.clone()),
            });
        }
        let preview = RegistryPreview {
            id: Uuid::new_v4(),
            registry_id: snapshot.registry_id,
            registry_name: snapshot.registry_name,
            checkpoint: snapshot.checkpoint,
            source_url,
            expected_checkpoint: source.as_ref().map(|v| v.0.clone()),
            expected_source_hash: source.map(|v| v.1),
            policy: snapshot.publication_policy,
            rows,
            withdrawals: snapshot
                .tombstones
                .iter()
                .filter(|t| t.entity_type == "player")
                .count(),
        };
        let encoded = serde_json::to_string(&preview).map_err(|_| ApplicationError::Storage)?;
        if encoded.len() > MAX_SNAPSHOT_BYTES {
            return Err(ApplicationError::InvalidRegistry);
        }
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        tx.execute(
            "DELETE FROM registry_jobs WHERE created_at < unixepoch()-3600",
            [],
        )
        .map_err(|_| ApplicationError::Storage)?;
        let count: i64 = tx
            .query_row("SELECT count(*) FROM registry_jobs", [], |r| r.get(0))
            .map_err(|_| ApplicationError::Storage)?;
        if count >= 3 {
            return Err(ApplicationError::RegistryConflict);
        }
        tx.execute(
            "INSERT INTO registry_jobs(id,created_at,payload,preview) VALUES(?1,unixepoch(),?2,?3)",
            params![preview.id.to_string(), payload, encoded],
        )
        .map_err(|_| ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)?;
        Ok(preview)
    }

    fn confirm_registry(&mut self, id: Uuid) -> Result<RegistryImportResult, ApplicationError> {
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|_| ApplicationError::Storage)?;
        if let Some(result) = tx
            .query_row(
                "SELECT result FROM registry_receipts WHERE id=?1",
                [id.to_string()],
                |r| r.get::<_, String>(0),
            )
            .optional()
            .map_err(|_| ApplicationError::Storage)?
        {
            return serde_json::from_str(&result).map_err(|_| ApplicationError::Storage);
        }
        let (payload,encoded):(String,String)=tx.query_row("SELECT CASE WHEN length(payload)<=33554432 THEN payload END,CASE WHEN length(preview)<=33554432 THEN preview END FROM registry_jobs WHERE id=?1 AND created_at >= unixepoch()-3600 AND created_at<=unixepoch()",[id.to_string()],|r|Ok((r.get(0)?,r.get(1)?))).optional().map_err(|_|ApplicationError::Storage)?.ok_or(ApplicationError::RegistryConflict)?;
        let preview: RegistryPreview =
            serde_json::from_str(&encoded).map_err(|_| ApplicationError::Storage)?;
        let snapshot = parse_snapshot(payload.as_bytes())?;
        let source = source_on(&tx, preview.registry_id)?;
        if source.as_ref().map(|s| &s.0) != preview.expected_checkpoint.as_ref()
            || source.as_ref().map(|s| &s.1) != preview.expected_source_hash.as_ref()
            || preview.registry_id != snapshot.registry_id
            || preview.checkpoint != snapshot.checkpoint
        {
            return Err(ApplicationError::RegistryConflict);
        }
        validate_source_history(&tx, &snapshot)?;
        let remote: HashMap<_, _> = snapshot.players.iter().map(|p| (p.id, p)).collect();
        let mut remote_ids = HashSet::new();
        let mut local_ids = HashSet::new();
        let mut memberships: HashMap<Uuid, HashSet<Uuid>> = HashMap::new();
        for link in &snapshot.memberships {
            memberships
                .entry(link.player_id)
                .or_default()
                .insert(link.club_id);
        }
        for row in &preview.rows {
            if !remote_ids.insert(row.remote_id)
                || !local_ids.insert(row.local_id)
                || remote.get(&row.remote_id).is_none_or(|p| {
                    serde_json::to_value(*p).ok() != serde_json::to_value(&row.remote).ok()
                })
                || row.revision != row.remote.revision
            {
                return Err(ApplicationError::RegistryConflict);
            }
            let expected = memberships.get(&row.remote_id).cloned().unwrap_or_default();
            if expected.len() != row.remote_clubs.len()
                || row.remote_clubs.iter().any(|id| !expected.contains(id))
            {
                return Err(ApplicationError::RegistryConflict);
            }
        }
        if remote_ids.len() != remote.len() {
            return Err(ApplicationError::RegistryConflict);
        }
        let digest = semantic_digest(&payload)?;
        let mut prepared = vec![];
        for row in &preview.rows {
            if row.skip {
                continue;
            }
            let current = player_on(&tx, row.local_id)?;
            if current.as_ref().map(fingerprint).transpose()? != row.expected_hash {
                return Err(ApplicationError::RegistryConflict);
            }
            let mut player = current.clone().unwrap_or_else(|| Player {
                id: row.local_id,
                name: String::new(),
                club: String::new(),
                profile: PlayerProfile::default(),
            });
            player.name = row.proposed.name.clone();
            player.club = row.proposed.club.clone();
            player.profile.birth_year = row.proposed.birth_year;
            player.profile.country = row.proposed.country.clone();
            if let Some(photo) = &row.photo {
                player.profile.photo = Some(photo.clone());
            }
            validate_imported_player(&player)?;
            prepared.push((row, current, player));
        }
        tx.execute("INSERT INTO registry_sources(registry_id,name,checkpoint,source_url,payload,semantic_hash) VALUES(?1,?2,?3,?4,?5,?6) ON CONFLICT(registry_id) DO UPDATE SET name=excluded.name,checkpoint=excluded.checkpoint,source_url=COALESCE(excluded.source_url,registry_sources.source_url),payload=excluded.payload,semantic_hash=excluded.semantic_hash",params![snapshot.registry_id.to_string(),snapshot.registry_name,snapshot.checkpoint,preview.source_url,payload,digest]).map_err(|_|ApplicationError::Storage)?;
        tx.execute(
            "UPDATE registry_import_context SET active=1 WHERE singleton=1",
            [],
        )
        .map_err(|_| ApplicationError::Storage)?;
        for club in &snapshot.clubs {
            tx.execute("INSERT INTO registry_clubs(registry_id,remote_id,local_id,name,revision,payload,upstream_state) VALUES(?1,?2,?3,?4,?5,?6,'live') ON CONFLICT(registry_id,remote_id) DO UPDATE SET name=excluded.name,revision=excluded.revision,payload=excluded.payload,upstream_state='live'",params![snapshot.registry_id.to_string(),club.id.to_string(),Uuid::new_v4().to_string(),club.name,club.revision,serde_json::to_string(club).map_err(|_|ApplicationError::Storage)?]).map_err(|_|ApplicationError::Storage)?;
        }
        let mut count = 0;
        for (row, current, player) in prepared {
            if current.as_ref() != Some(&player) {
                write_player(&tx, &player)?;
                count += 1;
            }
            tx.execute("INSERT INTO registry_links(registry_id,remote_id,local_id,remote_revision,last_remote,overrides,upstream_state) VALUES(?1,?2,?3,?4,?5,?6,'live') ON CONFLICT(registry_id,remote_id) DO UPDATE SET local_id=excluded.local_id,remote_revision=excluded.remote_revision,last_remote=excluded.last_remote,overrides=excluded.overrides,upstream_state='live'",params![snapshot.registry_id.to_string(),row.remote_id.to_string(),row.local_id.to_string(),row.revision,serde_json::to_string(&row.remote).map_err(|_|ApplicationError::Storage)?,serde_json::to_string(&row.overrides).map_err(|_|ApplicationError::Storage)?]).map_err(|_|ApplicationError::Storage)?;
            tx.execute(
                "DELETE FROM registry_memberships WHERE registry_id=?1 AND player_remote_id=?2",
                params![snapshot.registry_id.to_string(), row.remote_id.to_string()],
            )
            .map_err(|_| ApplicationError::Storage)?;
            for club in &row.remote_clubs {
                tx.execute(
                    "INSERT INTO registry_memberships VALUES(?1,?2,?3)",
                    params![
                        snapshot.registry_id.to_string(),
                        row.remote_id.to_string(),
                        club.to_string()
                    ],
                )
                .map_err(|_| ApplicationError::Storage)?;
            }
        }
        let mut withdrawn = 0;
        for tombstone in &snapshot.tombstones {
            if tombstone.entity_type == "player" {
                withdrawn+=tx.execute("UPDATE registry_links SET upstream_state='withdrawn',remote_revision=?1 WHERE registry_id=?2 AND remote_id=?3 AND upstream_state='live'",params![tombstone.revision,snapshot.registry_id.to_string(),tombstone.entity_id.to_string()]).map_err(|_|ApplicationError::Storage)?;
                tx.execute(
                    "DELETE FROM registry_memberships WHERE registry_id=?1 AND player_remote_id=?2",
                    params![
                        snapshot.registry_id.to_string(),
                        tombstone.entity_id.to_string()
                    ],
                )
                .map_err(|_| ApplicationError::Storage)?;
            } else if tombstone.entity_type == "club" {
                tx.execute("UPDATE registry_clubs SET upstream_state='withdrawn',revision=?1 WHERE registry_id=?2 AND remote_id=?3",params![tombstone.revision,snapshot.registry_id.to_string(),tombstone.entity_id.to_string()]).map_err(|_|ApplicationError::Storage)?;
                tx.execute(
                    "DELETE FROM registry_memberships WHERE registry_id=?1 AND club_remote_id=?2",
                    params![
                        snapshot.registry_id.to_string(),
                        tombstone.entity_id.to_string()
                    ],
                )
                .map_err(|_| ApplicationError::Storage)?;
            }
        }
        tx.execute(
            "UPDATE registry_import_context SET active=0 WHERE singleton=1",
            [],
        )
        .map_err(|_| ApplicationError::Storage)?;
        let result = RegistryImportResult {
            players: count,
            withdrawn,
            checkpoint: snapshot.checkpoint,
        };
        tx.execute(
            "INSERT INTO registry_receipts(id,result) VALUES(?1,?2)",
            params![
                id.to_string(),
                serde_json::to_string(&result).map_err(|_| ApplicationError::Storage)?
            ],
        )
        .map_err(|_| ApplicationError::Storage)?;
        tx.execute("DELETE FROM registry_jobs WHERE id=?1", [id.to_string()])
            .map_err(|_| ApplicationError::Storage)?;
        tx.commit().map_err(|_| ApplicationError::Storage)?;
        Ok(result)
    }
    fn registry_sources(&self) -> Result<Vec<RegistrySource>, ApplicationError> {
        let mut statement=self.connection.prepare("SELECT registry_id,name,checkpoint,source_url FROM registry_sources ORDER BY name LIMIT 100").map_err(|_|ApplicationError::Storage)?;
        let rows = statement
            .query_map([], |r| {
                Ok((r.get::<_, String>(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
            })
            .map_err(|_| ApplicationError::Storage)?;
        rows.map(|row| {
            let (id, name, checkpoint, source_url) = row.map_err(|_| ApplicationError::Storage)?;
            Ok(RegistrySource {
                registry_id: Uuid::parse_str(&id).map_err(|_| ApplicationError::Storage)?,
                name,
                checkpoint,
                source_url,
            })
        })
        .collect()
    }
    fn registry_previews(&self) -> Result<Vec<RegistryPreview>, ApplicationError> {
        let mut statement=self.connection.prepare("SELECT CASE WHEN length(preview)<=33554432 THEN preview END FROM registry_jobs WHERE created_at >= unixepoch()-3600 ORDER BY created_at LIMIT 3").map_err(|_|ApplicationError::Storage)?;
        let rows = statement
            .query_map([], |r| r.get::<_, String>(0))
            .map_err(|_| ApplicationError::Storage)?;
        rows.map(|row| {
            serde_json::from_str(&row.map_err(|_| ApplicationError::Storage)?)
                .map_err(|_| ApplicationError::Storage)
        })
        .collect()
    }
    fn refine_registry(
        &mut self,
        id: Uuid,
        decisions: &[RegistryDecision],
    ) -> Result<RegistryPreview, ApplicationError> {
        let (payload,encoded):(String,String)=self.connection.query_row("SELECT CASE WHEN length(payload)<=33554432 THEN payload END,CASE WHEN length(preview)<=33554432 THEN preview END FROM registry_jobs WHERE id=?1 AND created_at>=unixepoch()-3600",[id.to_string()],|r|Ok((r.get(0)?,r.get(1)?))).map_err(|_|ApplicationError::RegistryConflict)?;
        let old: RegistryPreview =
            serde_json::from_str(&encoded).map_err(|_| ApplicationError::Storage)?;
        let preview = preview_registry(self, &payload, old.source_url, decisions)?;
        self.cancel_registry(id)?;
        Ok(preview)
    }
    fn cancel_registry(&mut self, id: Uuid) -> Result<(), ApplicationError> {
        self.connection
            .execute("DELETE FROM registry_jobs WHERE id=?1", [id.to_string()])
            .map_err(|_| ApplicationError::Storage)?;
        Ok(())
    }
    fn registry_photo(&self, id: Uuid, remote_id: Uuid) -> Result<RegistryMedia, ApplicationError> {
        let payload:String=self.connection.query_row("SELECT CASE WHEN length(payload)<=33554432 THEN payload END FROM registry_jobs WHERE id=?1 AND created_at>=unixepoch()-3600",[id.to_string()],|r|r.get(0)).map_err(|_|ApplicationError::RegistryConflict)?;
        let snapshot = parse_snapshot(payload.as_bytes())?;
        let id = snapshot
            .players
            .iter()
            .find(|p| p.id == remote_id)
            .and_then(|p| p.photo_id)
            .ok_or(ApplicationError::NotFound)?;
        snapshot
            .media
            .into_iter()
            .find(|m| m.id == id)
            .ok_or(ApplicationError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use librett_application::{GuardedPlayerRepository, PlayerRepository};
    fn payload() -> String {
        include_str!("../../application/contracts/examples/published.json").to_owned()
    }
    fn decisions(value: &str) -> Vec<RegistryDecision> {
        parse_snapshot(value.as_bytes())
            .unwrap()
            .players
            .into_iter()
            .map(|p| RegistryDecision {
                remote_id: p.id,
                birth_year: Some(1990),
                ..Default::default()
            })
            .collect()
    }
    fn import(repo: &mut SqliteTournamentRepository, value: &str) -> RegistryPreview {
        let preview = preview_registry(repo, value, None, &decisions(value)).unwrap();
        repo.confirm_registry(preview.id).unwrap();
        preview
    }
    #[test]
    fn fresh_ids_duplicate_names_completion_and_receipts() {
        let mut repo =
            SqliteTournamentRepository::initialize(Connection::open_in_memory().unwrap()).unwrap();
        let pending = preview_registry(&mut repo, &payload(), None, &[]).unwrap();
        assert!(!pending.rows[1].valid);
        assert_eq!(
            repo.confirm_registry(pending.id),
            Err(ApplicationError::BirthYearRequired)
        );
        assert_eq!(repo.list_players().unwrap().len(), 0);
        repo.cancel_registry(pending.id).unwrap();
        let preview = import(&mut repo, &payload());
        assert_eq!(repo.list_players().unwrap().len(), 2);
        for row in &preview.rows {
            assert_ne!(row.remote_id, row.local_id);
        }
        let first = repo.confirm_registry(preview.id).unwrap();
        assert_eq!(first.players, 2);
        let again = preview_registry(
            &mut repo,
            &payload(),
            Some("https://moved.example.invalid/snapshot".into()),
            &[],
        )
        .unwrap();
        assert_eq!(repo.confirm_registry(again.id).unwrap().players, 0);
        assert_eq!(repo.list_players().unwrap().len(), 2);
        assert_eq!(
            repo.registry_sources().unwrap()[0].source_url.as_deref(),
            Some("https://moved.example.invalid/snapshot")
        );
    }
    #[test]
    fn manual_edits_and_historical_registration_snapshots_survive_refresh_and_withdrawal() {
        let mut repo =
            SqliteTournamentRepository::initialize(Connection::open_in_memory().unwrap()).unwrap();
        let first = import(&mut repo, &payload());
        let row = &first.rows[0];
        let old = repo.find_player(row.local_id).unwrap();
        let mut edited = old.clone();
        edited.name = "Local override".into();
        edited.profile.notes = "Private note".into();
        repo.save_player_checked(Uuid::new_v4(), &edited, Some(&old))
            .unwrap();
        let mut value: serde_json::Value = serde_json::from_str(&payload()).unwrap();
        value["checkpoint"] = "5".into();
        value["players"][0]["revision"] = "2".into();
        value["players"][0]["display_name"] = "Changed remote name".into();
        let newer = serde_json::to_string(&value).unwrap();
        let preview = preview_registry(&mut repo, &newer, None, &[]).unwrap();
        assert!(preview.rows[0].conflicts.contains(&"name".into()));
        repo.confirm_registry(preview.id).unwrap();
        assert_eq!(
            repo.find_player(row.local_id).unwrap().name,
            "Local override"
        );
        assert_eq!(
            repo.find_player(row.local_id).unwrap().profile.notes,
            "Private note"
        );
        let stale = preview_registry(&mut repo, &payload(), None, &[]);
        assert!(matches!(stale, Err(ApplicationError::RegistryConflict)));
        let mut withdrawn = value.clone();
        withdrawn["checkpoint"] = "7".into();
        withdrawn["players"].as_array_mut().unwrap().remove(0);
        withdrawn["memberships"] = serde_json::json!([]);
        withdrawn["media"] = serde_json::json!([]);
        withdrawn["tombstones"] = serde_json::json!([{"entity_type":"player","entity_id":row.remote_id.to_string(),"revision":"3","removed_at_checkpoint":"6"},{"entity_type":"media","entity_id":"55555555-5555-4555-8555-555555555555","revision":"2","removed_at_checkpoint":"7"}]);
        let preview = preview_registry(&mut repo, &withdrawn.to_string(), None, &[]).unwrap();
        repo.confirm_registry(preview.id).unwrap();
        assert_eq!(
            repo.find_player(row.local_id).unwrap().name,
            "Local override"
        );
        let state: String = repo
            .connection
            .query_row(
                "SELECT upstream_state FROM registry_links WHERE local_id=?1",
                [row.local_id.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(state, "withdrawn");
    }
    #[test]
    fn stale_preview_and_failure_roll_back_source_cursor() {
        let mut repo =
            SqliteTournamentRepository::initialize(Connection::open_in_memory().unwrap()).unwrap();
        let first = import(&mut repo, &payload());
        let before = repo.registry_sources().unwrap()[0].checkpoint.clone();
        let mut value: serde_json::Value = serde_json::from_str(&payload()).unwrap();
        value["checkpoint"] = "5".into();
        value["players"][0]["revision"] = "2".into();
        value["players"][0]["display_name"] = "Updated name".into();
        let preview = preview_registry(&mut repo, &value.to_string(), None, &[]).unwrap();
        let mut local = repo.find_player(first.rows[0].local_id).unwrap();
        let old = local.clone();
        local.profile.country = "Local edit".into();
        repo.save_player_checked(Uuid::new_v4(), &local, Some(&old))
            .unwrap();
        assert_eq!(
            repo.confirm_registry(preview.id),
            Err(ApplicationError::RegistryConflict)
        );
        assert_eq!(repo.registry_sources().unwrap()[0].checkpoint, before);
        repo.cancel_registry(preview.id).unwrap();
        let preview = preview_registry(&mut repo, &value.to_string(), None, &[]).unwrap();
        repo.connection.execute_batch("CREATE TRIGGER fail_registry_receipt BEFORE INSERT ON registry_receipts BEGIN SELECT RAISE(ABORT,'synthetic failure'); END;").unwrap();
        assert_eq!(
            repo.confirm_registry(preview.id),
            Err(ApplicationError::Storage)
        );
        assert_eq!(repo.registry_sources().unwrap()[0].checkpoint, before);
        let active: i64 = repo
            .connection
            .query_row("SELECT active FROM registry_import_context", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(active, 0);
    }
    #[test]
    fn explicit_link_keeps_manual_fields_and_local_deletion_is_not_recreated() {
        let mut repo =
            SqliteTournamentRepository::initialize(Connection::open_in_memory().unwrap()).unwrap();
        let mut manual = Player::new("Manual player", "Manual club").unwrap();
        manual.profile.birth_year = Some(1980);
        repo.insert_player(&manual).unwrap();
        let source = parse_snapshot(payload().as_bytes()).unwrap().players[0].id;
        let mut choices = decisions(&payload());
        choices[0].local_id = Some(manual.id);
        choices[0].birth_year = None;
        let preview = preview_registry(&mut repo, &payload(), None, &choices).unwrap();
        repo.confirm_registry(preview.id).unwrap();
        assert_eq!(repo.find_player(manual.id).unwrap().name, "Manual player");
        repo.delete_player(manual.id).unwrap();
        let preview = preview_registry(&mut repo, &payload(), None, &[]).unwrap();
        assert!(preview.rows[0].skip);
        repo.confirm_registry(preview.id).unwrap();
        assert_eq!(repo.list_players().unwrap().len(), 1);
        let detached: Option<String> = repo
            .connection
            .query_row(
                "SELECT local_id FROM registry_links WHERE remote_id=?1",
                [source.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(detached, None);
    }
    #[test]
    fn importer_never_updates_entry_member_snapshots() {
        let mut repo =
            SqliteTournamentRepository::initialize(Connection::open_in_memory().unwrap()).unwrap();
        let preview = import(&mut repo, &payload());
        let player = repo.find_player(preview.rows[0].local_id).unwrap();
        let tournament =
            librett_application::create_tournament(&mut repo, "Synthetic tournament").unwrap();
        let tournament = librett_application::add_category(
            &mut repo,
            tournament.id,
            "Synthetic category",
            Discipline::Singles,
            CompetitionFormat::Knockout,
        )
        .unwrap();
        let category = tournament.categories[0].id;
        let entry = librett_application::register_entry(
            &mut repo,
            tournament.id,
            category,
            vec![player.id],
        )
        .unwrap();
        let mut value: serde_json::Value = serde_json::from_str(&payload()).unwrap();
        value["checkpoint"] = "5".into();
        value["players"][0]["revision"] = "2".into();
        value["players"][0]["display_name"] = "Updated public name".into();
        let next = preview_registry(&mut repo, &value.to_string(), None, &[]).unwrap();
        repo.confirm_registry(next.id).unwrap();
        assert_eq!(
            repo.list_entries(category).unwrap()[0].members[0].name,
            entry.members[0].name
        );
        assert_eq!(
            repo.find_player(player.id).unwrap().name,
            "Updated public name"
        );
    }
    #[test]
    fn wordpress_export_import_restart_and_backup_restore() {
        let directory =
            std::env::temp_dir().join(format!("librett-registry-roundtrip-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join("live.sqlite");
        let payload =
            include_str!("../../application/contracts/examples/valid-wordpress-roundtrip.json");
        let mut repo = SqliteTournamentRepository::open(&path).unwrap();
        let choices: Vec<_> = parse_snapshot(payload.as_bytes())
            .unwrap()
            .players
            .iter()
            .filter(|p| p.birth_year.is_none())
            .map(|p| RegistryDecision {
                remote_id: p.id,
                birth_year: Some(1985),
                ..Default::default()
            })
            .collect();
        let preview = preview_registry(&mut repo, payload, None, &choices).unwrap();
        assert_eq!(repo.confirm_registry(preview.id).unwrap().players, 2);
        assert!(repo
            .list_players()
            .unwrap()
            .iter()
            .all(|p| p.club == "Roundtrip club" && p.profile.birth_year.is_some()));
        let backup = repo.create_backup(&directory, "registry-test").unwrap();
        let bytes = std::fs::read(directory.join(backup.name)).unwrap();
        drop(repo);
        let mut repo = SqliteTournamentRepository::open(&path).unwrap();
        assert_eq!(repo.registry_sources().unwrap().len(), 1);
        repo.import_backup(&directory, &bytes).unwrap();
        assert_eq!(repo.list_players().unwrap().len(), 2);
        assert_eq!(repo.registry_sources().unwrap().len(), 1);
        let next = preview_registry(&mut repo, payload, None, &[]).unwrap();
        assert_eq!(repo.confirm_registry(next.id).unwrap().players, 0);
        drop(repo);
        std::fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn higher_checkpoint_does_not_allow_regressed_clubs_or_lost_withdrawals() {
        let mut repo =
            SqliteTournamentRepository::initialize(Connection::open_in_memory().unwrap()).unwrap();
        import(&mut repo, &payload());
        let mut value: serde_json::Value = serde_json::from_str(&payload()).unwrap();
        value["checkpoint"] = "9".into();
        value["clubs"][0]["name"] = "Conflicting same-revision club".into();
        assert!(matches!(
            preview_registry(&mut repo, &value.to_string(), None, &[]),
            Err(ApplicationError::RegistryConflict)
        ));
        let mut value: serde_json::Value = serde_json::from_str(&payload()).unwrap();
        value["checkpoint"] = "9".into();
        value["tombstones"] = serde_json::json!([{"entity_type":"player","entity_id":"55555555-5555-4555-8555-555555555555","revision":"2","removed_at_checkpoint":"8"}]);
        let accepted = preview_registry(&mut repo, &value.to_string(), None, &[]).unwrap();
        repo.confirm_registry(accepted.id).unwrap();
        value["checkpoint"] = "10".into();
        value["tombstones"] = serde_json::json!([]);
        assert!(matches!(
            preview_registry(&mut repo, &value.to_string(), None, &[]),
            Err(ApplicationError::RegistryConflict)
        ));
    }
}
