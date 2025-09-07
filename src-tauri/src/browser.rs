use std::{env, process::Command};
use tauri::AppHandle;

use crate::{app_state::Config, focus};

pub fn open(app: &AppHandle, cfg: &Config) {
    if focus::focus_existing().is_ok() {
        return;
    }
    for b in &cfg.browser_candidates {
        if command_exists(b) {
            if let Err(e) = Command::new(b)
                .args([format!("--app={}", cfg.chat_url), String::from("--new-window")])
                .spawn()
            {
                eprintln!("failed to open browser {b}: {e}");
                if let Some(win) = app.get_window("main") {
                    tauri::api::dialog::message(Some(&win), "Failed to launch browser");
                }
            }
            return;
        }
    }
    match Command::new("xdg-open").arg(&cfg.chat_url).spawn() {
        Ok(_) => {
            if let Err(e) = tauri::api::notification::Notification::new(&app.config().tauri.bundle.identifier)
                .title("ChatGPT Shell")
                .body("No preferred browser found, used xdg-open")
                .show()
            {
                eprintln!("failed to show notification: {e}");
            }
        }
        Err(e) => {
            eprintln!("failed to spawn xdg-open: {e}");
            if let Some(win) = app.get_window("main") {
                tauri::api::dialog::message(Some(&win), "Failed to launch browser");
            }
        }
    }
}

fn command_exists(cmd: &str) -> bool {
    env::var_os("PATH").map_or(false, |paths| {
        env::split_paths(&paths).any(|p| p.join(cmd).exists())
    })
}
