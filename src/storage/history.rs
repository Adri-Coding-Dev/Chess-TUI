use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub date: DateTime<Utc>,
    pub white: String,
    pub black: String,
    pub result: String,
    pub moves: Vec<String>,
}

pub fn load_history() -> Result<Vec<HistoryEntry>> {
    let path = history_path();
    if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(Vec::new())
    }
}

pub fn save_history(history: &[HistoryEntry]) -> Result<()> {
    let path = history_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(history)?;
    std::fs::write(path, content)?;
    Ok(())
}

fn history_path() -> std::path::PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push("chess-tui");
    std::fs::create_dir_all(&path).ok();
    path.push("history.json");
    path
}
