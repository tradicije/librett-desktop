use crate::state::Database;
use librett_application::{self as application, ApplicationError, TournamentRepository};
use librett_domain::Tournament;
use uuid::Uuid;

#[tauri::command]
pub(crate) fn list_tournaments(
    database: tauri::State<Database>,
) -> Result<Vec<Tournament>, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .list()
}

#[tauri::command]
pub(crate) fn list_trashed_tournaments(
    database: tauri::State<Database>,
) -> Result<Vec<Tournament>, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .list_trashed_tournaments()
}

#[tauri::command]
pub(crate) fn trash_tournament(
    database: tauri::State<Database>,
    id: Uuid,
) -> Result<(), ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .trash_tournament(id)
}

#[tauri::command]
pub(crate) fn restore_tournament(
    database: tauri::State<Database>,
    id: Uuid,
) -> Result<Tournament, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .restore_tournament(id)
}

#[tauri::command]
pub(crate) fn create_tournament_with_cover(
    database: tauri::State<Database>,
    id: Uuid,
    name: String,
    cover: Option<String>,
) -> Result<Tournament, ApplicationError> {
    application::create_tournament_with_cover(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        id,
        &name,
        cover,
    )
}

#[tauri::command]
pub(crate) fn update_tournament_details(
    database: tauri::State<Database>,
    id: Uuid,
    name: String,
    cover: Option<String>,
    expected_name: String,
    expected_cover: Option<String>,
) -> Result<Tournament, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .update_tournament_details(id, &name, cover, &expected_name, expected_cover)
}
