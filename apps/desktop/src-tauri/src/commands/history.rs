use crate::state::Database;
use librett_application::ApplicationError;
use uuid::Uuid;

#[tauri::command]
pub(crate) fn get_action_history(
    database: tauri::State<Database>,
    kinds: Vec<String>,
    tournament_id: Option<Uuid>,
    category_id: Option<Uuid>,
    action: Option<String>,
    before_at: Option<String>,
    before_id: Option<i64>,
) -> Result<librett_storage_sqlite::HistoryPage, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .action_history(
            kinds,
            tournament_id,
            category_id,
            action,
            before_at,
            before_id,
        )
}
