#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_config_defaults() {
        let config = Config::default();
        assert_eq!(config.session_dir, "sessions");
        assert!(config.ignored_classes.contains(&"rofi".to_string()));
        assert!(config.ignored_classes.contains(&"waybar".to_string()));
    }
}
