use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, AppHandle, Manager};

use crate::{app_state::AppState, browser, screenshots};

pub fn create() -> SystemTray {
    let open = CustomMenuItem::new("open_chatgpt", "Open ChatGPT");
    let prompt = CustomMenuItem::new("quick_prompt", "Quick Prompt");
    let shot = CustomMenuItem::new("screenshot", "Screenshot to File");
    let quit = CustomMenuItem::new("quit", "Quit");
    let menu = SystemTrayMenu::new()
        .add_item(open)
        .add_item(prompt)
        .add_item(shot)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(menu)
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        let cfg = app.state::<AppState>().config.lock().unwrap().clone();
        match id.as_str() {
            "open_chatgpt" => browser::open(app, &cfg),
            "quick_prompt" => {
                if let Some(w) = app.get_window("main") { let _ = w.show(); let _ = w.set_focus(); }
            }
            "screenshot" => screenshots::capture(app, &cfg),
            "quit" => app.exit(0),
            _ => {}
        }
    }
}
