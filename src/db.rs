use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub command: String,
    pub run_count: u32,
    pub last_run: DateTime<Utc>,
    pub last_dir: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub version: u32,
    pub entries: Vec<Entry>,
}

impl Database {
    fn db_path() -> std::path::PathBuf {
        let home = std::env::var("HOME").unwrap_or_default();
        std::path::Path::new(&home).join(".local/share/smart-run/db.json")
    }
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::db_path();
        if !path.exists() {
            return Ok(Self {
                version: 1,
                entries: Vec::new(),
            });
        }
        let contents = std::fs::read_to_string(&path)?;
        let db = serde_json::from_str(&contents)?;
        Ok(db)
    }
    pub fn save(&self) {}
    pub fn add(&mut self, command: String, current_dir: Option<String>) {}
    pub fn sorted_entries(&self, current_dir: Option<&str>) -> Vec<&Entry> {}
}
