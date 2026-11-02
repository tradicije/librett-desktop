use crate::state::Database;
use librett_application::{self as application, ApplicationError};
use std::collections::HashMap;
use uuid::Uuid;

#[tauri::command]
pub(crate) fn get_schedule(
    database: tauri::State<Database>,
    tournament_id: Uuid,
) -> Result<application::ScheduleState, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .schedule(tournament_id)
}

#[tauri::command]
pub(crate) fn change_schedule(
    database: tauri::State<Database>,
    request: application::ScheduleRequest,
) -> Result<application::ScheduleState, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .change_schedule(request)
}

#[tauri::command]
pub(crate) fn get_match_tables(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    draw_id: Uuid,
) -> Result<HashMap<String, usize>, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .match_tables(tournament_id, draw_id)
}
