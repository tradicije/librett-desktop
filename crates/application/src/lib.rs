use librett_domain::{CompetitionFormat, Discipline, DomainError, Tournament};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationError {
    InvalidResult,
    MatchConflict,
    ResultImpact,
    PlayerConflict,
    CashConflict,
    InvalidRules,
    InvalidDraw,
    DrawConflict,
    InvalidCash,
    PlayerInUse,
    InvalidProfile,
    InvalidMembers,
    AlreadyRegistered,
    NameRequired,
    NameTooLong,
    DuplicateCategory,
    NotFound,
    Storage,
}

impl From<DomainError> for ApplicationError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::InvalidResult => Self::InvalidResult,
            DomainError::InvalidRules => Self::InvalidRules,
            DomainError::InvalidDraw => Self::InvalidDraw,
            DomainError::InvalidCash => Self::InvalidCash,
            DomainError::InvalidProfile => Self::InvalidProfile,
            DomainError::InvalidMembers => Self::InvalidMembers,
            DomainError::NameRequired => Self::NameRequired,
            DomainError::NameTooLong => Self::NameTooLong,
            DomainError::DuplicateCategory => Self::DuplicateCategory,
        }
    }
}

pub trait DrawRepository {
    fn find_draw(
        &self,
        category_id: Uuid,
    ) -> Result<Option<librett_domain::CategoryDraw>, ApplicationError>;
    fn save_draw(
        &mut self,
        draw: librett_domain::CategoryDraw,
        expected_revision: u32,
    ) -> Result<librett_domain::CategoryDraw, ApplicationError>;
}

