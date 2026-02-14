use std::collections::HashMap;

use tauri::{AppHandle, Manager};

use crate::{app_state::AppState, browser, quick_prompt, screenshots};

pub fn register(app: &AppHandle) {
    let cfg = app.state::<AppState>().config.lock().unwrap().clone();
    let mut seen_accelerators: HashMap<String, String> = HashMap::new();
    let mut conflicts: Vec<String> = Vec::new();

    let open_app = app.clone();
    register_shortcut(
        app,
        &mut seen_accelerators,
        &mut conflicts,
        "Open ChatGPT",
        &cfg.hotkeys.open_chat,
        "Failed to register open shortcut",
        move || {
            let cfg = open_app.state::<AppState>().config.lock().unwrap().clone();
            browser::open(&open_app, &cfg);
        },
    );

    let prompt_app = app.clone();
    register_shortcut(
        app,
        &mut seen_accelerators,
        &mut conflicts,
        "Toggle Quick Prompt",
        &cfg.hotkeys.quick_prompt,
        "Failed to register prompt shortcut",
        move || quick_prompt::toggle(&prompt_app),
    );

    let shot_app = app.clone();
    register_shortcut(
        app,
        &mut seen_accelerators,
        &mut conflicts,
        "Capture Screenshot",
        &cfg.hotkeys.screenshot,
        "Failed to register screenshot shortcut",
        move || {
            let cfg = shot_app.state::<AppState>().config.lock().unwrap().clone();
            screenshots::capture(&shot_app, &cfg);
        },
    );

    if !conflicts.is_empty() {
        let details = format!("Ignored conflicting shortcuts:\n{}", conflicts.join("\n"));
        if let Some(win) = app.get_window("main") {
            tauri::api::dialog::message(Some(&win), &details);
        } else {
            eprintln!("{details}");
        }
    }
}

fn register_shortcut<F>(
    app: &AppHandle,
    seen_accelerators: &mut HashMap<String, String>,
    conflicts: &mut Vec<String>,
    action_name: &str,
    accelerator: &str,
    register_error: &str,
    on_trigger: F,
) where
    F: Fn() + Send + 'static,
{
    if accelerator.trim().is_empty() {
        conflicts.push(format!("{action_name}: empty shortcut"));
        return;
    }

    let normalized = normalize_accelerator(accelerator);
    if let Some(existing) = seen_accelerators.get(&normalized) {
        conflicts.push(format!(
            "{action_name} ({accelerator}) skipped; already used by {existing}"
        ));
        return;
    }
    seen_accelerators.insert(normalized, action_name.to_string());

    if app
        .global_shortcut_manager()
        .register(accelerator.to_string(), on_trigger)
        .is_err()
    {
        if let Some(win) = app.get_window("main") {
            tauri::api::dialog::message(Some(&win), register_error);
        }
    }
}

fn normalize_accelerator(value: &str) -> String {
    value
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}
