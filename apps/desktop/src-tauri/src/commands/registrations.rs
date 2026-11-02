use crate::state::Database;
use librett_application::{self as application, ApplicationError, PlayerRepository};
use librett_domain::{Entry, EntryStatus};
use uuid::Uuid;

#[tauri::command]
pub(crate) fn list_entries(
    database: tauri::State<Database>,
    category_id: String,
) -> Result<Vec<Entry>, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .list_entries(Uuid::parse_str(&category_id).map_err(|_| ApplicationError::NotFound)?)
}

#[tauri::command]
pub(crate) fn register_entry(
    database: tauri::State<Database>,
    tournament_id: String,
    category_id: String,
    player_ids: Vec<String>,
) -> Result<Entry, ApplicationError> {
    let player_ids = player_ids
        .into_iter()
        .map(|id| Uuid::parse_str(&id).map_err(|_| ApplicationError::NotFound))
        .collect::<Result<Vec<_>, _>>()?;
    application::register_entry(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        Uuid::parse_str(&tournament_id).map_err(|_| ApplicationError::NotFound)?,
        Uuid::parse_str(&category_id).map_err(|_| ApplicationError::NotFound)?,
        player_ids,
    )
}

#[tauri::command]
pub(crate) fn register_entries(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
    player_groups: Vec<Vec<Uuid>>,
) -> Result<Vec<Entry>, ApplicationError> {
    application::register_entries(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        category_id,
        player_groups,
    )
}

#[tauri::command]
pub(crate) fn set_entry_status(
    database: tauri::State<Database>,
    tournament_id: String,
    entry_id: String,
    status: EntryStatus,
) -> Result<(), ApplicationError> {
    application::set_entry_status(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        Uuid::parse_str(&tournament_id).map_err(|_| ApplicationError::NotFound)?,
        Uuid::parse_str(&entry_id).map_err(|_| ApplicationError::NotFound)?,
        status,
    )
}

#[tauri::command]
pub(crate) fn set_player_attendance(
    database: tauri::State<Database>,
    tournament_id: String,
    player_id: String,
    checked_in: bool,
) -> Result<(), ApplicationError> {
    application::set_player_attendance(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        Uuid::parse_str(&tournament_id).map_err(|_| ApplicationError::NotFound)?,
        Uuid::parse_str(&player_id).map_err(|_| ApplicationError::NotFound)?,
        checked_in,
    )
}

#[tauri::command]
pub(crate) fn registration_started(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<bool, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .registration_started(tournament_id, category_id)
}
