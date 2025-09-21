// Contract tests for ConfigManager and HotkeyManager traits
// These tests MUST FAIL initially, then pass after implementation

use std::path::PathBuf;

// Import the contract interfaces
// This will fail until we create the actual config manager module
use ghost_timer::services::config_manager::{ConfigManager, HotkeyManager};
use ghost_timer::models::config::{
    Configuration, ConfigError, ValidationError, HotkeyError, HotkeyInfo,
    DisplayConfig, BehaviorConfig, HotkeyConfig, NotificationConfig, Color
};

#[cfg(test)]
mod config_manager_contract_tests {
    use super::*;

    /// Create a config manager instance for testing
    /// This will fail until ConfigManager is implemented
    fn create_test_config_manager() -> impl ConfigManager {
        ghost_timer::services::config_manager::ConfigManagerImpl::new()
    }

    #[test]
    fn contract_load_nonexistent_config_returns_default() {
        let config_manager = create_test_config_manager();
        
        // If config file doesn't exist, should return default
        let result = config_manager.load();
        
        assert!(result.is_ok(), "Loading nonexistent config should return default");
        let config = result.unwrap();
        let default_config = Configuration::default();
        
        // Should match default values
        assert_eq!(config.version, default_config.version);
        assert_eq!(config.display.transparency, default_config.display.transparency);
        assert_eq!(config.behavior.always_on_top, default_config.behavior.always_on_top);
    }

    #[test]
    fn contract_save_and_load_roundtrip() {
        let config_manager = create_test_config_manager();
        let mut config = Configuration::default();
        
        // Modify some values
        config.display.transparency = 0.5;
        config.display.position = (200, 300);
        config.behavior.always_on_top = false;
        
        // Save the config
        let save_result = config_manager.save(&config);
        assert!(save_result.is_ok(), "Saving config should succeed");
        
        // Load it back
        let load_result = config_manager.load();
        assert!(load_result.is_ok(), "Loading saved config should succeed");
        
        let loaded_config = load_result.unwrap();
        assert_eq!(loaded_config.display.transparency, 0.5);
        assert_eq!(loaded_config.display.position, (200, 300));
        assert_eq!(loaded_config.behavior.always_on_top, false);
    }

    #[test]
    fn contract_validate_valid_config() {
        let config_manager = create_test_config_manager();
        let config = Configuration::default();
        
        let errors = config_manager.validate(&config);
        
        assert!(errors.is_empty(), "Default configuration should be valid");
    }

    #[test]
    fn contract_validate_invalid_transparency() {
        let config_manager = create_test_config_manager();
        let mut config = Configuration::default();
        
        // Set invalid transparency
        config.display.transparency = -0.5;
        
        let errors = config_manager.validate(&config);
        
        assert!(!errors.is_empty(), "Invalid transparency should produce validation errors");
        assert!(errors.iter().any(|e| matches!(e, ValidationError::InvalidTransparency(_))));
    }

    #[test]
    fn contract_validate_invalid_position() {
        let config_manager = create_test_config_manager();
        let mut config = Configuration::default();
        
        // Set unreasonable position (way off screen)
        config.display.position = (50000, 50000);
        
        let errors = config_manager.validate(&config);
        
        assert!(!errors.is_empty(), "Invalid position should produce validation errors");
        assert!(errors.iter().any(|e| matches!(e, ValidationError::InvalidPosition(_, _))));
    }

    #[test]
    fn contract_config_path_points_to_appdata() {
        let config_manager = create_test_config_manager();
        
        let path = config_manager.config_path();
        
        assert!(path.is_absolute(), "Config path should be absolute");
        assert!(path.to_string_lossy().contains("GhostTimer"), "Path should contain app name");
        assert!(path.extension().map_or(false, |ext| ext == "json"), "Config file should be .json");
    }

    #[test]
    fn contract_exists_reflects_file_presence() {
        let config_manager = create_test_config_manager();
        
        // Before saving, file should not exist
        assert!(!config_manager.exists(), "Config file should not exist initially");
        
        // After saving, file should exist
        let config = Configuration::default();
        config_manager.save(&config).expect("Save should succeed");
        assert!(config_manager.exists(), "Config file should exist after saving");
    }

    #[test]
    fn contract_backup_creates_backup_file() {
        let config_manager = create_test_config_manager();
        
        // First save a config
        let config = Configuration::default();
        config_manager.save(&config).expect("Save should succeed");
        
        // Then create backup
        let backup_result = config_manager.backup();
        
        assert!(backup_result.is_ok(), "Backup should succeed when config exists");
        
        // Backup file should exist (we can't easily test this without filesystem access)
        // But the operation should not fail
    }

    #[test]
    fn contract_default_configuration_structure() {
        let default_config = Configuration::default();
        
        // Check version is set
        assert!(!default_config.version.is_empty(), "Version should not be empty");
        
        // Check display defaults
        assert!(default_config.display.transparency >= 0.0 && default_config.display.transparency <= 1.0);
        assert!(default_config.display.hover_transparency >= default_config.display.transparency);
        
        // Check behavior defaults
        assert!(default_config.behavior.always_on_top, "Should default to always on top");
        assert!(default_config.behavior.remember_position, "Should default to remember position");
        
        // Check hotkeys have reasonable defaults
        assert!(default_config.hotkeys.toggle_visibility.is_some(), "Should have default toggle hotkey");
        assert!(default_config.hotkeys.start_stop.is_some(), "Should have default start/stop hotkey");
        
        // Check notifications defaults
        assert!(default_config.notifications.sound_enabled, "Should default to sound enabled");
    }
}

#[cfg(test)]
mod hotkey_manager_contract_tests {
    use super::*;

