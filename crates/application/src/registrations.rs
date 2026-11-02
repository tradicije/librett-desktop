use crate::{ApplicationError, PlayerRepository, TournamentRepository};
use uuid::Uuid;

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
