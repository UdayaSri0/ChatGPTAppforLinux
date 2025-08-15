#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod browser;
mod focus;
mod global_hotkeys;
mod paths;
mod screenshots;
mod tray;

use app_state::AppState;
use tauri::{Manager};

#[tauri::command]
fn open_chatgpt(app: tauri::AppHandle) {
    let cfg = app.state::<AppState>().config.lock().unwrap().clone();
    browser::open(&app, &cfg);
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            if let Some(w) = app.get_window("main") {
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
        .setup(|app| {
            let state = AppState::load(app);
            app.manage(state);
            global_hotkeys::register(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_chatgpt])
        .system_tray(tray::create())
        .on_system_tray_event(tray::handler)
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
