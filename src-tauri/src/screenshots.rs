use std::process::Command;
use tauri::AppHandle;

use crate::{app_state::Config, browser, paths};

pub fn capture(app: &AppHandle, cfg: &Config) {
    if which::which("scrot").is_err() {
        tauri::api::dialog::message(Some(&app.get_window("main").unwrap()), "scrot not installed. Install with: sudo apt install scrot");
        return;
    }
    let file = paths::screenshot_file();
    let path_str = file.to_string_lossy().to_string();
    let status = Command::new("scrot").args(["-s", &path_str]).status();
    if status.map(|s| s.success()).unwrap_or(false) {
        let _ = tauri::api::notification::Notification::new(&app.config().tauri.bundle.identifier)
            .title("Screenshot saved")
            .body(&path_str)
            .show();
        browser::open(app, cfg);
    }
}
