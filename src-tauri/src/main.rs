//! VoxShift — Tauri backend.
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub version: String,
    pub autosave: bool,
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { version: "1.0.0".into(), autosave: true, theme: "dark".into() }
    }
}

#[tauri::command]
fn load_config() -> Result<AppConfig, String> {
    let path = config_path()?;
    let raw = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&raw).or_else(|_| Ok(AppConfig::default()))
}

#[tauri::command]
fn save_config(cfg: AppConfig) -> Result<(), String> {
    let path = config_path()?;
    let s = serde_json::to_string_pretty(&cfg).map_err(|e| e.to_string())?;
    fs::write(path, s).map_err(|e| e.to_string())
}

fn config_path() -> Result<PathBuf, String> {
    let base = std::env::var("APPDATA").map_err(|_| "no appdata".to_string())?;
    let dir = PathBuf::from(base).join("voxshift");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("config.json"))
}

#[tauri::command]
fn app_version() -> String { "1.0.0".into() }

#[tauri::command]
fn platform_info() -> String {
    #[cfg(target_os = "windows")]
    { "windows".into() }
    #[cfg(target_os = "macos")]
    { "macos".into() }
    #[cfg(target_os = "linux")]
    { "linux".into() }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_config, save_config, app_version, platform_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
