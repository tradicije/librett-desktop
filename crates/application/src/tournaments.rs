use crate::ApplicationError;
use librett_domain::Tournament;
use uuid::Uuid;

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
