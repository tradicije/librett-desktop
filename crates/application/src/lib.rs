use librett_domain::{CompetitionFormat, Discipline, DomainError, Tournament};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationError {
    NameRequired,
    NameTooLong,
    DuplicateCategory,
    NotFound,
    Storage,
}

impl From<DomainError> for ApplicationError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::NameRequired => Self::NameRequired,
            DomainError::NameTooLong => Self::NameTooLong,
            DomainError::DuplicateCategory => Self::DuplicateCategory,
        }
    }
}

pub trait TournamentRepository {
    fn list(&self) -> Result<Vec<Tournament>, ApplicationError>;
    fn find(&self, id: Uuid) -> Result<Tournament, ApplicationError>;
    fn insert(&mut self, tournament: &Tournament) -> Result<(), ApplicationError>;
    fn insert_category(&mut self, tournament_id: Uuid, category: &librett_domain::Category) -> Result<(), ApplicationError>;
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
    let category = tournament.categories.last().ok_or(ApplicationError::Storage)?;
    repository.insert_category(tournament_id, category)?;
    Ok(tournament)
}
