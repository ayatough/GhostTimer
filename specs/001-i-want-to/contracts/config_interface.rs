// Configuration Management Interface Contract
// This file defines the interface for loading, saving, and validating user preferences

use serde::{Deserialize, Serialize};

/// Configuration persistence interface
pub trait ConfigManager {
    /// Load configuration from disk
    /// Returns default configuration if file doesn't exist or is invalid
    fn load(&self) -> Result<Configuration, ConfigError>;
    
    /// Save configuration to disk
    /// Creates directory structure if needed
    fn save(&self, config: &Configuration) -> Result<(), ConfigError>;
    
    /// Validate configuration values
    /// Returns list of validation errors
    fn validate(&self, config: &Configuration) -> Vec<ValidationError>;
    
    /// Get default configuration
    fn default() -> Configuration;
    
    /// Get configuration file path
    fn config_path(&self) -> std::path::PathBuf;
    
    /// Check if configuration file exists
    fn exists(&self) -> bool;
    
    /// Create backup of current configuration
    fn backup(&self) -> Result<(), ConfigError>;
}

/// Hotkey registration and handling
pub trait HotkeyManager {
    /// Register a global hotkey
    /// Returns hotkey ID for later unregistration
    fn register_hotkey(&mut self, keys: &str) -> Result<u32, HotkeyError>;
    
    /// Unregister a previously registered hotkey
    fn unregister_hotkey(&mut self, id: u32) -> Result<(), HotkeyError>;
    
    /// Unregister all hotkeys managed by this instance
    fn unregister_all(&mut self);
    
    /// Check if a hotkey combination is valid
    fn validate_hotkey(&self, keys: &str) -> Result<(), HotkeyError>;
    
    /// Parse hotkey string into key components
    fn parse_hotkey(&self, keys: &str) -> Result<HotkeyInfo, HotkeyError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub version: String,
    pub display: DisplayConfig,
    pub behavior: BehaviorConfig,
    pub hotkeys: HotkeyConfig,
    pub notifications: NotificationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub transparency: f32,
    pub hover_transparency: f32,
    pub position: (i32, i32),
    pub text_color: Option<Color>,
    pub show_controls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub always_on_top: bool,
    pub remember_position: bool,
    pub auto_detect_background: bool,
    pub minimize_to_tray: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub toggle_visibility: Option<String>,
    pub start_stop: Option<String>,
    pub reset: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub sound_enabled: bool,
    pub visual_flash: bool,
    pub system_notification: bool,
    pub sound_file: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone)]
pub struct HotkeyInfo {
    pub modifiers: Vec<String>, // e.g., ["Ctrl", "Alt"]
    pub key: String,            // e.g., "T"
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    FileNotFound,
    InvalidFormat(String),
    WriteError(String),
    ValidationFailed(Vec<ValidationError>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidTransparency(f32),
    InvalidPosition(i32, i32),
    InvalidHotkey(String),
    InvalidSoundFile(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum HotkeyError {
    InvalidFormat(String),
    AlreadyRegistered(String),
    RegistrationFailed(String),
    NotRegistered(u32),
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            display: DisplayConfig {
                transparency: 0.3,        // 70% transparent
                hover_transparency: 0.8,  // 20% transparent on hover
                position: (100, 100),
                text_color: None,         // Auto-detect
                show_controls: true,
            },
            behavior: BehaviorConfig {
                always_on_top: true,
                remember_position: true,
                auto_detect_background: true,
                minimize_to_tray: false,
            },
            hotkeys: HotkeyConfig {
                toggle_visibility: Some("Ctrl+Alt+T".to_string()),
                start_stop: Some("Ctrl+Alt+S".to_string()),
                reset: Some("Ctrl+Alt+R".to_string()),
            },
            notifications: NotificationConfig {
                sound_enabled: true,
                visual_flash: true,
                system_notification: true,
                sound_file: None,
            },
        }
    }
}

#[cfg(test)]
mod config_contract_tests {
    use super::*;
    
    // Contract test: Default configuration should be valid
    #[test]
    fn default_configuration_is_valid() {
        panic!("Config manager implementation not yet created");
    }
    
    // Contract test: Save and load should round-trip correctly
    #[test]
    fn save_load_roundtrip() {
        panic!("Config manager implementation not yet created");
    }
    
    // Contract test: Invalid values should fail validation
    #[test]
    fn invalid_values_fail_validation() {
        panic!("Config manager implementation not yet created");
    }
    
    // Contract test: Hotkey registration should succeed for valid keys
    #[test]
    fn valid_hotkey_registration() {
        panic!("Hotkey manager implementation not yet created");
    }
    
    // Contract test: Invalid hotkey should fail registration
    #[test]
    fn invalid_hotkey_fails_registration() {
        panic!("Hotkey manager implementation not yet created");
    }
}