fn draw_category(
    repository: &impl TournamentRepository,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<librett_domain::Category, ApplicationError> {
    repository
        .find(tournament_id)?
        .categories
        .into_iter()
        .find(|c| c.id == category_id && !c.archived)
        .ok_or(ApplicationError::NotFound)
}

pub fn get_category_draw(
    repository: &(impl TournamentRepository + DrawRepository),
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<Option<librett_domain::CategoryDraw>, ApplicationError> {
    draw_category(repository, tournament_id, category_id)?;
    repository.find_draw(category_id)
}

fn draw_entries(
    repository: &impl PlayerRepository,
    category_id: Uuid,
) -> Result<Vec<librett_domain::Entry>, ApplicationError> {
    let mut entries = repository.list_entries(category_id)?;
    for entry in &mut entries {
        for member in &mut entry.members {
            member.checked_in = false;
        }
    }
    Ok(entries)
}

pub fn preview_category_draw(
    repository: &(impl TournamentRepository + PlayerRepository + CategoryRulesRepository),
    tournament_id: Uuid,
    category_id: Uuid,
    mode: librett_domain::DrawMode,
    settings: librett_domain::DrawSettings,
    seeds: Vec<Uuid>,
) -> Result<librett_domain::CategoryDraw, ApplicationError> {
    let category = draw_category(repository, tournament_id, category_id)?;
    let configured = repository.find_category_rules(category_id)?;
    if category.format == CompetitionFormat::GroupsKnockout
        && (settings.group_count != configured.rules.group_count
            || settings.qualifiers_per_group != configured.rules.qualifiers_per_group)
    {
        return Err(ApplicationError::InvalidRules);
    }
    Ok(librett_domain::create_draw(
        &category,
        &draw_entries(repository, category_id)?,
        mode,
        settings,
        seeds,
        Uuid::new_v4(),
    )?)
}

pub fn save_category_draw(
    repository: &mut (impl TournamentRepository
              + PlayerRepository
              + DrawRepository
              + CategoryRulesRepository),
    tournament_id: Uuid,
    mut draw: librett_domain::CategoryDraw,
    expected_revision: u32,
) -> Result<librett_domain::CategoryDraw, ApplicationError> {
    let category = draw_category(repository, tournament_id, draw.category_id)?;
    let configured = repository.find_category_rules(draw.category_id)?;
    if category.format == CompetitionFormat::GroupsKnockout
        && (draw.settings.group_count != configured.rules.group_count
            || draw.settings.qualifiers_per_group != configured.rules.qualifiers_per_group)
    {
        return Err(ApplicationError::InvalidRules);
    }
    let entries = draw_entries(repository, draw.category_id)?;
    librett_domain::validate_draw(&draw, &category, &entries)?;
    // Names and clubs come from persisted entry snapshots, never from client text.
    draw.participants = entries
        .into_iter()
        .filter(|e| e.status == librett_domain::EntryStatus::Registered)
        .collect();
    repository.save_draw(draw, expected_revision)
}

pub trait PlayerRepository {
    fn list_players(&self) -> Result<Vec<librett_domain::Player>, ApplicationError>;
    fn find_player(&self, id: Uuid) -> Result<librett_domain::Player, ApplicationError>;
    fn insert_player(&mut self, player: &librett_domain::Player) -> Result<(), ApplicationError>;
    fn delete_player(&mut self, id: Uuid) -> Result<(), ApplicationError>;
    fn update_player(&mut self, player: &librett_domain::Player) -> Result<(), ApplicationError>;
    fn list_entries(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<librett_domain::Entry>, ApplicationError>;
    fn insert_entries(&mut self, entries: &[librett_domain::Entry])
        -> Result<(), ApplicationError>;
    fn insert_entry(&mut self, entry: &librett_domain::Entry) -> Result<(), ApplicationError> {
        self.insert_entries(std::slice::from_ref(entry))
    }
}

pub fn create_player(
    repository: &mut impl PlayerRepository,
    name: &str,
    club: &str,
) -> Result<librett_domain::Player, ApplicationError> {
    let player = librett_domain::Player::new(name, club)?;
    repository.insert_player(&player)?;
    Ok(player)
}

pub fn save_player_profile(
    repository: &mut impl PlayerRepository,
    id: Option<Uuid>,
    name: &str,
    club: &str,
    profile: librett_domain::PlayerProfile,
) -> Result<librett_domain::Player, ApplicationError> {
    let mut player = librett_domain::Player::new(name, club)?;
    player.profile = profile.validated()?;
    if let Some(id) = id {
        repository.find_player(id)?;
        player.id = id;
        repository.update_player(&player)?;
    } else {
        repository.insert_player(&player)?;
    }
    Ok(player)
}

pub fn delete_player(
    repository: &mut impl PlayerRepository,
    id: Uuid,
) -> Result<(), ApplicationError> {
    repository.delete_player(id)
}

pub fn register_entry(
    repository: &mut (impl TournamentRepository + PlayerRepository),
    tournament_id: Uuid,
    category_id: Uuid,
    player_ids: Vec<Uuid>,
) -> Result<librett_domain::Entry, ApplicationError> {
    let tournament = repository.find(tournament_id)?;
    let category = tournament
        .categories
        .iter()
        .find(|c| c.id == category_id && !c.archived)
        .ok_or(ApplicationError::NotFound)?;
    let members = player_ids
        .into_iter()
        .map(|id| repository.find_player(id))
        .collect::<Result<Vec<_>, _>>()?;
    let entry = librett_domain::Entry::new(category_id, category.discipline, members)?;
    let existing = repository.list_entries(category_id)?;
    if existing.iter().any(|registered| {
        registered
            .members
            .iter()
            .any(|member| entry.members.iter().any(|new| new.id == member.id))
    }) {
        return Err(ApplicationError::AlreadyRegistered);
    }
    repository.insert_entry(&entry)?;
    // Read the registration projection so previously recorded tournament-wide
    // attendance is reflected when entering an additional category.
    repository
        .list_entries(category_id)?
        .into_iter()
        .find(|saved| saved.id == entry.id)
        .ok_or(ApplicationError::Storage)
}

pub trait TournamentRepository {
    fn list(&self) -> Result<Vec<Tournament>, ApplicationError>;
    fn find(&self, id: Uuid) -> Result<Tournament, ApplicationError>;
    fn insert(&mut self, tournament: &Tournament) -> Result<(), ApplicationError>;
    fn insert_category(
        &mut self,
        tournament_id: Uuid,
        category: &librett_domain::Category,
    ) -> Result<(), ApplicationError>;
}

pub fn create_tournament(
    repository: &mut impl TournamentRepository,
    name: &str,
) -> Result<Tournament, ApplicationError> {
    let tournament = Tournament::new(name)?;
    repository.insert(&tournament)?;
    Ok(tournament)
}

pub fn add_category(
    repository: &mut impl TournamentRepository,
    tournament_id: Uuid,
    name: &str,
    discipline: Discipline,
    format: CompetitionFormat,
) -> Result<Tournament, ApplicationError> {
    let mut tournament = repository.find(tournament_id)?;
    tournament.add_category(name, discipline, format)?;
    let category = tournament
        .categories
        .last()
        .ok_or(ApplicationError::Storage)?;
    repository.insert_category(tournament_id, category)?;
    Ok(tournament)
}

pub trait RegistrationRepository {
    fn set_entry_status(
        &mut self,
        tournament_id: Uuid,
        entry_id: Uuid,
        status: librett_domain::EntryStatus,
    ) -> Result<(), ApplicationError>;
    fn set_player_attendance(
        &mut self,
        tournament_id: Uuid,
        player_id: Uuid,
        checked_in: bool,
    ) -> Result<(), ApplicationError>;
}

pub fn set_entry_status(
    repository: &mut impl RegistrationRepository,
    tournament_id: Uuid,
    entry_id: Uuid,
    status: librett_domain::EntryStatus,
) -> Result<(), ApplicationError> {
    repository.set_entry_status(tournament_id, entry_id, status)
}

pub fn set_player_attendance(
    repository: &mut impl RegistrationRepository,
    tournament_id: Uuid,
    player_id: Uuid,
    checked_in: bool,
) -> Result<(), ApplicationError> {
    repository.set_player_attendance(tournament_id, player_id, checked_in)
}

pub trait CashRepository {
    fn list_cash(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<librett_domain::CashRecord>, ApplicationError>;
    fn record_cash(
        &mut self,
        tournament_id: Uuid,
        record: librett_domain::CashRecord,
    ) -> Result<(), ApplicationError>;
}

pub fn record_cash(
    repository: &mut impl CashRepository,
    tournament_id: Uuid,
    mut record: librett_domain::CashRecord,
) -> Result<(), ApplicationError> {
    record.note = record.note.trim().to_owned();
    if !(1..=librett_domain::MAX_CASH_MINOR).contains(&record.amount_minor)
        || record.note.chars().count() > 500
    {
        return Err(ApplicationError::InvalidCash);
    }
    repository.record_cash(tournament_id, record)
}

pub fn add_category_with_fee(
    repository: &mut impl TournamentRepository,
    tournament_id: Uuid,
    name: &str,
    discipline: Discipline,
    format: CompetitionFormat,
    fee_minor: i64,
) -> Result<Tournament, ApplicationError> {
    if !(0..=librett_domain::MAX_CASH_MINOR).contains(&fee_minor) {
        return Err(ApplicationError::InvalidCash);
    }
    let mut tournament = repository.find(tournament_id)?;
    tournament.add_category(name, discipline, format)?;
    let category = tournament
        .categories
        .last_mut()
        .ok_or(ApplicationError::Storage)?;
    category.fee_minor = fee_minor;
    repository.insert_category(tournament_id, category)?;
    Ok(tournament)
}

pub trait CategoryRepository {
    fn delete_category(
        &mut self,
        tournament_id: Uuid,
        category_id: Uuid,
    ) -> Result<(), ApplicationError>;
}

pub fn register_entries(
    repository: &mut (impl TournamentRepository + PlayerRepository),
    tournament_id: Uuid,
    category_id: Uuid,
    player_groups: Vec<Vec<Uuid>>,
) -> Result<Vec<librett_domain::Entry>, ApplicationError> {
    if player_groups.is_empty() || player_groups.len() > 1000 {
        return Err(ApplicationError::InvalidMembers);
    }
    let tournament = repository.find(tournament_id)?;
    let category = tournament
        .categories
        .iter()
        .find(|c| c.id == category_id && !c.archived)
        .ok_or(ApplicationError::NotFound)?;
    let mut claimed = repository
        .list_entries(category_id)?
        .iter()
        .flat_map(|e| e.members.iter().map(|m| m.id))
        .collect::<std::collections::HashSet<_>>();
    let mut entries = Vec::new();
    for ids in player_groups {
        let players = ids
            .into_iter()
            .map(|id| repository.find_player(id))
            .collect::<Result<Vec<_>, _>>()?;
        let entry = librett_domain::Entry::new(category_id, category.discipline, players)?;
        for member in &entry.members {
            if !claimed.insert(member.id) {
                return Err(ApplicationError::AlreadyRegistered);
            }
        }
        entries.push(entry);
    }
    repository.insert_entries(&entries)?;
    let ids = entries
        .iter()
        .map(|e| e.id)
        .collect::<std::collections::HashSet<_>>();
    Ok(repository
        .list_entries(category_id)?
        .into_iter()
        .filter(|e| ids.contains(&e.id))
        .collect())
}

pub trait PlayerCashRepository {
    fn settle_player_cash_checked(
        &mut self,
        request_id: Uuid,
        tournament_id: Uuid,
        player_id: Uuid,
        entry_ids: Vec<Uuid>,
        paid: bool,
        expected: Vec<ExpectedCashAmount>,
    ) -> Result<(), ApplicationError>;
    fn cash_ledger(
        &self,
        tournament_id: Uuid,
    ) -> Result<librett_domain::CashLedger, ApplicationError>;
    fn settle_player_cash(
        &mut self,
        request_id: Uuid,
        tournament_id: Uuid,
        player_id: Uuid,
        entry_ids: Vec<Uuid>,
        paid: bool,
    ) -> Result<(), ApplicationError>;
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CategoryConfiguration {
    pub category_id: Uuid,
    pub revision: u32,
    pub rules: librett_domain::CategoryRules,
}

pub trait CategoryRulesRepository {
    fn save_configured_category(
        &mut self,
        tournament_id: Uuid,
        category: &librett_domain::Category,
        rules: &librett_domain::CategoryRules,
        expected_revision: u32,
    ) -> Result<CategoryConfiguration, ApplicationError>;
    fn find_category_rules(
        &self,
        category_id: Uuid,
    ) -> Result<CategoryConfiguration, ApplicationError>;
    fn insert_configured_category(
        &mut self,
        tournament_id: Uuid,
        category: &librett_domain::Category,
        rules: &librett_domain::CategoryRules,
    ) -> Result<(), ApplicationError>;
    fn save_category_rules(
        &mut self,
        tournament_id: Uuid,
        category_id: Uuid,
        rules: &librett_domain::CategoryRules,
        expected_revision: u32,
    ) -> Result<CategoryConfiguration, ApplicationError>;
}

pub fn create_category_with_rules(
    repository: &mut (impl TournamentRepository + CategoryRulesRepository),
    tournament_id: Uuid,
    category_id: Uuid,
    name: &str,
    discipline: Discipline,
    format: CompetitionFormat,
    fee_minor: i64,
    rules: librett_domain::CategoryRules,
) -> Result<Tournament, ApplicationError> {
    rules.validate(format)?;
    if !(0..=librett_domain::MAX_CASH_MINOR).contains(&fee_minor) {
        return Err(ApplicationError::InvalidCash);
    }
    let mut tournament = repository.find(tournament_id)?;
    if let Some(existing) = tournament.categories.iter().find(|c| c.id == category_id) {
        if existing.name != name.trim()
            || existing.discipline != discipline
            || existing.format != format
            || existing.fee_minor != fee_minor
            || existing.archived
            || repository.find_category_rules(category_id)?.rules != rules
        {
            return Err(ApplicationError::DrawConflict);
        }
        return Ok(tournament);
    }
    tournament.add_category(name, discipline, format)?;
    let category = tournament
        .categories
        .last_mut()
        .ok_or(ApplicationError::Storage)?;
    category.id = category_id;
    category.fee_minor = fee_minor;
    repository.insert_configured_category(tournament_id, category, &rules)?;
    Ok(tournament)
}

pub fn get_category_rules(
    repository: &(impl TournamentRepository + CategoryRulesRepository),
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<CategoryConfiguration, ApplicationError> {
    draw_category(repository, tournament_id, category_id)?;
    repository.find_category_rules(category_id)
}

pub fn save_category_rules(
    repository: &mut (impl TournamentRepository + CategoryRulesRepository),
    tournament_id: Uuid,
    category_id: Uuid,
    rules: librett_domain::CategoryRules,
    expected_revision: u32,
) -> Result<CategoryConfiguration, ApplicationError> {
    let category = draw_category(repository, tournament_id, category_id)?;
    rules.validate(category.format)?;
    repository.save_category_rules(tournament_id, category_id, &rules, expected_revision)
}

pub fn update_category_with_rules(
    repository: &mut (impl TournamentRepository + CategoryRulesRepository),
    tournament_id: Uuid,
    category_id: Uuid,
    name: &str,
    discipline: Discipline,
    format: CompetitionFormat,
    fee_minor: i64,
    rules: librett_domain::CategoryRules,
    expected_revision: u32,
) -> Result<Tournament, ApplicationError> {
    rules.validate(format)?;
    if !(0..=librett_domain::MAX_CASH_MINOR).contains(&fee_minor) {
        return Err(ApplicationError::InvalidCash);
    }
    let tournament = repository.find(tournament_id)?;
    let existing = tournament
        .categories
        .iter()
        .find(|c| c.id == category_id && !c.archived)
        .ok_or(ApplicationError::NotFound)?;
    let mut validated = Tournament::new("category validation")?;
    validated.add_category(name, discipline, format)?;
    let mut category = validated.categories.remove(0);
    category.id = existing.id;
    category.fee_minor = fee_minor;
    if tournament.categories.iter().any(|other| {
        other.id != category_id
            && other.discipline == discipline
            && other.name.to_lowercase() == category.name.to_lowercase()
    }) {
        return Err(ApplicationError::DuplicateCategory);
    }
    repository.save_configured_category(tournament_id, &category, &rules, expected_revision)?;
    repository.find(tournament_id)
}

pub trait GuardedPlayerRepository {
    fn save_player_checked(
        &mut self,
        request_id: Uuid,
        player: &librett_domain::Player,
        expected: Option<&librett_domain::Player>,
    ) -> Result<librett_domain::Player, ApplicationError>;
}
pub fn save_player_checked(
    repository: &mut impl GuardedPlayerRepository,
    request_id: Uuid,
    player_id: Uuid,
    name: &str,
    club: &str,
    profile: librett_domain::PlayerProfile,
    expected: Option<librett_domain::Player>,
) -> Result<librett_domain::Player, ApplicationError> {
    let mut player = librett_domain::Player::new(name, club)?;
    player.id = player_id;
    player.profile = profile.validated()?;
    if expected
        .as_ref()
        .is_some_and(|previous| previous.id != player_id)
    {
        return Err(ApplicationError::PlayerConflict);
    }
    repository.save_player_checked(request_id, &player, expected.as_ref())
}
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExpectedCashAmount {
    pub entry_id: Uuid,
    pub amount_minor: i64,
}

pub fn create_tournament_with_cover(
    repository: &mut impl TournamentRepository,
    id: Uuid,
    name: &str,
    cover: Option<String>,
) -> Result<Tournament, ApplicationError> {
    if let Some(image) = &cover {
        librett_domain::validate_jpeg(image, true)?;
    }
    let mut tournament = Tournament::new(name)?;
    tournament.id = id;
    tournament.cover = cover;
    match repository.find(id) {
        Ok(existing) => {
            if existing.name != tournament.name || existing.cover != tournament.cover {
                return Err(ApplicationError::DrawConflict);
            }
            return Ok(existing);
        }
        Err(ApplicationError::NotFound) => {}
        Err(error) => return Err(error),
    }
    repository.insert(&tournament)?;
    Ok(tournament)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryEditorState {
    pub category: librett_domain::Category,
    pub configuration: CategoryConfiguration,
    pub used: bool,
}
pub trait CategoryEditorRepository {
    fn category_editor_state(
        &self,
        tournament_id: Uuid,
        category_id: Uuid,
    ) -> Result<CategoryEditorState, ApplicationError>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatchPage {
    pub draw: Option<librett_domain::CategoryDraw>,
    pub rules: librett_domain::CategoryRules,
    pub rules_revision: u32,
    pub stale: bool,
    pub round_count: usize,
    pub total: usize,
    pub matches: Vec<librett_domain::ScheduledMatch>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveMatchRequest {
    pub request_id: Uuid,
    pub tournament_id: Uuid,
    pub category_id: Uuid,
    pub draw_id: Uuid,
    pub rules_revision: u32,
    pub key: String,
    pub expected_revision: u32,
    pub result: librett_domain::MatchResult,
    pub invalidate_downstream: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompetitionState {
    pub draw_id: Option<Uuid>,
    pub stale: bool,
    pub groups: Vec<librett_domain::GroupStanding>,
    pub slots: Vec<librett_domain::QualificationSlot>,
    pub matches: Vec<librett_domain::ScheduledMatch>,
    pub order_revisions: Vec<u32>,
    pub result_versions: Vec<u64>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GroupOrderRequest {
    pub request_id: Uuid,
    pub tournament_id: Uuid,
    pub category_id: Uuid,
    pub draw_id: Uuid,
    pub group: usize,
    pub expected_revision: u32,
    pub expected_result_version: u64,
    pub order: Option<Vec<Uuid>>,
    pub invalidate_downstream: bool,
}
