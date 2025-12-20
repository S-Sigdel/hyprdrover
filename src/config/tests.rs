#[cfg(test)]
mod tests {
    use crate::config::Config;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_config_defaults() {
        let config = Config::default();

        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let expected_dir = PathBuf::from(home)
            .join(".config")
            .join("hyprdrover")
            .join("sessions")
            .to_string_lossy()
            .into_owned();

        assert_eq!(config.session_dir, expected_dir);
        assert!(config.ignored_classes.contains(&"rofi".to_string()));
        assert!(config.ignored_classes.contains(&"waybar".to_string()));
    }
}
