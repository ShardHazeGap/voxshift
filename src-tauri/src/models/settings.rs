//! Persisted application state.
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
    pub output_dir: Option<String>,
    pub copy_to_clipboard: bool,
    pub parallel_jobs: u32,
    pub keep_history: u32,
}

impl Settings {
    pub fn path() -> Result<PathBuf, String> {
        let base = std::env::var("APPDATA")
            .or_else(|_| std::env::var("HOME"))
            .map_err(|_| "no home".to_string())?;
        let dir = PathBuf::from(base).join("fitclip");
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        Ok(dir.join("settings.json"))
    }

    pub fn load() -> Settings {
        Self::path().ok()
            .and_then(|p| fs::read_to_string(p).ok())
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let p = Self::path()?;
        let s = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(p, s).map_err(|e| e.to_string())
    }
}
