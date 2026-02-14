use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

use crate::{app_state::AppState, browser, quick_prompt, screenshots};

pub fn create() -> SystemTray {
    let open = CustomMenuItem::new("open_chatgpt", "Open ChatGPT in Browser");
    let prompt = CustomMenuItem::new("toggle_quick_prompt", "Toggle Quick Prompt");
    let shot = CustomMenuItem::new("screenshot", "Capture Screenshot to File");
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
            "toggle_quick_prompt" => quick_prompt::toggle(app),
            "screenshot" => screenshots::capture(app, &cfg),
            "quit" => app.exit(0),
            _ => {}
        }
    }
}
