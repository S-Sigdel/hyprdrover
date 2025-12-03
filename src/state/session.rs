use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use chrono::Local;
use crate::config::Config;
use crate::ipc::{self, SessionSnapshot};
use crate::snapshot;

pub struct SessionManager {
    config: Config,
}

impl SessionManager {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Capture the current state, filtering out ignored windows
    pub fn snapshot(&self) -> Result<PathBuf, Box<dyn Error>> {
        // 1. Capture raw state from Hyprland
        let mut state = ipc::capture_state()?;

        // 2. Filter out ignored classes (like rofi, waybar)
        let original_count = state.clients.len();
        state.clients.retain(|client| {
            !self.config.ignored_classes.contains(&client.class)
        });
        let filtered_count = original_count - state.clients.len();

        if filtered_count > 0 {
            println!("Filtered out {} ignored windows.", filtered_count);
        }

        // 3. Save to disk
        self.save_to_disk(&state)
    }

    fn save_to_disk(&self, snapshot: &SessionSnapshot) -> Result<PathBuf, Box<dyn Error>> {
        let session_dir = Path::new(&self.config.session_dir);
        if !session_dir.exists() {
            fs::create_dir_all(session_dir)?;
        }

        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let filename = format!("session_{}.json", timestamp);
        let file_path = session_dir.join(&filename);

        let json_string = serde_json::to_string_pretty(snapshot)?;
        fs::write(&file_path, json_string)?;

        Ok(file_path)
    }

    pub fn list_sessions(&self) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let session_dir = Path::new(&self.config.session_dir);
        if !session_dir.exists() {
            return Ok(vec![]);
        }

        let mut sessions = vec![];
        for entry in fs::read_dir(session_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                sessions.push(path);
            }
        }
        // Sort by name (timestamp) descending
        sessions.sort_by(|a, b| b.cmp(a));
        Ok(sessions)
    }
}
