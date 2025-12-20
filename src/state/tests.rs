#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::state::SessionManager;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_session_manager_creation() {
        let config = Config::default();
        let _manager = SessionManager::new(config);
    }

    #[test]
    fn test_list_sessions_empty() {
        // Use a temp dir for testing to avoid messing with real sessions
        let temp_dir = "test_sessions_empty";
        if Path::new(temp_dir).exists() {
            fs::remove_dir_all(temp_dir).unwrap();
        }

        let mut config = Config::default();
        config.session_dir = temp_dir.to_string();

        let manager = SessionManager::new(config);
        let sessions = manager.list_sessions().unwrap();

        assert_eq!(sessions.len(), 0);

        // Cleanup
        if Path::new(temp_dir).exists() {
            fs::remove_dir_all(temp_dir).unwrap();
        }
    }
}
