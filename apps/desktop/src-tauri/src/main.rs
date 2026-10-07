#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::Engine;
use librett_application::{
    self as application, ApplicationError, CashRepository, CategoryEditorRepository,
    CategoryRepository, PlayerCashRepository, PlayerRepository, TournamentRepository,
};
use librett_domain::{
    CategoryDraw, CategoryRules, CompetitionFormat, Discipline, DrawMode, DrawSettings, Entry,
    EntryStatus, Player, PlayerProfile, Tournament,
};
use librett_storage_sqlite::SqliteTournamentRepository;
use std::collections::HashMap;
use std::sync::Mutex;
struct PrintReports(Mutex<HashMap<String, String>>);
use tauri::{Emitter, Manager};
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
fn add_category(
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

#[cfg(target_os = "macos")]
fn center_window_controls(window: &tauri::WebviewWindow) -> tauri::Result<()> {
    window.with_webview(|webview| {
        // SAFETY: Tauri executes with_webview on the main thread and provides
        // a live WKWebView. No native references escape this callback.
        let native = unsafe { &*webview.inner().cast::<objc2_web_kit::WKWebView>() };
        let Some(window) = native.window() else {
            return;
        };
        let bounds = native.bounds();
        // Keep in sync with the 48 CSS-pixel workspace title bar. AppKit uses
        // logical points as well; convert coordinates rather than assuming
        // a fixed traffic-light size or title-bar inset on each macOS release.
        let desired = bounds.origin.y
            + if native.isFlipped() {
                24.0
            } else {
                bounds.size.height - 24.0
            };
        for kind in [
            objc2_app_kit::NSWindowButton::CloseButton,
            objc2_app_kit::NSWindowButton::MiniaturizeButton,
            objc2_app_kit::NSWindowButton::ZoomButton,
        ] {
            let Some(button) = window.standardWindowButton(kind) else {
                continue;
            };
            // SAFETY: this standard AppKit button is live and accessed on the
            // main thread; its parent is retained for this callback only.
            let Some(parent) = (unsafe { button.superview() }) else {
                continue;
            };
            let frame = button.frame();
            let mut center = frame.origin;
            center.x += frame.size.width / 2.0;
            center.y += frame.size.height / 2.0;
            let actual = parent.convertPoint_toView(center, Some(native));
            let mut origin = frame.origin;
            let delta = desired - actual.y;
            origin.y += if parent.isFlipped() == native.isFlipped() {
                delta
            } else {
                -delta
            };
            button.setFrameOrigin(origin);
        }
    })
}

fn main() {
    let context = tauri::generate_context!();
    #[cfg(target_os = "macos")]
    {
        // Development runs a bare Cargo executable rather than a named .app.
        // Set its display name before AppKit initializes the application UI.
        let name = context
            .config()
            .product_name
            .as_deref()
            .unwrap_or("LibreTT");
        objc2_foundation::NSProcessInfo::processInfo()
            .setProcessName(&objc2_foundation::NSString::from_str(name));
    }
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    let context = {
        let mut context = context;
        // The workspace strip owns Linux and Windows window controls. Configure
        // this before creating windows so a second system title bar never flashes.
        for window in &mut context.config_mut().app.windows {
            window.decorations = false;
            // Transparent surfaces fail to repaint on the tested NVIDIA/Wayland
            // setup. Keep the known-working opaque surface; rounding is deferred.
            window.transparent = false;
            window.background_color = None;
        }
        context
    };
    // GDK's Wayland app_id defaults to the GLib program name. Set it before
    // GTK initializes so KWin matches the same desktop entry as the taskbar.
    #[cfg(target_os = "linux")]
    {
        glib::set_prgname(Some(&context.config().identifier));
        glib::set_application_name(
            context
                .config()
                .product_name
                .as_deref()
                .unwrap_or("LibreTT"),
        );
    }

    let app = tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            if let Some(window) = app.get_webview_window("main") {
                center_window_controls(&window)?;
                window.with_webview(|webview| {
                    // SAFETY: Tauri supplies a live WKWebView and executes this
                    // callback on the main thread. The pointer is not retained.
                    unsafe {
                        let native = &*webview.inner().cast::<objc2_web_kit::WKWebView>();
                        native.setAllowsBackForwardNavigationGestures(true);
                    }
                })?;
            }
            let directory = app.path().app_data_dir()?;
            std::fs::create_dir_all(&directory)?;
            let repository = SqliteTournamentRepository::open(directory.join("librett.sqlite"))?;
            if let Err(error) = repository.automatic_backup(&directory.join("backups")) {
                eprintln!("Automatic backup failed: {error:?}");
            }
            app.manage(Database(Mutex::new(repository)));
            app.manage(PrintReports(Mutex::new(HashMap::new())));
            Ok(())
        })
        .on_window_event(|window, event| {
            #[cfg(target_os = "macos")]
            if matches!(
                event,
                tauri::WindowEvent::Resized(_) | tauri::WindowEvent::Focused(true)
            ) && !window.is_fullscreen().unwrap_or(false)
            {
                if let Some(webview) = window.get_webview_window("main") {
                    let _ = center_window_controls(&webview);
                }
            }
            #[cfg(not(target_os = "macos"))]
            let _ = (window, event);
        })
        .invoke_handler(tauri::generate_handler![
            list_backups,
            create_backup,
            automatic_backup,
            restore_backup,
            import_backup,
            export_backup,
            open_data_folder,
            open_official_link,
            get_schedule,
            get_match_tables,
            change_schedule,
            save_report,
            open_print_report,
            get_print_report,
            print_report,
            exit_application,
            update_category_with_rules,
            create_category_with_rules,
            get_category_rules,
            save_category_rules,
            get_category_draw,
            preview_category_draw,
            save_category_draw,
            delete_category,
            register_entries,
            cash_ledger,
            settle_player_cash,
            list_cash,
            record_cash,
            registration_started,
            list_tournaments,
            list_trashed_tournaments,
            get_action_history,
            trash_tournament,
            restore_tournament,
            create_tournament_with_cover,
            update_tournament_details,
            get_match_page,
            get_competition_state,
            get_category_report,
            get_category_results,
            get_tournament_progress,
            change_completion,
            save_group_order,
            save_knockout_fillers,
            save_match_result,
            get_category_editor_state,
            add_category,
            list_players,
            get_player,
            delete_player,
            save_player_checked,
            list_entries,
            set_entry_status,
            set_player_attendance,
            register_entry
        ])
        .build(context)
        .expect("Unable to start LibreTT");
    app.run(|app, event| {
        if let tauri::RunEvent::ExitRequested {
            code: None, api, ..
        } = event
        {
            if let Some(window) = app.get_webview_window("main") {
                api.prevent_exit();
                let _ = window.emit("librett-exit-request", ());
            }
        }
    });
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
fn save_player_checked(
    database: tauri::State<Database>,
    request_id: Uuid,
    player_id: Uuid,
    name: String,
    club: String,
    profile: PlayerProfile,
    expected: Option<Player>,
) -> Result<Player, ApplicationError> {
    application::save_player_checked(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        request_id,
        player_id,
        &name,
        &club,
        profile,
        expected,
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

#[tauri::command]
fn set_entry_status(
    database: tauri::State<Database>,
    tournament_id: String,
    entry_id: String,
    status: EntryStatus,
) -> Result<(), ApplicationError> {
    application::set_entry_status(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        Uuid::parse_str(&tournament_id).map_err(|_| ApplicationError::NotFound)?,
        Uuid::parse_str(&entry_id).map_err(|_| ApplicationError::NotFound)?,
        status,
    )
}

#[tauri::command]
fn set_player_attendance(
    database: tauri::State<Database>,
    tournament_id: String,
    player_id: String,
    checked_in: bool,
) -> Result<(), ApplicationError> {
    application::set_player_attendance(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        Uuid::parse_str(&tournament_id).map_err(|_| ApplicationError::NotFound)?,
        Uuid::parse_str(&player_id).map_err(|_| ApplicationError::NotFound)?,
        checked_in,
    )
}

#[tauri::command]
fn list_cash(
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
fn record_cash(
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
fn list_trashed_tournaments(
    database: tauri::State<Database>,
) -> Result<Vec<Tournament>, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .list_trashed_tournaments()
}
#[tauri::command]
fn trash_tournament(database: tauri::State<Database>, id: Uuid) -> Result<(), ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .trash_tournament(id)
}
#[tauri::command]
fn restore_tournament(
    database: tauri::State<Database>,
    id: Uuid,
) -> Result<Tournament, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .restore_tournament(id)
}

#[tauri::command]
fn registration_started(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<bool, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .registration_started(tournament_id, category_id)
}

#[tauri::command]
fn delete_category(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<Tournament, ApplicationError> {
    let mut repo = database.0.lock().map_err(|_| ApplicationError::Storage)?;
    repo.delete_category(tournament_id, category_id)?;
    repo.find(tournament_id)
}
#[tauri::command]
fn register_entries(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
    player_groups: Vec<Vec<Uuid>>,
) -> Result<Vec<Entry>, ApplicationError> {
    application::register_entries(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        category_id,
        player_groups,
    )
}

#[tauri::command]
fn cash_ledger(
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
fn settle_player_cash(
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

#[tauri::command]
fn get_category_draw(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<Option<CategoryDraw>, ApplicationError> {
    application::get_category_draw(
        &*database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        category_id,
    )
}
#[tauri::command]
fn preview_category_draw(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
    mode: DrawMode,
    settings: DrawSettings,
    seeds: Vec<Uuid>,
) -> Result<CategoryDraw, ApplicationError> {
    application::preview_category_draw(
        &*database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        category_id,
        mode,
        settings,
        seeds,
    )
}
#[tauri::command]
fn save_category_draw(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    draw: CategoryDraw,
    expected_revision: u32,
    confirm_restart: Option<bool>,
) -> Result<CategoryDraw, ApplicationError> {
    application::save_category_draw(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        tournament_id,
        draw,
        expected_revision,
        confirm_restart.unwrap_or(false),
    )
}

#[tauri::command]
fn create_category_with_rules(
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
fn get_category_rules(
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
fn save_category_rules(
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
fn update_category_with_rules(
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
fn create_tournament_with_cover(
    database: tauri::State<Database>,
    id: Uuid,
    name: String,
    cover: Option<String>,
) -> Result<Tournament, ApplicationError> {
    application::create_tournament_with_cover(
        &mut *database.0.lock().map_err(|_| ApplicationError::Storage)?,
        id,
        &name,
        cover,
    )
}
#[tauri::command]
fn get_category_editor_state(
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

#[tauri::command]
fn exit_application(app: tauri::AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn update_tournament_details(
    database: tauri::State<Database>,
    id: Uuid,
    name: String,
    cover: Option<String>,
    expected_name: String,
    expected_cover: Option<String>,
) -> Result<Tournament, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .update_tournament_details(id, &name, cover, &expected_name, expected_cover)
}

#[tauri::command]
fn get_match_page(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
    group: usize,
    round: usize,
    page: usize,
    knockout: bool,
) -> Result<application::MatchPage, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .match_page(tournament_id, category_id, group, round, page, knockout)
}
#[tauri::command]
fn save_match_result(
    database: tauri::State<Database>,
    request: application::SaveMatchRequest,
) -> Result<librett_domain::ScheduledMatch, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .save_match_result(request)
}

#[tauri::command]
fn get_competition_state(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<application::CompetitionState, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .competition_state(tournament_id, category_id)
}
#[tauri::command]
fn save_group_order(
    database: tauri::State<Database>,
    request: application::GroupOrderRequest,
) -> Result<application::CompetitionState, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .save_group_order(request)
}

#[tauri::command]
fn save_knockout_fillers(
    database: tauri::State<Database>,
    request: application::SaveFillersRequest,
) -> Result<application::CompetitionState, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .save_knockout_fillers(request)
}

#[tauri::command]
fn get_category_results(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    category_id: Uuid,
) -> Result<application::CategoryResults, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .category_results(tournament_id, category_id)
}
#[tauri::command]
fn get_tournament_progress(
    database: tauri::State<Database>,
    tournament_id: Uuid,
) -> Result<application::TournamentProgress, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .tournament_progress(tournament_id)
}
#[tauri::command]
fn change_completion(
    database: tauri::State<Database>,
    request: application::CompletionRequest,
) -> Result<application::TournamentProgress, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .change_completion(request)
}

fn data_directory(app: &tauri::AppHandle) -> Result<std::path::PathBuf, ApplicationError> {
    app.path()
        .app_data_dir()
        .map_err(|_| ApplicationError::Storage)
}
fn exports_directory(app: &tauri::AppHandle) -> Result<std::path::PathBuf, ApplicationError> {
    let directory = app
        .path()
        .download_dir()
        .or_else(|_| app.path().app_data_dir())
        .map_err(|_| ApplicationError::Storage)?
        .join("LibreTT");
    std::fs::create_dir_all(&directory).map_err(|_| ApplicationError::Storage)?;
    Ok(directory)
}
#[tauri::command]
fn list_backups(app: tauri::AppHandle) -> Result<Vec<application::BackupInfo>, ApplicationError> {
    SqliteTournamentRepository::backups(&data_directory(&app)?.join("backups"))
}
#[tauri::command]
fn create_backup(
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
fn automatic_backup(
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
fn restore_backup(
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
fn import_backup(
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
fn export_backup(
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
// Only project-owned destinations can be opened from the home notice.
#[tauri::command]
fn open_official_link(destination: String) -> Result<(), ApplicationError> {
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
fn open_data_folder(app: tauri::AppHandle, kind: String) -> Result<(), ApplicationError> {
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
#[tauri::command]
fn get_schedule(
    database: tauri::State<Database>,
    tournament_id: Uuid,
) -> Result<application::ScheduleState, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .schedule(tournament_id)
}
#[tauri::command]
fn change_schedule(
    database: tauri::State<Database>,
    request: application::ScheduleRequest,
) -> Result<application::ScheduleState, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .change_schedule(request)
}
#[tauri::command]
fn save_report(
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
fn open_print_report(
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
fn get_print_report(
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
fn print_report(window: tauri::WebviewWindow) -> Result<(), ApplicationError> {
    if !window.label().starts_with("report-") {
        return Err(ApplicationError::NotFound);
    }
    window.print().map_err(|_| ApplicationError::Storage)
}

#[tauri::command]
fn get_match_tables(
    database: tauri::State<Database>,
    tournament_id: Uuid,
    draw_id: Uuid,
) -> Result<HashMap<String, usize>, ApplicationError> {
    database
        .0
        .lock()
        .map_err(|_| ApplicationError::Storage)?
        .match_tables(tournament_id, draw_id)
}

#[tauri::command]
fn get_category_report(
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

#[tauri::command]
fn get_action_history(
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
