use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub session_dir: String,
    pub ignored_classes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            session_dir: "sessions".to_string(),
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
