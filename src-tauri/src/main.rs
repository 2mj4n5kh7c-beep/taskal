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
        .map_err(|e| {
            if e.is_timeout() { "Your university didn't answer in time. Try again in a minute.".to_string() }
            else { format!("Couldn't reach your university ({}). Check your internet connection.", e.without_url()) }
        })?;
    let status = resp.status();
    if !status.is_success() {
        return Err(format!("HTTP {}", status.as_u16()));
    }
    resp.text().await.map_err(|e| e.without_url().to_string())
}

/// Reads a university course catalog page (timetables, groups, exam dates).
/// Only the catalog sites below can be reached, and the reply comes back as raw bytes (base64),
/// because some universities still use Hebrew page encodings rather than UTF-8.
const CATALOG_HOSTS: &[&str] = &[
    "shnaton.huji.ac.il",
    "sap.cheesefork.cf",
    "arazim-project.com",
    "courses.biu.ac.il",
    "bgu4u.bgu.ac.il",
    "www.runi.ac.il",
];

static CATALOG_CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();

#[derive(serde::Serialize)]
struct CatalogReply {
    status: u16,
    url: String,
    body: String,
}

#[tauri::command]
async fn catalog_fetch(
    url: String,
    method: Option<String>,
    form: Option<Vec<(String, String)>>,
) -> Result<CatalogReply, String> {
    use base64::Engine as _;
    let parsed = reqwest::Url::parse(&url).map_err(|e| e.to_string())?;
    let host = parsed.host_str().unwrap_or("");
    if parsed.scheme() != "https" || !CATALOG_HOSTS.contains(&host) {
        return Err("That catalog isn't supported.".into());
    }
    let client = CATALOG_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Taskal course catalog lookup)")
            .cookie_store(true)
            .timeout(std::time::Duration::from_secs(40))
            .build()
            .expect("http client")
    });
    let req = if method.as_deref() == Some("POST") {
        client.post(parsed).form(&form.unwrap_or_default())
    } else {
        client.get(parsed)
    };
    let resp = req.send().await.map_err(|e| {
        if e.is_timeout() { "The course catalog didn't answer in time. Try again in a minute.".to_string() }
        else { format!("Couldn't reach the course catalog ({}).", e.without_url()) }
    })?;
    let status = resp.status().as_u16();
    let final_url = resp.url().to_string();
    let bytes = resp.bytes().await.map_err(|e| e.without_url().to_string())?;
    Ok(CatalogReply { status, url: final_url, body: base64::engine::general_purpose::STANDARD.encode(&bytes) })
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
        .invoke_handler(tauri::generate_handler![fetch_calendar, catalog_fetch])
        .setup(|app| {
            // Windows: a tray icon so Taskal can keep running (and reminding) with its window closed.
            #[cfg(not(target_os = "macos"))]
            {
                use tauri::menu::{Menu, MenuItem};
                use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
                let open = MenuItem::with_id(app, "open", "Open Taskal 2", true, None::<&str>)?;
                let quit = MenuItem::with_id(app, "quit", "Quit Taskal 2", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&open, &quit])?;
                TrayIconBuilder::new()
                    .icon(app.default_window_icon().unwrap().clone())
                    .tooltip("Taskal 2")
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
