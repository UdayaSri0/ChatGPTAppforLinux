use std::{env, process::Command};
use tauri::AppHandle;

use crate::{app_state::Config, browser, paths};

pub fn capture(app: &AppHandle, cfg: &Config) {
    if !command_exists("scrot") {
        let win = app.get_window("main");
        tauri::api::dialog::message(win.as_ref(), "scrot not installed. Install with: sudo apt install scrot");
        return;
    }
    let file = paths::screenshot_file();
    let path_str = file.to_string_lossy().to_string();
    match Command::new("scrot").args(["-s", &path_str]).status() {
        Ok(status) if status.success() => {
            if let Err(e) = tauri::api::notification::Notification::new(&app.config().tauri.bundle.identifier)
                .title("Screenshot saved")
                .body(&path_str)
                .show()
            {
                eprintln!("failed to show notification: {e}");
            }
            browser::open(app, cfg);
        }
        _ => {
            eprintln!("failed to capture screenshot");
            let win = app.get_window("main");
            tauri::api::dialog::message(win.as_ref(), "Failed to capture screenshot");
        }
    }
}

fn command_exists(cmd: &str) -> bool {
    env::var_os("PATH").map_or(false, |paths| {
        env::split_paths(&paths).any(|p| p.join(cmd).exists())
    })
}
