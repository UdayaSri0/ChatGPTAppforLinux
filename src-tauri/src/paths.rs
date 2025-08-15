use std::path::PathBuf;
use tauri::AppHandle;

pub fn app_dir(app: &AppHandle) -> PathBuf {
    tauri::api::path::app_data_dir(app.config()).expect("app dir")
}

pub fn config_path(app: &AppHandle) -> PathBuf {
    app_dir(app).join("config.json")
}

pub fn snippets_path(app: &AppHandle) -> PathBuf {
    app_dir(app).join("snippets.json")
}

pub fn screenshots_dir() -> PathBuf {
    dirs::home_dir().unwrap().join("Pictures").join("ChatGPT-Shots")
}

pub fn screenshot_file() -> PathBuf {
    use chrono::Local;
    let dir = screenshots_dir();
    std::fs::create_dir_all(&dir).ok();
    dir.join(format!("{}.png", Local::now().format("%Y%m%d-%H%M%S")))
}
