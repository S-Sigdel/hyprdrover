use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub session_dir: String,
    pub ignored_classes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let session_dir = PathBuf::from(home)
            .join(".config")
            .join("hyprdrover")
            .join("sessions");

        Self {
            session_dir: session_dir.to_string_lossy().into_owned(),
            // Don't snapshot these background/overlay apps
            ignored_classes: vec![
                "rofi".to_string(),
                "waybar".to_string(),
                "dunst".to_string(),
                "hyprland-share-picker".to_string(),
                "polkit-gnome-authentication-agent-1".to_string(),
            ],
        }
    }
}
