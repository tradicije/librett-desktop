// Copyright (C) 2026 Aleksa Dimitrijević. AGPL-3.0-or-later.
use crate::ApplicationError;
use librett_domain::{Player, PlayerProfile};
use serde::{
    de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor},
    Deserialize, Serialize,
};
use serde_json::{Map, Value};
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    fmt,
    rc::Rc,
};
use uuid::Uuid;

pub const MAX_SNAPSHOT_BYTES: usize = 32 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySnapshot {
    pub registry_id: Uuid,
    pub registry_name: String,
    pub checkpoint: String,
    pub publication_policy: Value,
    pub players: Vec<RegistryPlayer>,
    pub clubs: Vec<RegistryClub>,
    pub memberships: Vec<RegistryMembership>,
    pub media: Vec<RegistryMedia>,
    pub tombstones: Vec<RegistryTombstone>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPlayer {
    pub id: Uuid,
    pub revision: String,
    pub slug: String,
    pub display_name: String,
    pub birth_year: Option<u16>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub biography: Option<String>,
    pub photo_id: Option<Uuid>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryClub {
    pub id: Uuid,
    pub revision: String,
    pub slug: String,
    pub name: String,
    pub country: Option<String>,
    pub region: Option<String>,
    pub abbreviation: Option<String>,
    pub aliases: Option<Vec<String>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryMembership {
    pub player_id: Uuid,
    pub club_id: Uuid,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryMedia {
    pub id: Uuid,
    pub revision: String,
    pub content_sha256: String,
    pub mime_type: String,
    pub byte_length: u32,
    pub width: u32,
    pub height: u32,
    pub content_url: String,
    pub attribution: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryTombstone {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub revision: String,
    pub removed_at_checkpoint: String,
}

pub fn counter(value: &str) -> Result<i64, ApplicationError> {
    let number = value
        .parse::<i64>()
        .map_err(|_| ApplicationError::InvalidRegistry)?;
    if number < 0 || number.to_string() != value {
        return Err(ApplicationError::InvalidRegistry);
    }
    Ok(number)
}
pub fn https_url(value: &str) -> Result<url::Url, ApplicationError> {
    let parsed = url::Url::parse(value).map_err(|_| ApplicationError::InvalidRegistry)?;
    if value.len() > 2048
        || value.chars().any(char::is_whitespace)
        || parsed.scheme() != "https"
        || parsed.host_str().is_none()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.fragment().is_some()
    {
        return Err(ApplicationError::InvalidRegistry);
    }
    Ok(parsed)
}

// This is a Serde visitor, not a JSON lexer/parser. serde_json validates syntax and Unicode.
struct BoundedValue {
    depth: u8,
    items: Rc<Cell<usize>>,
}
impl<'de> DeserializeSeed<'de> for BoundedValue {
    type Value = Value;
    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        deserializer.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for BoundedValue {
    type Value = Value;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bounded JSON")
    }
    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }
    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Value, E> {
        Ok(value.into())
    }
    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Value, E> {
        Ok(value.into())
    }
    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Value, E> {
        serde_json::Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("nonfinite number"))
    }
    fn visit_unit<E: de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_str<E: de::Error>(self, value: &str) -> Result<Value, E> {
        self.visit_string(value.to_owned())
    }
    fn visit_string<E: de::Error>(self, value: String) -> Result<Value, E> {
        if value.len() > 32768 {
            return Err(E::custom("token limit"));
        }
        Ok(Value::String(value))
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut input: A) -> Result<Value, A::Error> {
        if self.depth > 8 {
            return Err(de::Error::custom("depth limit"));
        }
        let mut result = Vec::new();
        while let Some(value) = input.next_element_seed(BoundedValue {
            depth: self.depth + 1,
            items: self.items.clone(),
        })? {
            let count = self.items.get() + 1;
            self.items.set(count);
            if count > 100000 {
                return Err(de::Error::custom("item limit"));
            }
            result.push(value);
        }
        Ok(Value::Array(result))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut input: A) -> Result<Value, A::Error> {
        if self.depth > 8 {
            return Err(de::Error::custom("depth limit"));
        }
        let mut result = Map::new();
        while let Some(key) = input.next_key::<String>()? {
            if result.len() >= 32 || key.len() > 32768 || result.contains_key(&key) {
                return Err(de::Error::custom("duplicate key"));
            }
            result.insert(
                key,
                input.next_value_seed(BoundedValue {
                    depth: self.depth + 1,
                    items: self.items.clone(),
                })?,
            );
        }
        Ok(Value::Object(result))
    }
}

pub fn parse_snapshot(bytes: &[u8]) -> Result<RegistrySnapshot, ApplicationError> {
    if bytes.len() > MAX_SNAPSHOT_BYTES {
        return Err(ApplicationError::InvalidRegistry);
    }
    // Bound encoded string tokens before either maintained JSON parser allocates them.
    let (mut inside, mut escaped, mut size) = (false, false, 0usize);
    for byte in bytes {
        if !inside {
            if *byte == b'"' {
                inside = true;
                size = 0;
            }
            continue;
        }
        if !escaped && *byte == b'"' {
            inside = false;
            continue;
        }
        size += 1;
        if size > 32768 {
            return Err(ApplicationError::InvalidRegistry);
        }
        escaped = !escaped && *byte == b'\\';
    }
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = BoundedValue {
        depth: 1,
        items: Rc::new(Cell::new(0)),
    }
    .deserialize(&mut deserializer)
    .map_err(|_| ApplicationError::InvalidRegistry)?;
    deserializer
        .end()
        .map_err(|_| ApplicationError::InvalidRegistry)?;
    let schema: Value =
        serde_json::from_str(include_str!("../contracts/public-snapshot-v1.schema.json"))
            .map_err(|_| ApplicationError::Storage)?;
    let validator = jsonschema::options()
        .with_draft(jsonschema::Draft::Draft202012)
        .should_validate_formats(true)
        .build(&schema)
        .map_err(|_| ApplicationError::Storage)?;
    if !validator.is_valid(&value) {
        return Err(ApplicationError::InvalidRegistry);
    }
    let stamp = value["exported_at"]
        .as_str()
        .ok_or(ApplicationError::InvalidRegistry)?;
    let date = chrono::NaiveDateTime::parse_from_str(stamp, "%Y-%m-%dT%H:%M:%SZ")
        .map_err(|_| ApplicationError::InvalidRegistry)?;
    if date.format("%Y-%m-%dT%H:%M:%SZ").to_string() != stamp {
        return Err(ApplicationError::InvalidRegistry);
    }
    let snapshot: RegistrySnapshot =
        serde_json::from_value(value).map_err(|_| ApplicationError::InvalidRegistry)?;
    validate_graph(&snapshot)?;
    Ok(snapshot)
}
fn validate_graph(data: &RegistrySnapshot) -> Result<(), ApplicationError> {
    if data.players.len() > 20000
        || data.clubs.len() > 5000
        || data.media.len() > 20000
        || data.memberships.len() > 40000
        || data.tombstones.len() > 50000
    {
        return Err(ApplicationError::InvalidRegistry);
    }
    for name in ["dataset_terms_url", "media_terms_url"] {
        https_url(
            data.publication_policy[name]
                .as_str()
                .ok_or(ApplicationError::InvalidRegistry)?,
        )?;
    }
    let checkpoint = counter(&data.checkpoint)?;
    let mut ids: HashSet<(String, Uuid)> = HashSet::new();
    let mut slugs = HashSet::new();
    for (kind, id, revision, slug) in data
        .players
        .iter()
        .map(|p| ("player", p.id, &p.revision, Some(&p.slug)))
        .chain(
            data.clubs
                .iter()
                .map(|p| ("club", p.id, &p.revision, Some(&p.slug))),
        )
        .chain(
            data.media
                .iter()
                .map(|p| ("media", p.id, &p.revision, None)),
        )
    {
        if !ids.insert((kind.into(), id))
            || counter(revision)? > checkpoint
            || slug.is_some_and(|s| !slugs.insert((kind, s)))
        {
            return Err(ApplicationError::InvalidRegistry);
        }
    }
    let mut pairs = HashSet::new();
    for link in &data.memberships {
        if !pairs.insert((link.player_id, link.club_id))
            || !ids.contains(&("player".into(), link.player_id))
            || !ids.contains(&("club".into(), link.club_id))
        {
            return Err(ApplicationError::InvalidRegistry);
        }
    }
    let mut photos = HashSet::new();
    for player in &data.players {
        if let Some(photo) = player.photo_id {
            if !ids.contains(&("media".into(), photo)) {
                return Err(ApplicationError::InvalidRegistry);
            }
            photos.insert(photo);
        }
    }
    if photos.len() != data.media.len() {
        return Err(ApplicationError::InvalidRegistry);
    }
    for photo in &data.media {
        https_url(&photo.content_url)?;
        if u64::from(photo.width) * u64::from(photo.height) > 16777216 {
            return Err(ApplicationError::InvalidRegistry);
        }
    }
    let mut tombstones = HashSet::new();
    for tombstone in &data.tombstones {
        if !tombstones.insert((&tombstone.entity_type, tombstone.entity_id))
            || ids.contains(&(tombstone.entity_type.clone(), tombstone.entity_id))
            || counter(&tombstone.removed_at_checkpoint)? > checkpoint
            || counter(&tombstone.revision)? > counter(&tombstone.removed_at_checkpoint)?
        {
            return Err(ApplicationError::InvalidRegistry);
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryDecision {
    pub remote_id: Uuid,
    pub local_id: Option<Uuid>,
    pub birth_year: Option<u16>,
    pub name: Option<String>,
    pub club: Option<String>,
    #[serde(default)]
    pub use_registry: Vec<String>,
    #[serde(default)]
    pub skip: bool,
    pub photo: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryFields {
    pub name: String,
    pub club: String,
    pub birth_year: Option<u16>,
    pub country: String,
}
impl From<&Player> for RegistryFields {
    fn from(p: &Player) -> Self {
        Self {
            name: p.name.clone(),
            club: p.club.clone(),
            birth_year: p.profile.birth_year,
            country: p.profile.country.clone(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPreviewRow {
    pub remote_id: Uuid,
    pub local_id: Uuid,
    pub revision: String,
    pub source_name: String,
    pub current: Option<RegistryFields>,
    pub proposed: RegistryFields,
    pub conflicts: Vec<String>,
    pub overrides: HashMap<String, bool>,
    pub skip: bool,
    pub valid: bool,
    pub expected_hash: Option<String>,
    pub remote: RegistryPlayer,
    pub remote_clubs: Vec<Uuid>,
    pub photo: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPreview {
    pub id: Uuid,
    pub registry_id: Uuid,
    pub registry_name: String,
    pub checkpoint: String,
    pub source_url: Option<String>,
    pub expected_checkpoint: Option<String>,
    pub expected_source_hash: Option<String>,
    pub policy: Value,
    pub rows: Vec<RegistryPreviewRow>,
    pub withdrawals: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySource {
    pub registry_id: Uuid,
    pub name: String,
    pub checkpoint: String,
    pub source_url: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegistryImportResult {
    pub players: usize,
    pub withdrawn: usize,
    pub checkpoint: String,
}

pub trait RegistryRepository {
    fn stage_registry(
        &mut self,
        payload: &str,
        source_url: Option<String>,
        decisions: &[RegistryDecision],
    ) -> Result<RegistryPreview, ApplicationError>;
    fn confirm_registry(&mut self, id: Uuid) -> Result<RegistryImportResult, ApplicationError>;
    fn registry_sources(&self) -> Result<Vec<RegistrySource>, ApplicationError>;
    fn registry_previews(&self) -> Result<Vec<RegistryPreview>, ApplicationError>;
    fn refine_registry(
        &mut self,
        id: Uuid,
        decisions: &[RegistryDecision],
    ) -> Result<RegistryPreview, ApplicationError>;
    fn cancel_registry(&mut self, id: Uuid) -> Result<(), ApplicationError>;
    fn registry_photo(&self, id: Uuid, remote_id: Uuid) -> Result<RegistryMedia, ApplicationError>;
}

pub fn preview_registry(
    repository: &mut impl RegistryRepository,
    payload: &str,
    source_url: Option<String>,
    decisions: &[RegistryDecision],
) -> Result<RegistryPreview, ApplicationError> {
    parse_snapshot(payload.as_bytes())?;
    if let Some(url) = &source_url {
        https_url(url)?;
    }
    if decisions.len() > 20000 {
        return Err(ApplicationError::InvalidRegistry);
    }
    let mut ids = HashSet::new();
    for decision in decisions {
        if !ids.insert(decision.remote_id)
            || decision.use_registry.iter().any(|key| {
                !["name", "club", "birth_year", "country", "photo"].contains(&key.as_str())
            })
        {
            return Err(ApplicationError::InvalidRegistry);
        }
    }
    repository.stage_registry(payload, source_url, decisions)
}

pub type MergedPlayer = (Player, HashMap<String, bool>, Vec<String>);

pub fn merge_player(
    local: Option<&Player>,
    remote: &RegistryPlayer,
    club: &str,
    mut overrides: HashMap<String, bool>,
    decision: Option<&RegistryDecision>,
    id: Uuid,
) -> Result<MergedPlayer, ApplicationError> {
    let mut candidate = local.cloned().unwrap_or_else(|| Player {
        id,
        name: remote.display_name.clone(),
        club: club.into(),
        profile: PlayerProfile::default(),
    });
    let mut conflicts = Vec::new();
    let accepted =
        |key: &str| decision.is_some_and(|d| d.use_registry.iter().any(|field| field == key));
    for (key, remote_value) in [
        ("name", Some(remote.display_name.as_str())),
        ("club", Some(club)),
        ("country", remote.country.as_deref()),
    ] {
        let Some(value) = remote_value else { continue };
        let target = match key {
            "name" => &mut candidate.name,
            "club" => &mut candidate.club,
            _ => &mut candidate.profile.country,
        };
        if overrides.get(key).copied().unwrap_or(false) && !accepted(key) {
            if *target != value {
                conflicts.push(key.into());
            }
        } else {
            *target = value.into();
            if accepted(key) {
                overrides.remove(key);
            }
        }
    }
    if let Some(year) = remote.birth_year {
        if overrides.get("birth_year").copied().unwrap_or(false) && !accepted("birth_year") {
            if candidate.profile.birth_year != Some(year) {
                conflicts.push("birth_year".into());
            }
        } else {
            candidate.profile.birth_year = Some(year);
            if accepted("birth_year") {
                overrides.remove("birth_year");
            }
        }
    }
    if let Some(decision) = decision {
        if let Some(value) = &decision.name {
            candidate.name = value.clone();
            if value != &remote.display_name {
                overrides.insert("name".into(), true);
            }
        }
        if let Some(value) = &decision.club {
            candidate.club = value.clone();
            if value != club {
                overrides.insert("club".into(), true);
            }
        }
        if let Some(year) = decision.birth_year {
            candidate.profile.birth_year = Some(year);
            if Some(year) != remote.birth_year {
                overrides.insert("birth_year".into(), true);
            }
        }
        if let Some(photo) = &decision.photo {
            librett_domain::validate_jpeg(photo, false)?;
            candidate.profile.photo = Some(photo.clone());
            overrides.remove("photo");
        }
    }
    candidate.id = id;
    Ok((candidate, overrides, conflicts))
}
pub fn validate_imported_player(player: &Player) -> Result<(), ApplicationError> {
    Player::new(&player.name, &player.club)?;
    player.profile.clone().validated()?;
    if player.profile.birth_year.is_none() {
        return Err(ApplicationError::BirthYearRequired);
    }
    Ok(())
}

pub fn fingerprint<T: Serialize>(value: &T) -> Result<String, ApplicationError> {
    use sha2::{Digest, Sha256};
    let bytes = serde_json::to_vec(value).map_err(|_| ApplicationError::Storage)?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
pub fn semantic_digest(payload: &str) -> Result<String, ApplicationError> {
    fn ordered(value: Value) -> Value {
        match value {
            Value::Array(values) => {
                let mut values: Vec<_> = values.into_iter().map(ordered).collect();
                values.sort_by_cached_key(Value::to_string);
                Value::Array(values)
            }
            Value::Object(values) => {
                Value::Object(values.into_iter().map(|(k, v)| (k, ordered(v))).collect())
            }
            value => value,
        }
    }
    let mut value: Value =
        serde_json::from_str(payload).map_err(|_| ApplicationError::InvalidRegistry)?;
    value
        .as_object_mut()
        .ok_or(ApplicationError::InvalidRegistry)?
        .remove("exported_at");
    fingerprint(&ordered(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_contract_examples_and_parser_failures() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("contracts/examples");
        for file in std::fs::read_dir(root).unwrap() {
            let file = file.unwrap();
            let name = file.file_name().to_string_lossy().into_owned();
            let bytes = std::fs::read(file.path()).unwrap();
            assert_eq!(
                parse_snapshot(&bytes).is_ok(),
                !name.starts_with("reject-"),
                "{name}"
            );
        }
        for bytes in [
            b"{} {}".as_slice(),
            b"{\"a\":1,\"\\u0061\":2}",
            b"[[[[[[[[[]]]]]]]]]",
            b"{\"x\":\"\\ud800\"}",
            b"{\"x\":1e999}",
        ] {
            assert!(parse_snapshot(bytes).is_err());
        }
        let large = format!("{{\"x\":\"{}\"}}", "a".repeat(32769));
        assert!(parse_snapshot(large.as_bytes()).is_err());
        assert_eq!(counter("9223372036854775807").unwrap(), i64::MAX);
        assert!(counter("9223372036854775808").is_err());
        assert!(counter("01").is_err());
    }
    #[test]
    fn explicit_overrides_are_sticky_even_when_values_match_and_omission_preserves_data() {
        let mut local = Player::new("Local", "Local club").unwrap();
        local.profile.birth_year = Some(1990);
        local.profile.country = "Local country".into();
        local.profile.email = "private@example.invalid".into();
        let mut remote: RegistryPlayer =
            parse_snapshot(include_bytes!("../contracts/examples/published.json"))
                .unwrap()
                .players
                .remove(0);
        remote.display_name = local.name.clone();
        remote.country = None;
        remote.birth_year = None;
        let flags = HashMap::from([("name".into(), true)]);
        let (same, flags, _) =
            merge_player(Some(&local), &remote, &local.club, flags, None, local.id).unwrap();
        assert_eq!(same, local);
        assert_eq!(flags.get("name"), Some(&true));
        remote.display_name = "New upstream".into();
        let (same, flags, conflicts) =
            merge_player(Some(&local), &remote, &local.club, flags, None, local.id).unwrap();
        assert_eq!(same.name, "Local");
        assert!(conflicts.contains(&"name".into()));
        let choice = RegistryDecision {
            remote_id: remote.id,
            use_registry: vec!["name".into()],
            ..Default::default()
        };
        let (updated, flags, _) = merge_player(
            Some(&local),
            &remote,
            &local.club,
            flags,
            Some(&choice),
            local.id,
        )
        .unwrap();
        assert_eq!(updated.name, "New upstream");
        assert!(!flags.contains_key("name"));
        assert_eq!(updated.profile.email, "private@example.invalid");
        assert_eq!(updated.profile.country, "Local country");
    }
}
