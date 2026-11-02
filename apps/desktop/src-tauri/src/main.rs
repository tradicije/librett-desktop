#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use librett_storage_sqlite::SqliteTournamentRepository;
use state::{Database, PrintReports};
use std::{collections::HashMap, sync::Mutex};
use tauri::{Emitter, Manager};

mod commands;
mod registry_http;
mod state;

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
            commands::registry::preview_registry,
            commands::registry::refine_registry,
            commands::registry::confirm_registry_import,
            commands::registry::registry_sources,
            commands::registry::registry_previews,
            commands::registry::cancel_registry_preview,
            commands::registry::download_registry_photo,
            commands::backups::list_backups,
            commands::backups::create_backup,
            commands::backups::automatic_backup,
            commands::backups::restore_backup,
            commands::backups::import_backup,
            commands::backups::export_backup,
            commands::system::open_data_folder,
            commands::system::open_official_link,
            commands::scheduling::get_schedule,
            commands::scheduling::get_match_tables,
            commands::scheduling::change_schedule,
            commands::reports::save_report,
            commands::reports::open_print_report,
            commands::reports::get_print_report,
            commands::reports::print_report,
            commands::system::exit_application,
            commands::categories::update_category_with_rules,
            commands::categories::create_category_with_rules,
            commands::categories::get_category_rules,
            commands::categories::save_category_rules,
            commands::draw::get_category_draw,
            commands::draw::preview_category_draw,
            commands::draw::save_category_draw,
            commands::categories::delete_category,
            commands::registrations::register_entries,
            commands::cash::cash_ledger,
            commands::cash::settle_player_cash,
            commands::cash::list_cash,
            commands::cash::record_cash,
            commands::registrations::registration_started,
            commands::tournaments::list_tournaments,
            commands::tournaments::list_trashed_tournaments,
            commands::history::get_action_history,
            commands::tournaments::trash_tournament,
            commands::tournaments::restore_tournament,
            commands::tournaments::create_tournament_with_cover,
            commands::tournaments::update_tournament_details,
            commands::matches::get_match_page,
            commands::draw::get_competition_state,
            commands::reports::get_category_report,
            commands::results::get_category_results,
            commands::results::get_tournament_progress,
            commands::results::change_completion,
            commands::draw::save_group_order,
            commands::draw::save_knockout_fillers,
            commands::matches::save_match_result,
            commands::categories::get_category_editor_state,
            commands::categories::add_category,
            commands::players::list_players,
            commands::players::get_player,
            commands::players::delete_player,
            commands::players::save_player_checked,
            commands::registrations::list_entries,
            commands::registrations::set_entry_status,
            commands::registrations::set_player_attendance,
            commands::registrations::register_entry
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
