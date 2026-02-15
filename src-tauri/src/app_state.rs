use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::AppHandle;

use crate::paths;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Hotkeys {
    pub open_chat: String,
    pub quick_prompt: String,
    pub screenshot: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
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
                    if let Err(e) = std::fs::create_dir_all(parent) {
                        eprintln!("failed to create config directory: {e}");
                    }
                }
                if let Err(e) = std::fs::copy(res, &path) {
                    eprintln!("failed to copy default config: {e}");
                }
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
        let config: Config = serde_json::from_str(&data)
            .unwrap_or_else(|e| {
                eprintln!("failed to parse config: {e}");
                Config::default()
            })
            .normalize();
        Self {
            config: Mutex::new(config),
        }
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
            chat_url: "https://chatgpt.com".into(),
            hotkeys: Hotkeys::default(),
            browser_candidates: vec![
                "google-chrome".into(),
                "chromium".into(),
                "brave-browser".into(),
            ],
        }
    }
}

impl Config {
    fn normalize(mut self) -> Self {
        let defaults = Config::default();

        if self.chat_url.trim().is_empty() {
            self.chat_url = defaults.chat_url.clone();
        }
        if self.hotkeys.open_chat.trim().is_empty() {
            self.hotkeys.open_chat = defaults.hotkeys.open_chat.clone();
        }
        if self.hotkeys.quick_prompt.trim().is_empty() {
            self.hotkeys.quick_prompt = defaults.hotkeys.quick_prompt.clone();
        }
        if self.hotkeys.screenshot.trim().is_empty() {
            self.hotkeys.screenshot = defaults.hotkeys.screenshot.clone();
        }

        self.browser_candidates = self
            .browser_candidates
            .into_iter()
            .filter(|candidate| !candidate.trim().is_empty())
            .collect();
        if self.browser_candidates.is_empty() {
            self.browser_candidates = defaults.browser_candidates;
        }

        self
    }
}
