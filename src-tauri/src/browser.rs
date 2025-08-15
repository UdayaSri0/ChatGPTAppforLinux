use std::process::Command;
use tauri::AppHandle;

use crate::{app_state::Config, focus};

pub fn open(app: &AppHandle, cfg: &Config) {
    if focus::focus_existing().is_ok() {
        return;
    }
    for b in &cfg.browser_candidates {
        if which::which(b).is_ok() {
            let _ = Command::new(b)
                .args([format!("--app={}", cfg.chat_url), String::from("--new-window")])
                .spawn();
            return;
        }
    }
    let _ = Command::new("xdg-open").arg(&cfg.chat_url).spawn();
    let _ = tauri::api::notification::Notification::new(&app.config().tauri.bundle.identifier)
        .title("ChatGPT Shell")
        .body("No preferred browser found, used xdg-open")
        .show();
}
