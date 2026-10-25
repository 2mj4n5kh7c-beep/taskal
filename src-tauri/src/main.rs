// Hide the extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch_calendar])
        .run(tauri::generate_context!())
        .expect("error while running Taskal");
}
