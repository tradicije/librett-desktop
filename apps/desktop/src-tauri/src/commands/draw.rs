use crate::state::Database;
use librett_application::{self as application, ApplicationError};
use librett_domain::{CategoryDraw, DrawMode, DrawSettings};
use uuid::Uuid;

#[tauri::command]
pub(crate) fn get_category_draw(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<Option<CategoryDraw>, ApplicationError> {
    application::get_category_draw(
        &*database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        category_id,
    )
}

#[tauri::command]
pub(crate) fn preview_category_draw(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
    mode: DrawMode,
    settings: DrawSettings,
    seeds: Vec<Uuid>,
) -> Result<CategoryDraw, ApplicationError> {
    application::preview_category_draw(
        &*database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        category_id,
        mode,
        settings,
        seeds,
    )
}

#[tauri::command]
pub(crate) fn save_category_draw(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    draw: CategoryDraw,
    expected_revision: u32,
    confirm_restart: Option<bool>,
) -> Result<CategoryDraw, ApplicationError> {
    application::save_category_draw(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        draw,
        expected_revision,
        confirm_restart.unwrap_or(false),
    )
}

#[tauri::command]
pub(crate) fn get_competition_state(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<application::CompetitionState, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .competition_state(tournament_id, category_id)
}

#[tauri::command]
pub(crate) fn save_group_order(
    database: tauri::State<Database>,
    request: application::GroupOrderRequest,
) -> Result<application::CompetitionState, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .save_group_order(request)
}

#[tauri::command]
pub(crate) fn save_knockout_fillers(
    database: tauri::State<Database>,
    request: application::SaveFillersRequest,
) -> Result<application::CompetitionState, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .save_knockout_fillers(request)
}
