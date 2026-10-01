#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use librett_application::{self as application, ApplicationError, TournamentRepository};
use librett_domain::{CompetitionFormat, Discipline, Tournament};
use librett_storage_sqlite::SqliteTournamentRepository;
use std::sync::Mutex;
use tauri::Manager;
use uuid::Uuid;

struct Database(Mutex<SqliteTournamentRepository>);

#[tauri::command]
fn list_tournaments(database: tauri::State<Database>) -> Result<Vec<Tournament>, ApplicationError> {
    database.0.lock().map_err(|_| ApplicationError::Storage)?.list()
}

#[tauri::command]
fn create_tournament(database: tauri::State<Database>, name: String) -> Result<Tournament, ApplicationError> {
    application::create_tournament(&mut *database.0.lock().map_err(|_| ApplicationError::Storage)?, &name)
}

#[tauri::command]
fn add_category(database: tauri::State<Database>, tournament_id: String, name: String, discipline: Discipline, format: CompetitionFormat) -> Result<Tournament, ApplicationError> {
    let id = Uuid::parse_str(&tournament_id).map_err(|_| ApplicationError::NotFound)?;
    application::add_category(&mut *database.0.lock().map_err(|_| ApplicationError::Storage)?, id, &name, discipline, format)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let directory = app.path().app_data_dir()?;
            std::fs::create_dir_all(&directory)?;
            let repository = SqliteTournamentRepository::open(directory.join("librett.sqlite"))?;
            app.manage(Database(Mutex::new(repository)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![list_tournaments, create_tournament, add_category])
        .run(tauri::generate_context!())
        .expect("Unable to start LibreTT");
}
