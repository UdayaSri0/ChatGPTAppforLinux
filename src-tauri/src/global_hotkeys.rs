use tauri::{AppHandle, Manager};

use crate::{app_state::AppState, browser, screenshots};

pub fn register(app: &AppHandle) {
    let cfg = app.state::<AppState>().config.lock().unwrap().clone();
    let open_app = app.clone();
    if let Err(_) = app.global_shortcut_manager().register(cfg.hotkeys.open_chat.clone(), move || {
        let cfg = open_app.state::<AppState>().config.lock().unwrap().clone();
        browser::open(&open_app, &cfg);
    }) {
        if let Some(win) = app.get_window("main") {
            tauri::api::dialog::message(Some(&win), "Failed to register open shortcut");
        }
    }
    let prompt_app = app.clone();
    if let Err(_) = app.global_shortcut_manager().register(cfg.hotkeys.quick_prompt.clone(), move || {
        if let Some(w) = prompt_app.get_window("main") {
            let vis = w.is_visible().unwrap_or(false);
            if vis { let _ = w.hide(); } else { let _ = w.show(); let _ = w.set_focus(); }
        }
    }) {
        if let Some(win) = app.get_window("main") {
            tauri::api::dialog::message(Some(&win), "Failed to register prompt shortcut");
        }
    }
    let shot_app = app.clone();
    if let Err(_) = app.global_shortcut_manager().register(cfg.hotkeys.screenshot.clone(), move || {
        let cfg = shot_app.state::<AppState>().config.lock().unwrap().clone();
        screenshots::capture(&shot_app, &cfg);
    }) {
        if let Some(win) = app.get_window("main") {
            tauri::api::dialog::message(Some(&win), "Failed to register screenshot shortcut");
        }
    }
}
