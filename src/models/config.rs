// Configuration data structures with serde serialization support
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub version: String,
    pub display: DisplayConfig,
    pub behavior: BehaviorConfig,
    pub hotkeys: HotkeyConfig,
    pub notifications: NotificationConfig,
}

/// Display-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub transparency: f32,           // 0.0 (transparent) to 1.0 (opaque)
    pub hover_transparency: f32,     // Transparency when hovered
    pub position: (i32, i32),        // Screen coordinates (logical pixels)
    pub text_color: Option<Color>,   // None = auto-detect, Some = manual
    pub show_controls: bool,         // Show start/pause buttons
}

/// Behavior-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub always_on_top: bool,
    pub remember_position: bool,
    pub auto_detect_background: bool,
    pub minimize_to_tray: bool,
}

/// Hotkey configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub toggle_visibility: Option<String>,  // e.g., "Ctrl+Alt+T"
    pub start_stop: Option<String>,         // e.g., "Ctrl+Alt+S"
    pub reset: Option<String>,              // e.g., "Ctrl+Alt+R"
}

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub sound_enabled: bool,
    pub visual_flash: bool,
    pub system_notification: bool,
    pub sound_file: Option<String>,
}

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    /// Calculate perceived luminance using standard formula
    pub fn luminance(&self) -> f32 {
        0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32
    }
    
    /// Predefined colors
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
}

/// Hotkey information structure
#[derive(Debug, Clone)]
pub struct HotkeyInfo {
    pub modifiers: Vec<String>, // e.g., ["Ctrl", "Alt"]
    pub key: String,            // e.g., "T"
}

/// Configuration-related errors
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    FileNotFound,
    InvalidFormat(String),
    WriteError(String),
    ValidationFailed(Vec<ValidationError>),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound => write!(f, "Configuration file not found"),
            ConfigError::InvalidFormat(msg) => write!(f, "Invalid configuration format: {}", msg),
            ConfigError::WriteError(msg) => write!(f, "Failed to write configuration: {}", msg),
            ConfigError::ValidationFailed(errors) => {
                write!(f, "Configuration validation failed: {} errors", errors.len())
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Validation errors for configuration values
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidTransparency(f32),
    InvalidPosition(i32, i32),
    InvalidHotkey(String),
    InvalidSoundFile(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidTransparency(value) => {
                write!(f, "Invalid transparency value: {} (must be 0.0-1.0)", value)
            }
            ValidationError::InvalidPosition(x, y) => {
                write!(f, "Invalid position: ({}, {}) (out of screen bounds)", x, y)
            }
            ValidationError::InvalidHotkey(keys) => {
                write!(f, "Invalid hotkey: '{}' (malformed key combination)", keys)
            }
            ValidationError::InvalidSoundFile(path) => {
                write!(f, "Invalid sound file: '{}' (file not found or unsupported format)", path)
            }
        }
    }
}

/// Hotkey-related errors
#[derive(Debug, Clone, PartialEq)]
pub enum HotkeyError {
    InvalidFormat(String),
    AlreadyRegistered(String),
    RegistrationFailed(String),
    NotRegistered(u32),
}

impl std::fmt::Display for HotkeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HotkeyError::InvalidFormat(msg) => write!(f, "Invalid hotkey format: {}", msg),
            HotkeyError::AlreadyRegistered(keys) => write!(f, "Hotkey already registered: {}", keys),
            HotkeyError::RegistrationFailed(msg) => write!(f, "Hotkey registration failed: {}", msg),
            HotkeyError::NotRegistered(id) => write!(f, "Hotkey not registered: {}", id),
        }
    }
}

impl std::error::Error for HotkeyError {}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            display: DisplayConfig::default(),
            behavior: BehaviorConfig::default(),
            hotkeys: HotkeyConfig::default(),
            notifications: NotificationConfig::default(),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            transparency: 0.3,        // 70% transparent
            hover_transparency: 0.8,  // 20% transparent on hover
            position: (100, 100),     // Default position with margin
            text_color: None,         // Auto-detect
            show_controls: true,      // Show controls by default
        }
    }
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            always_on_top: true,
            remember_position: true,
            auto_detect_background: true,
            minimize_to_tray: false,
        }
    }
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            toggle_visibility: Some("Ctrl+Alt+T".to_string()),
            start_stop: Some("Ctrl+Alt+S".to_string()),
            reset: Some("Ctrl+Alt+R".to_string()),
        }
    }
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            sound_enabled: true,
            visual_flash: true,
            system_notification: true,
            sound_file: None, // Use default system sound
        }
    }
}

/// Configuration validation methods
impl Configuration {
    /// Validate all configuration values
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        
        // Validate display configuration
        errors.extend(self.display.validate());
        
