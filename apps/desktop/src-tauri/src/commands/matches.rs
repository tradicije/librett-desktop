use crate::state::Database;
use librett_application::{self as application, ApplicationError};
use uuid::Uuid;

#[tauri::command]
pub(crate) fn get_match_page(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
    group: usize,
    round: usize,
    page: usize,
    knockout: bool,
) -> Result<application::MatchPage, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .match_page(tournament_id, category_id, group, round, page, knockout)
}

#[tauri::command]
pub(crate) fn save_match_result(
    database: tauri::State<Database>,
    request: application::SaveMatchRequest,
) -> Result<librett_domain::ScheduledMatch, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .save_match_result(request)
}
