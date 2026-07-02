use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub player_name: String,
    pub time_minutes: u64,       // starting time per player
    pub ai_level: Option<u8>,    // None for two-player, Some(1-5)
    pub port: u16,
    pub theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            player_name: "Player".into(),
            time_minutes: 10,
            ai_level: Some(1),
            port: 23456,
            theme: "tokio_night".into(),
        }
    }
}

impl Config {
    fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("chess-tui");
        std::fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
