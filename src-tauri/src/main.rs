// Hide the extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, RunEvent, WindowEvent};

/// Downloads a calendar (.ics) from the user's university.
/// Done natively so the browser's cross-site rules (CORS) don't block it.
#[tauri::command]
async fn fetch_calendar(url: String) -> Result<String, String> {
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err("The link must start with https://".into());
    }
    let client = reqwest::Client::builder()
        .user_agent("Taskal (calendar sync)")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Couldn't reach your university: {e}"))?;
    let status = resp.status();
    if !status.is_success() {
        return Err(format!("HTTP {}", status.as_u16()));
    }
    resp.text().await.map_err(|e| e.to_string())
}

fn show_main(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--background"]),
        ))
        .invoke_handler(tauri::generate_handler![fetch_calendar])
        .setup(|app| {
            // Windows: a tray icon so Taskal can keep running (and reminding) with its window closed.
            #[cfg(not(target_os = "macos"))]
            {
                use tauri::menu::{Menu, MenuItem};
                use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
                let open = MenuItem::with_id(app, "open", "Open Taskal", true, None::<&str>)?;
                let quit = MenuItem::with_id(app, "quit", "Quit Taskal", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&open, &quit])?;
                TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .tooltip("Taskal")
                    .menu(&menu)
                    .show_menu_on_left_click(false)
                    .on_menu_event(|app, e| match e.id.as_ref() {
                        "open" => show_main(app),
                        "quit" => app.exit(0),
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, e| {
                        if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = e {
                            show_main(tray.app_handle());
                        }
                    })
                    .build(app)?;
            }
            // Started automatically at login: stay out of the way, just keep reminders running.
            if std::env::args().any(|a| a == "--background") {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.hide();
                }
            }
            Ok(())
        })
        // Closing the window hides it instead of quitting, so reminders keep coming.
        .on_window_event(|win, ev| {
            if let WindowEvent::CloseRequested { api, .. } = ev {
                api.prevent_close();
                let _ = win.hide();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while running Taskal");

    app.run(|_handle, _ev| {
        // macOS: clicking the Dock icon brings the window back.
        #[cfg(target_os = "macos")]
        if let RunEvent::Reopen { .. } = _ev {
            show_main(_handle);
        }
    });
}
