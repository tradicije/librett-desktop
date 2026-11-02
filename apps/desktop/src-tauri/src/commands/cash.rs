use crate::state::Database;
use librett_application::{
    self as application, ApplicationError, CashRepository, PlayerCashRepository,
};
use uuid::Uuid;

#[tauri::command]
pub(crate) fn list_cash(
    database: tauri::State<Database>,
    tournament_id: Uuid,
) -> Result<Vec<librett_domain::CashRecord>, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .list_cash(tournament_id)
}

#[tauri::command]
pub(crate) fn record_cash(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    record: librett_domain::CashRecord,
) -> Result<(), ApplicationError> {
    application::record_cash(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        record,
    )
}

#[tauri::command]
pub(crate) fn cash_ledger(
    database: tauri::State<Database>,
    tournament_id: Uuid,
) -> Result<librett_domain::CashLedger, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .cash_ledger(tournament_id)
}

#[tauri::command]
pub(crate) fn settle_player_cash(
    database: tauri::State<Database>,
    request_id: Uuid,
    tournament_id: Uuid,
    player_id: Uuid,
    entry_ids: Vec<Uuid>,
    paid: bool,
    expected: Vec<application::ExpectedCashAmount>,
) -> Result<(), ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .settle_player_cash_checked(
            request_id,
            tournament_id,
            player_id,
            entry_ids,
            paid,
            expected,
        )
}
