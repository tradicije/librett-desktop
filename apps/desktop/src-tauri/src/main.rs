#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use librett_application::{
    self as application, ApplicationError, PlayerRepository, TournamentRepository,
};
use librett_domain::{CompetitionFormat, Discipline, Entry, Player, PlayerProfile, Tournament};
use librett_storage_sqlite::SqliteTournamentRepository;
use std::sync::Mutex;
use tauri::Manager;
use uuid::Uuid;

struct Database(Mutex<SqliteTournamentRepository>);

#[tauri::command]
fn list_tournaments(database: tauri::State<Database>) -> Result<Vec<Tournament>, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .list()
}

#[tauri::command]
fn create_tournament(
    database: tauri::State<Database>,
    name: String,
) -> Result<Tournament, ApplicationError> {
    application::create_tournament(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        &name,
    )
}

#[tauri::command]
fn add_category(
    database: tauri::State<Database>,
    tournament_id: String,
    name: String,
    discipline: Discipline,
    format: CompetitionFormat,
) -> Result<Tournament, ApplicationError> {
    let id = Uuid::parse_str(&tournament_id).map_err(|_| ApplicationError::NotFound)?;
    application::add_category(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        id,
        &name,
        discipline,
        format,
    )
}

fn main() {
    let context = tauri::generate_context!();
    // GDK's Wayland app_id defaults to the GLib program name. Set it before
    // GTK initializes so KWin matches the same desktop entry as the taskbar.
    #[cfg(target_os = "linux")]
    glib::set_prgname(Some(&context.config().identifier));

    tauri::Builder::default()
        .setup(|app| {
            let directory = app.path().app_data_dir()?;
            std::fs::create_dir_all(&directory)?;
            let repository = SqliteTournamentRepository::open(directory.join("librett.sqlite"))?;
            app.manage(Database(Mutex::new(repository)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_tournaments,
            create_tournament,
            add_category,
            list_players,
            get_player,
            delete_player,
            create_player,
            save_player_profile,
            list_entries,
            register_entry
        ])
        .run(context)
        .expect("Unable to start LibreTT");
}

#[tauri::command]
fn list_players(database: tauri::State<Database>) -> Result<Vec<Player>, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .list_players()
}

#[tauri::command]
fn create_player(
    database: tauri::State<Database>,
    name: String,
    club: String,
) -> Result<Player, ApplicationError> {
    application::create_player(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        &name,
        &club,
    )
}

#[tauri::command]
fn list_entries(
    database: tauri::State<Database>,
    category_id: String,
) -> Result<Vec<Entry>, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .list_entries(Uuid::parse_str(&category_id).map_err(|_| ApplicationError::NotFound)?)
}

#[tauri::command]
fn register_entry(
    database: tauri::State<Database>,
    tournament_id: String,
    category_id: String,
    player_ids: Vec<String>,
) -> Result<Entry, ApplicationError> {
    let player_ids = player_ids
        .into_iter()
        .map(|id| Uuid::parse_str(&id).map_err(|_| ApplicationError::NotFound))
        .collect::<Result<Vec<_>, _>>()?;
    application::register_entry(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        Uuid::parse_str(&tournament_id).map_err(|_| ApplicationError::NotFound)?,
        Uuid::parse_str(&category_id).map_err(|_| ApplicationError::NotFound)?,
        player_ids,
    )
}

#[tauri::command]
fn save_player_profile(
    database: tauri::State<Database>,
    id: Option<String>,
    name: String,
    club: String,
    profile: PlayerProfile,
) -> Result<Player, ApplicationError> {
    let id = id
        .map(|id| Uuid::parse_str(&id).map_err(|_| ApplicationError::NotFound))
        .transpose()?;
    application::save_player_profile(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        id,
        &name,
        &club,
        profile,
    )
}

#[tauri::command]
fn get_player(database: tauri::State<Database>, id: String) -> Result<Player, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .find_player(Uuid::parse_str(&id).map_err(|_| ApplicationError::NotFound)?)
}

#[tauri::command]
fn delete_player(database: tauri::State<Database>, id: String) -> Result<(), ApplicationError> {
    application::delete_player(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        Uuid::parse_str(&id).map_err(|_| ApplicationError::NotFound)?,
    )
}
