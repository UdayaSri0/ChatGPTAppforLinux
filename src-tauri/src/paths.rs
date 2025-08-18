use std::path::PathBuf;
use tauri::AppHandle;

pub fn app_dir(app: &AppHandle) -> PathBuf {
    tauri::api::path::app_data_dir(app.config()).unwrap_or_else(|| {
        eprintln!("failed to get app data dir, using current dir");
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    })
}

pub fn config_path(app: &AppHandle) -> PathBuf {
    app_dir(app).join("config.json")
}

pub fn snippets_path(app: &AppHandle) -> PathBuf {
    app_dir(app).join("snippets.json")
}

pub fn screenshots_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join("Pictures").join("ChatGPT-Shots")
}

pub fn screenshot_file() -> PathBuf {
    use chrono::Local;
    let dir = screenshots_dir();
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("failed to create screenshot directory: {e}");
    }
    dir.join(format!("{}.png", Local::now().format("%Y%m%d-%H%M%S")))
}