    /// Create a hotkey manager instance for testing
    /// This will fail until HotkeyManager is implemented
    fn create_test_hotkey_manager() -> impl HotkeyManager {
        ghost_timer::services::hotkey_manager::HotkeyManagerImpl::new()
    }

    #[test]
    fn contract_register_valid_hotkey() {
        let mut hotkey_manager = create_test_hotkey_manager();
        
        let result = hotkey_manager.register_hotkey("Ctrl+Alt+T");
        
        assert!(result.is_ok(), "Registering valid hotkey should succeed");
        let hotkey_id = result.unwrap();
        assert!(hotkey_id > 0, "Hotkey ID should be positive");
    }

    #[test]
    fn contract_register_invalid_hotkey_fails() {
        let mut hotkey_manager = create_test_hotkey_manager();
        
        let result = hotkey_manager.register_hotkey("InvalidKey");
        
        assert!(result.is_err(), "Registering invalid hotkey should fail");
        assert!(matches!(result.unwrap_err(), HotkeyError::InvalidFormat(_)));
    }

    #[test]
    fn contract_register_duplicate_hotkey_fails() {
        let mut hotkey_manager = create_test_hotkey_manager();
        
        // Register first time
        let first_result = hotkey_manager.register_hotkey("Ctrl+Alt+T");
        assert!(first_result.is_ok(), "First registration should succeed");
        
        // Register same hotkey again
        let second_result = hotkey_manager.register_hotkey("Ctrl+Alt+T");
        assert!(second_result.is_err(), "Duplicate registration should fail");
        assert!(matches!(second_result.unwrap_err(), HotkeyError::AlreadyRegistered(_)));
    }

    #[test]
    fn contract_unregister_hotkey() {
        let mut hotkey_manager = create_test_hotkey_manager();
        
        // Register hotkey
        let hotkey_id = hotkey_manager.register_hotkey("Ctrl+Alt+T").expect("Registration should succeed");
        
        // Unregister it
        let result = hotkey_manager.unregister_hotkey(hotkey_id);
        
        assert!(result.is_ok(), "Unregistering valid hotkey should succeed");
    }

    #[test]
    fn contract_unregister_invalid_hotkey_fails() {
        let mut hotkey_manager = create_test_hotkey_manager();
        
        // Try to unregister non-existent hotkey
        let result = hotkey_manager.unregister_hotkey(999);
        
        assert!(result.is_err(), "Unregistering invalid hotkey should fail");
        assert!(matches!(result.unwrap_err(), HotkeyError::NotRegistered(_)));
    }

    #[test]
    fn contract_unregister_all_hotkeys() {
        let mut hotkey_manager = create_test_hotkey_manager();
        
        // Register multiple hotkeys
        hotkey_manager.register_hotkey("Ctrl+Alt+T").expect("First registration should succeed");
        hotkey_manager.register_hotkey("Ctrl+Alt+S").expect("Second registration should succeed");
        
        // Unregister all
        hotkey_manager.unregister_all();
        
        // Should be able to register the same hotkeys again
        let result = hotkey_manager.register_hotkey("Ctrl+Alt+T");
        assert!(result.is_ok(), "Should be able to re-register after unregister_all");
    }

    #[test]
    fn contract_validate_hotkey_strings() {
        let hotkey_manager = create_test_hotkey_manager();
        
        // Valid hotkeys
        assert!(hotkey_manager.validate_hotkey("Ctrl+Alt+T").is_ok(), "Ctrl+Alt+T should be valid");
        assert!(hotkey_manager.validate_hotkey("Shift+F1").is_ok(), "Shift+F1 should be valid");
        assert!(hotkey_manager.validate_hotkey("Ctrl+Space").is_ok(), "Ctrl+Space should be valid");
        
        // Invalid hotkeys
        assert!(hotkey_manager.validate_hotkey("").is_err(), "Empty string should be invalid");
        assert!(hotkey_manager.validate_hotkey("InvalidKey").is_err(), "Invalid key should be invalid");
        assert!(hotkey_manager.validate_hotkey("Ctrl++").is_err(), "Malformed hotkey should be invalid");
    }

    #[test]
    fn contract_parse_hotkey_components() {
        let hotkey_manager = create_test_hotkey_manager();
        
        let result = hotkey_manager.parse_hotkey("Ctrl+Alt+T");
        
        assert!(result.is_ok(), "Parsing valid hotkey should succeed");
        
        let hotkey_info = result.unwrap();
        assert!(hotkey_info.modifiers.contains(&"Ctrl".to_string()), "Should contain Ctrl modifier");
        assert!(hotkey_info.modifiers.contains(&"Alt".to_string()), "Should contain Alt modifier");
        assert_eq!(hotkey_info.key, "T", "Key should be T");
    }

    #[test]
    fn contract_parse_simple_hotkey() {
        let hotkey_manager = create_test_hotkey_manager();
        
        let result = hotkey_manager.parse_hotkey("F1");
        
        assert!(result.is_ok(), "Parsing simple hotkey should succeed");
        
        let hotkey_info = result.unwrap();
        assert!(hotkey_info.modifiers.is_empty(), "Simple hotkey should have no modifiers");
        assert_eq!(hotkey_info.key, "F1", "Key should be F1");
    }

    #[test]
    fn contract_parse_invalid_hotkey_fails() {
        let hotkey_manager = create_test_hotkey_manager();
        
        let result = hotkey_manager.parse_hotkey("InvalidHotkey");
        
        assert!(result.is_err(), "Parsing invalid hotkey should fail");
        assert!(matches!(result.unwrap_err(), HotkeyError::InvalidFormat(_)));
    }
}