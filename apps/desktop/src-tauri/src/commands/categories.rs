use crate::state::Database;
use librett_application::{
    self as application, ApplicationError, CategoryEditorRepository, CategoryRepository,
    TournamentRepository,
};
use librett_domain::{CategoryRules, CompetitionFormat, Discipline, Tournament};
use uuid::Uuid;

#[tauri::command]
pub(crate) fn add_category(
    database: tauri::State<Database>,
    tournament_id: String,
    name: String,
    discipline: Discipline,
    format: CompetitionFormat,
    fee_minor: i64,
) -> Result<Tournament, ApplicationError> {
    let id = Uuid::parse_str(&tournament_id).map_err(|_| ApplicationError::NotFound)?;
    application::add_category_with_fee(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        id,
        &name,
        discipline,
        format,
        fee_minor,
    )
}

#[tauri::command]
pub(crate) fn delete_category(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<Tournament, ApplicationError> {
    let mut repo = database.0.lock().map_err(|_| ApplicationError::Storage)?;
    repo.delete_category(tournament_id, category_id)?;
    repo.find(tournament_id)
}

#[tauri::command]
pub(crate) fn create_category_with_rules(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
    name: String,
    discipline: Discipline,
    format: CompetitionFormat,
    fee_minor: i64,
    rules: CategoryRules,
) -> Result<Tournament, ApplicationError> {
    application::create_category_with_rules(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        category_id,
        &name,
        discipline,
        format,
        fee_minor,
        rules,
    )
}

#[tauri::command]
pub(crate) fn get_category_rules(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<application::CategoryConfiguration, ApplicationError> {
    application::get_category_rules(
        &*database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        category_id,
    )
}

#[tauri::command]
pub(crate) fn save_category_rules(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
    rules: CategoryRules,
    expected_revision: u32,
    invalidate_downstream: Option<bool>,
) -> Result<application::CategoryConfiguration, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .save_category_rules_confirmed(
            tournament_id,
            category_id,
            &rules,
            expected_revision,
            invalidate_downstream.unwrap_or(false),
        )
}

#[tauri::command]
pub(crate) fn update_category_with_rules(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
    name: String,
    discipline: Discipline,
    format: CompetitionFormat,
    fee_minor: i64,
    rules: CategoryRules,
    expected_revision: u32,
    invalidate_downstream: Option<bool>,
) -> Result<Tournament, ApplicationError> {
    application::update_category_with_rules_confirmed(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        category_id,
        &name,
        discipline,
        format,
        fee_minor,
        rules,
        expected_revision,
        invalidate_downstream.unwrap_or(false),
    )
}

#[tauri::command]
pub(crate) fn get_category_editor_state(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<application::CategoryEditorState, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .category_editor_state(tournament_id, category_id)
}
