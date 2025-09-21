// GhostTimer Library
// Windows Desktop Timer Widget

// Module declarations - these will fail until modules are implemented
pub mod models {
    pub mod timer;
    pub mod config;
    pub mod display;
    pub mod app_state;
}

pub mod services {
    pub mod timer_service;
    pub mod window_manager;
    pub mod config_manager;
    pub mod background_detector;
    pub mod hotkey_manager;
}

pub mod cli;

// Re-export commonly used types
pub use models::{
    timer::{Timer, TimerControl, TimerState, TimerError},
    config::Configuration,
    app_state::AppState,
};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");