use super::paths::exports_directory;
use crate::state::Database;
use crate::state::PrintReports;
use librett_application::{self as application, ApplicationError};
use tauri::Manager;
use uuid::Uuid;

#[tauri::command]
pub(crate) fn save_report(
    app: tauri::AppHandle,
    database: tauri::State<Database>,
    kind: String,
    format: String,
    content: String,
) -> Result<String, ApplicationError> {
    if !["groups", "draw", "results", "cash", "schedule"].contains(&kind.as_str())
        || !["csv", "html"].contains(&format.as_str())
        || content.len() > 20 * 1024 * 1024
    {
        return Err(ApplicationError::InvalidResult);
    }
    let path =
        exports_directory(&app)?.join(format!("librett-{kind}-{}.{}", Uuid::new_v4(), format));
    std::fs::write(&path, content.as_bytes()).map_err(|_| ApplicationError::Storage)?;
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .record_export(&format!("{kind}.{format}"))?;
    Ok(path.to_string_lossy().into_owned())
}

#[tauri::command]
pub(crate) fn open_print_report(
    app: tauri::AppHandle,
    reports: tauri::State<PrintReports>,
    content: String,
) -> Result<(), ApplicationError> {
    if content.len() > 20 * 1024 * 1024 {
        return Err(ApplicationError::InvalidResult);
    }
    let token = Uuid::new_v4().to_string();
    {
        let mut items = reports.0.lock().map_err(|_| ApplicationError::Storage)?;
        if items.len() >= 8 {
            return Err(ApplicationError::Storage);
        }
        items.insert(token.clone(), content);
    }
    let label = format!("report-{token}");
    let window = tauri::WebviewWindowBuilder::new(
        &app,
        &label,
        tauri::WebviewUrl::App(format!("index.html?print={token}").into()),
    )
    .title("LibreTT — Print")
    .inner_size(1000.0, 800.0)
    .build();
    let window = match window {
        Ok(window) => window,
        Err(_) => {
            reports
                .0
                .lock()
                .map_err(|_| ApplicationError::Storage)?
                .remove(&token);
            return Err(ApplicationError::Storage);
        }
    };
    let handle = app.clone();
    window.on_window_event(move |event| {
        if matches!(event, tauri::WindowEvent::Destroyed) {
            if let Ok(mut items) = handle.state::<PrintReports>().0.lock() {
                items.remove(&token);
            }
        }
    });
    Ok(())
}

#[tauri::command]
pub(crate) fn get_print_report(
    window: tauri::WebviewWindow,
    reports: tauri::State<PrintReports>,
    token: String,
) -> Result<String, ApplicationError> {
    if window.label() != format!("report-{token}") {
        return Err(ApplicationError::NotFound);
    }
    reports
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .get(&token)
        .cloned()
        .ok_or(ApplicationError::NotFound)
}

#[tauri::command]
pub(crate) fn print_report(window: tauri::WebviewWindow) -> Result<(), ApplicationError> {
    if !window.label().starts_with("report-") {
        return Err(ApplicationError::NotFound);
    }
    window.print().map_err(|_| ApplicationError::Storage)
}

#[tauri::command]
pub(crate) fn get_category_report(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<application::CategoryReportData, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .category_report(tournament_id, category_id)
}
