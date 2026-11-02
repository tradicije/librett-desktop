use crate::state::Database;
use librett_application::{self as application, ApplicationError, PlayerRepository};
use librett_domain::{Player, PlayerProfile};
use uuid::Uuid;

#[tauri::command]
pub(crate) fn list_players(
    database: tauri::State<Database>,
) -> Result<Vec<Player>, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .list_players()
}

#[tauri::command]
pub(crate) fn save_player_checked(
    database: tauri::State<Database>,
    request_id: Uuid,
    player_id: Uuid,
    name: String,
    club: String,
    profile: PlayerProfile,
    expected: Option<Player>,
) -> Result<Player, ApplicationError> {
    application::save_player_checked(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        request_id,
        player_id,
        &name,
        &club,
        profile,
        expected,
    )
}

#[tauri::command]
pub(crate) fn get_player(
    database: tauri::State<Database>,
    id: String,
) -> Result<Player, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .find_player(Uuid::parse_str(&id).map_err(|_| ApplicationError::NotFound)?)
}

#[tauri::command]
pub(crate) fn delete_player(
    database: tauri::State<Database>,
    id: String,
) -> Result<(), ApplicationError> {
    application::delete_player(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        Uuid::parse_str(&id).map_err(|_| ApplicationError::NotFound)?,
    )
}
