use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::AppHandle;

use crate::paths;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hotkeys {
    pub open_chat: String,
    pub quick_prompt: String,
    pub screenshot: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent).ok();
                }
                let _ = std::fs::copy(res, &path);
            }
        }
        let data = match std::fs::read_to_string(&path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("failed to read config: {e}");
                return Self {
                    config: Mutex::new(Config::default()),
                };
            }
        };
        let config: Config = serde_json::from_str(&data).unwrap_or_else(|e| {
            eprintln!("failed to parse config: {e}");
            Config::default()
        });
        Self { config: Mutex::new(config) }
    }
}

impl Default for Hotkeys {
    fn default() -> Self {
        Self {
            open_chat: "Ctrl+Space".into(),
            quick_prompt: "Ctrl+Shift+P".into(),
            screenshot: "Ctrl+Shift+S".into(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            chat_url: "https://chat.openai.com".into(),
            hotkeys: Hotkeys::default(),
            browser_candidates: vec![
                "google-chrome".into(),
                "chromium".into(),
                "brave-browser".into(),
            ],
        }
    }
}
