use crate::ApplicationError;
use uuid::Uuid;

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

pub trait GuardedPlayerRepository {
    fn save_player_checked(
        &mut self,
        request_id: Uuid,
        player: &librett_domain::Player,
        expected: Option<&librett_domain::Player>,
    ) -> Result<librett_domain::Player, ApplicationError>;
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
    if player.profile.birth_year.is_none() {
        return Err(ApplicationError::BirthYearRequired);
    }
    if expected
        .as_ref()
        .is_some_and(|previous| previous.id != player_id)
    {
        return Err(ApplicationError::PlayerConflict);
    }
    repository.save_player_checked(request_id, &player, expected.as_ref())
}

pub fn delete_player(
    repository: &mut impl PlayerRepository,
    id: Uuid,
) -> Result<(), ApplicationError> {
    repository.delete_player(id)
}
