use super::paths::{data_directory, exports_directory};
use crate::state::Database;
use base64::Engine;
use librett_application::{self as application, ApplicationError};
use librett_storage_sqlite::SqliteTournamentRepository;
use uuid::Uuid;

#[tauri::command]
pub(crate) fn list_backups(
    app: tauri::AppHandle,
) -> Result<Vec<application::BackupInfo>, ApplicationError> {
    SqliteTournamentRepository::backups(&data_directory(&app)?.join("backups"))
}

#[tauri::command]
pub(crate) fn create_backup(
    app: tauri::AppHandle,
    database: tauri::State<Database>,
) -> Result<application::BackupInfo, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .create_backup(&data_directory(&app)?.join("backups"), "manual")
}

#[tauri::command]
pub(crate) fn automatic_backup(
    app: tauri::AppHandle,
    database: tauri::State<Database>,
) -> Result<(), ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .automatic_backup(&data_directory(&app)?.join("backups"))
}

#[tauri::command]
pub(crate) fn restore_backup(
    app: tauri::AppHandle,
    database: tauri::State<Database>,
    name: String,
) -> Result<(), ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .restore_backup(&data_directory(&app)?.join("backups"), &name)
}

#[tauri::command]
pub(crate) fn import_backup(
    app: tauri::AppHandle,
    database: tauri::State<Database>,
    encoded: String,
) -> Result<(), ApplicationError> {
    if encoded.len() > 360 * 1024 * 1024 {
        return Err(ApplicationError::InvalidBackup);
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| ApplicationError::InvalidBackup)?;
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .import_backup(&data_directory(&app)?.join("backups"), &bytes)
}

#[tauri::command]
pub(crate) fn export_backup(
    app: tauri::AppHandle,
    database: tauri::State<Database>,
    name: String,
) -> Result<String, ApplicationError> {
    let source =
        SqliteTournamentRepository::backup_path(&data_directory(&app)?.join("backups"), &name)?;
    let target = exports_directory(&app)?.join(format!("librett-backup-{}.sqlite", Uuid::new_v4()));
    std::fs::copy(source, &target).map_err(|_| ApplicationError::Storage)?;
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .record_export(&name)?;
    Ok(target.to_string_lossy().into_owned())
}
