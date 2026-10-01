use librett_domain::{CompetitionFormat, Discipline, DomainError, Tournament};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationError {
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
            DomainError::InvalidMembers => Self::InvalidMembers,
            DomainError::NameRequired => Self::NameRequired,
            DomainError::NameTooLong => Self::NameTooLong,
            DomainError::DuplicateCategory => Self::DuplicateCategory,
        }
    }
}

pub trait PlayerRepository {
    fn list_players(&self) -> Result<Vec<librett_domain::Player>, ApplicationError>;
    fn find_player(&self, id: Uuid) -> Result<librett_domain::Player, ApplicationError>;
    fn insert_player(&mut self, player: &librett_domain::Player) -> Result<(), ApplicationError>;
    fn list_entries(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<librett_domain::Entry>, ApplicationError>;
    fn insert_entry(&mut self, entry: &librett_domain::Entry) -> Result<(), ApplicationError>;
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
        .find(|c| c.id == category_id)
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
    Ok(entry)
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