        // Validate hotkeys
        errors.extend(self.hotkeys.validate());
        
        // Validate notifications
        errors.extend(self.notifications.validate());
        
        errors
    }
    
    /// Check if configuration is valid
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }
}

impl DisplayConfig {
    /// Validate display configuration
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        
        // Validate transparency values
        if self.transparency < 0.0 || self.transparency > 1.0 {
            errors.push(ValidationError::InvalidTransparency(self.transparency));
        }
        
        if self.hover_transparency < 0.0 || self.hover_transparency > 1.0 {
            errors.push(ValidationError::InvalidTransparency(self.hover_transparency));
        }
        
        // Hover transparency should be >= base transparency for usability
        if self.hover_transparency < self.transparency {
            errors.push(ValidationError::InvalidTransparency(self.hover_transparency));
        }
        
        // Validate position (basic bounds check - detailed validation needs monitor info)
        let (x, y) = self.position;
        if x < -5000 || x > 10000 || y < -5000 || y > 10000 {
            errors.push(ValidationError::InvalidPosition(x, y));
        }
        
        errors
    }
}

impl HotkeyConfig {
    /// Validate hotkey configuration
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        
        // Validate each hotkey if present
        if let Some(ref keys) = self.toggle_visibility {
            if !Self::is_valid_hotkey(keys) {
                errors.push(ValidationError::InvalidHotkey(keys.clone()));
            }
        }
        
        if let Some(ref keys) = self.start_stop {
            if !Self::is_valid_hotkey(keys) {
                errors.push(ValidationError::InvalidHotkey(keys.clone()));
            }
        }
        
        if let Some(ref keys) = self.reset {
            if !Self::is_valid_hotkey(keys) {
                errors.push(ValidationError::InvalidHotkey(keys.clone()));
            }
        }
        
        errors
    }
    
    /// Basic hotkey validation (more detailed validation in hotkey manager)
    fn is_valid_hotkey(keys: &str) -> bool {
        !keys.is_empty() && 
        !keys.contains("++") && 
        !keys.starts_with('+') && 
        !keys.ends_with('+')
    }
}

impl NotificationConfig {
    /// Validate notification configuration
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        
        // Validate sound file if specified
        if let Some(ref sound_file) = self.sound_file {
            if !sound_file.is_empty() && !PathBuf::from(sound_file).exists() {
                errors.push(ValidationError::InvalidSoundFile(sound_file.clone()));
            }
        }
        
        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_configuration_is_valid() {
        let config = Configuration::default();
        assert!(config.is_valid());
        assert!(config.validate().is_empty());
    }
    
    #[test]
    fn test_color_luminance_calculation() {
        assert_eq!(Color::BLACK.luminance(), 0.0);
        assert_eq!(Color::WHITE.luminance(), 255.0);
        
        let red = Color::new(255, 0, 0, 255);
        let green = Color::new(0, 255, 0, 255);
        let blue = Color::new(0, 0, 255, 255);
        
        // Green should have highest luminance due to formula weighting
        assert!(green.luminance() > red.luminance());
        assert!(green.luminance() > blue.luminance());
    }
    
    #[test]
    fn test_invalid_transparency_validation() {
        let mut config = Configuration::default();
        config.display.transparency = -0.5;
        
        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| matches!(e, ValidationError::InvalidTransparency(_))));
    }
    
    #[test]
    fn test_invalid_position_validation() {
        let mut config = Configuration::default();
        config.display.position = (50000, 50000);
        
        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| matches!(e, ValidationError::InvalidPosition(_, _))));
    }
    
    #[test]
    fn test_invalid_hotkey_validation() {
        let mut config = Configuration::default();
        config.hotkeys.toggle_visibility = Some("++InvalidKey".to_string());
        
        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| matches!(e, ValidationError::InvalidHotkey(_))));
    }
    
    #[test]
    fn test_serde_serialization() {
        let config = Configuration::default();
        
        // Serialize to JSON
        let json = serde_json::to_string(&config).expect("Serialization should succeed");
        assert!(!json.is_empty());
        
        // Deserialize back
        let deserialized: Configuration = serde_json::from_str(&json)
            .expect("Deserialization should succeed");
        
        // Should be equal
        assert_eq!(config.version, deserialized.version);
        assert_eq!(config.display.transparency, deserialized.display.transparency);
        assert_eq!(config.behavior.always_on_top, deserialized.behavior.always_on_top);
    }
    
    #[test]
    fn test_hover_transparency_validation() {
        let mut config = Configuration::default();
        config.display.transparency = 0.8;
        config.display.hover_transparency = 0.5; // Lower than base transparency
        
        let errors = config.validate();
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| matches!(e, ValidationError::InvalidTransparency(_))));
    }
}