use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::AppHandle;

use crate::paths;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hotkeys {
    pub open_chat: String,
    pub quick_prompt: String,
    pub screenshot: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub chat_url: String,
    pub hotkeys: Hotkeys,
    pub browser_candidates: Vec<String>,
}

pub struct AppState {
    pub config: Mutex<Config>,
}

impl AppState {
    pub fn load(app: &AppHandle) -> Self {
        let path = paths::config_path(app);
        if !path.exists() {
            if let Some(res) = app.path_resolver().resolve_resource("config.json") {
                std::fs::create_dir_all(path.parent().unwrap()).ok();
                let _ = std::fs::copy(res, &path);
            }
        }
        let data = std::fs::read_to_string(&path).expect("config");
        let config: Config = serde_json::from_str(&data).expect("parse config");
        Self { config: Mutex::new(config) }
    }
}
