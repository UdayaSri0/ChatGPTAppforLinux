use tauri::{AppHandle, Manager};

pub fn toggle(app: &AppHandle) {
    if let Some(window) = app.get_window("main") {
        let is_visible = window.is_visible().unwrap_or(false);
        if is_visible {
            let _ = window.hide();
        } else {
            let _ = window.unminimize();
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}
