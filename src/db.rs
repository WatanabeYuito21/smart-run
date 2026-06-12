use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub command: String,
    pub run_count: u32,
    pub last_run: DateTime<Utc>,
    pub last_dir: Option<String>,
}

impl Entry {
    pub fn score(&self, current_dir: Option<&str>) -> f64 {
        let elapsed_hours = Utc::now()
            .signed_duration_since(self.last_run)
            .num_seconds() as f64
            / 3600.0;
        let time_decay = 1.0 / (1.0 + elapsed_hours * 0.01);
        let context_boost = match (current_dir, &self.last_dir) {
            (Some(cur), Some(last)) if cur == last => 1.5,
            _ => 1.0,
        };
        self.run_count as f64 * time_decay * context_boost
    }
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub version: u32,
    pub entries: Vec<Entry>,
}

impl Database {
    fn db_path() -> std::path::PathBuf {
        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA").unwrap_or_default();
            std::path::Path::new(&appdata).join("smart-run\\db.json")
        }
        #[cfg(not(target_os = "windows"))]
        {
            let home = std::env::var("HOME").unwrap_or_default();
            std::path::Path::new(&home).join(".local/share/smart-run/db.json")
        }
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

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::db_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(&self)?;
        std::fs::write(&path, contents)?;
        Ok(())
    }

    pub fn add(&mut self, command: String, current_dir: Option<String>) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.command == command) {
            entry.run_count += 1;
            entry.last_run = Utc::now();
            entry.last_dir = current_dir;
        } else {
            self.entries.push(Entry {
                command,
                run_count: 1,
                last_run: Utc::now(),
                last_dir: current_dir,
            });
        }
    }

    pub fn sorted_entries(&self, current_dir: Option<&str>) -> Vec<&Entry> {
        let mut entries: Vec<&Entry> = self.entries.iter().collect();
        entries.sort_by(|a, b| {
            b.score(current_dir)
                .partial_cmp(&a.score(current_dir))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        entries
    }
}
