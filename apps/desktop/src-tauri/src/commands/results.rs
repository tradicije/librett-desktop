use crate::state::Database;
use librett_application::{self as application, ApplicationError};
use uuid::Uuid;

#[tauri::command]
pub(crate) fn get_category_results(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<application::CategoryResults, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .category_results(tournament_id, category_id)
}

#[tauri::command]
pub(crate) fn get_tournament_progress(
    database: tauri::State<Database>,
    tournament_id: Uuid,
) -> Result<application::TournamentProgress, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .tournament_progress(tournament_id)
}

#[tauri::command]
pub(crate) fn change_completion(
    database: tauri::State<Database>,
    request: application::CompletionRequest,
) -> Result<application::TournamentProgress, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .change_completion(request)
}
