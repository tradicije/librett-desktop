use super::paths::{data_directory, exports_directory};
use librett_application::ApplicationError;

#[tauri::command]
pub(crate) fn exit_application(app: tauri::AppHandle) {
    app.exit(0);
}

// Only project-owned destinations can be opened from the home notice.
#[tauri::command]
pub(crate) fn open_official_link(destination: String) -> Result<(), ApplicationError> {
    let url = match destination.as_str() {
        "website" => "https://librett.org",
        "source" => "https://github.com/tradicije/librett-desktop",
        _ => return Err(ApplicationError::NotFound),
    };
    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open").arg(url).spawn();
    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("explorer.exe").arg(url).spawn();
    #[cfg(target_os = "linux")]
    let result = std::process::Command::new("xdg-open").arg(url).spawn();
    result.map_err(|_| ApplicationError::Storage)?;
    Ok(())
}

#[tauri::command]
pub(crate) fn open_data_folder(
    app: tauri::AppHandle,
    kind: String,
) -> Result<(), ApplicationError> {
    let path = match kind.as_str() {
        "backups" => data_directory(&app)?.join("backups"),
        "exports" => exports_directory(&app)?,
        _ => return Err(ApplicationError::NotFound),
    };
    std::fs::create_dir_all(&path).map_err(|_| ApplicationError::Storage)?;
    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open").arg(&path).spawn();
    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("explorer.exe")
        .arg(&path)
        .spawn();
    #[cfg(target_os = "linux")]
    let result = std::process::Command::new("xdg-open").arg(&path).spawn();
    result.map_err(|_| ApplicationError::Storage)?;
    Ok(())
}
