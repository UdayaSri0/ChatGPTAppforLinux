#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_state;
mod browser;
mod focus;
mod global_hotkeys;
mod paths;
mod quick_prompt;
mod screenshots;
mod tray;

use app_state::AppState;
use single_instance::SingleInstance;
use tauri::Manager;

#[tauri::command]
fn open_chatgpt(app: tauri::AppHandle) {
    let cfg = app.state::<AppState>().config.lock().unwrap().clone();
    browser::open(&app, &cfg);
}

fn main() {
    let single_instance_guard = SingleInstance::new("com.example.chatgptnative")
        .expect("failed to initialize single instance guard");
    if !single_instance_guard.is_single() {
        let _ = focus::focus_existing();
        return;
    }

    tauri::Builder::default()
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
