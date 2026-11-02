use librett_application::ApplicationError;
use tauri::Manager;

pub(super) fn data_directory(
    app: &tauri::AppHandle,
) -> Result<std::path::PathBuf, ApplicationError> {
    app.path()
        .app_data_dir()
        .map_err(|_| ApplicationError::Storage)
}

pub(super) fn exports_directory(
    app: &tauri::AppHandle,
) -> Result<std::path::PathBuf, ApplicationError> {
    let directory = app
        .path()
        .download_dir()
        .or_else(|_| app.path().app_data_dir())
        .map_err(|_| ApplicationError::Storage)?
        .join("LibreTT");
    std::fs::create_dir_all(&directory).map_err(|_| ApplicationError::Storage)?;
    Ok(directory)
}
