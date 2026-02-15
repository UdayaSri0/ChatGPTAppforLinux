use tauri::{AppHandle, Manager};

use crate::app_state::Config;

pub fn open(app: &AppHandle, _cfg: &Config) {
    if let Some(window) = app.get_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    eprintln!("main window not found");
}
