use crate::registry_http;
use crate::state::Database;
use librett_application::{self as application, ApplicationError};
use uuid::Uuid;

#[tauri::command]
pub(crate) async fn preview_registry(
    database: tauri::State<'_, Database>,
    payload: Option<String>,
    source_url: Option<String>,
    decisions: Vec<application::registry::RegistryDecision>,
) -> Result<application::registry::RegistryPreview, ApplicationError> {
    let bytes = if let Some(payload) = payload {
        payload
    } else {
        let url = source_url
            .clone()
            .ok_or(ApplicationError::InvalidRegistry)?;
        tauri::async_runtime::spawn_blocking(move || registry_http::snapshot(&url))
            .await
            .map_err(|_| ApplicationError::RegistryFetch)??
    };
    application::registry::preview_registry(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        &bytes,
        source_url,
        &decisions,
    )
}

#[tauri::command]
pub(crate) fn confirm_registry_import(
    database: tauri::State<Database>,
    id: Uuid,
) -> Result<application::registry::RegistryImportResult, ApplicationError> {
    application::registry::RegistryRepository::confirm_registry(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        id,
    )
}

#[tauri::command]
pub(crate) fn registry_sources(
    database: tauri::State<Database>,
) -> Result<Vec<application::registry::RegistrySource>, ApplicationError> {
    application::registry::RegistryRepository::registry_sources(
        &*database.0.lock().map_err(|_| ApplicationError::Storage)?,
    )
}

#[tauri::command]
pub(crate) fn registry_previews(
    database: tauri::State<Database>,
) -> Result<Vec<application::registry::RegistryPreview>, ApplicationError> {
    application::registry::RegistryRepository::registry_previews(
        &*database.0.lock().map_err(|_| ApplicationError::Storage)?,
    )
}

#[tauri::command]
pub(crate) fn cancel_registry_preview(
    database: tauri::State<Database>,
    id: Uuid,
) -> Result<(), ApplicationError> {
    application::registry::RegistryRepository::cancel_registry(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        id,
    )
}

#[tauri::command]
pub(crate) async fn download_registry_photo(
    database: tauri::State<'_, Database>,
    id: Uuid,
    remote_id: Uuid,
) -> Result<(String, String), ApplicationError> {
    let descriptor = application::registry::RegistryRepository::registry_photo(
        &*database.0.lock().map_err(|_| ApplicationError::Storage)?,
        id,
        remote_id,
    )?;
    let mime = descriptor.mime_type.clone();
    let encoded = tauri::async_runtime::spawn_blocking(move || registry_http::photo(&descriptor))
        .await
        .map_err(|_| ApplicationError::RegistryFetch)??;
    Ok((encoded, mime))
}

#[tauri::command]
pub(crate) fn refine_registry(
    database: tauri::State<Database>,
    id: Uuid,
    decisions: Vec<application::registry::RegistryDecision>,
) -> Result<application::registry::RegistryPreview, ApplicationError> {
    application::registry::RegistryRepository::refine_registry(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        id,
        &decisions,
    )
}
