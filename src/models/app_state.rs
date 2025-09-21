// Application state and UI state models
use std::time::Duration;

use crate::models::{
    timer::{Timer, TimerControl, TimerState},
    config::Configuration,
    display::DisplayContext,
};

/// Overall application runtime state and event handling
#[derive(Debug)]
pub struct AppState {
    pub timer: Timer,
    pub config: Configuration,
    pub display_context: DisplayContext,
    pub ui_state: UiState,
    pub is_running: bool,
    pub notification_triggered: bool,
    pub config_dirty: bool, // Tracks if config needs saving
}

/// UI-specific state information
#[derive(Debug, Clone)]
pub struct UiState {
    pub is_visible: bool,
    pub is_hovered: bool,
    pub is_dragging: bool,
    pub drag_offset: Option<(f32, f32)>,
    pub context_menu_open: bool,
    pub settings_window_open: bool,
    pub controls_visible: bool,
    pub last_interaction: Option<std::time::Instant>,
}

impl AppState {
    /// Create a new application state with default values
    pub fn new() -> Self {
        Self {
            timer: Timer::new(),
            config: Configuration::default(),
            display_context: DisplayContext::new(),
            ui_state: UiState::new(),
            is_running: true,
            notification_triggered: false,
            config_dirty: false,
        }
    }
    
    /// Start a timer with the specified duration
    pub fn start_timer(&mut self, duration: Duration) -> Result<(), Box<dyn std::error::Error>> {
        let result = self.timer.start(duration)?;
        self.mark_interaction();
        Ok(result)
    }
    
    /// Pause the currently running timer
    pub fn pause_timer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let result = self.timer.pause()?;
        self.mark_interaction();
        Ok(result)
    }
    
    /// Resume a paused timer
    pub fn resume_timer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let result = self.timer.resume()?;
        self.mark_interaction();
        Ok(result)
    }
    
    /// Reset the timer to stopped state
    pub fn reset_timer(&mut self) {
        self.timer.reset();
        self.notification_triggered = false;
        self.mark_interaction();
    }
    
    /// Get current timer state
    pub fn timer_state(&self) -> &TimerState {
        self.timer.state()
    }
    
    /// Get remaining time if timer is running or paused
    pub fn remaining_time(&self) -> Option<Duration> {
        self.timer.remaining_time()
    }
    
    /// Check if timer has finished
    pub fn is_timer_finished(&self) -> bool {
        self.timer.is_finished()
    }
    
    /// Update timer state and return true if state changed
    pub fn tick_timer(&mut self) -> bool {
        let state_changed = self.timer.tick();
        
        // Check for timer completion
        if state_changed && self.timer.is_finished() {
            self.notification_triggered = true;
        }
        
        state_changed
    }
    
    /// Check if notification was triggered
    pub fn was_notification_triggered(&self) -> bool {
        self.notification_triggered
    }
    
    /// Get current window visibility
    pub fn is_window_visible(&self) -> bool {
        self.ui_state.is_visible
    }
    
    /// Set window visibility
    pub fn set_window_visible(&mut self, visible: bool) {
        self.ui_state.is_visible = visible;
        if visible {
            self.mark_interaction();
        }
    }
    
    /// Get current window transparency
    pub fn window_transparency(&self) -> f32 {
        if self.ui_state.is_hovered || self.ui_state.is_dragging {
            self.config.display.hover_transparency
        } else {
            self.config.display.transparency
        }
    }
    
    /// Set window transparency
    pub fn set_transparency(&mut self, alpha: f32) -> Result<(), Box<dyn std::error::Error>> {
        if alpha < 0.0 || alpha > 1.0 {
            return Err("Transparency must be between 0.0 and 1.0".into());
        }
        
        if self.ui_state.is_hovered {
            self.config.display.hover_transparency = alpha;
        } else {
            self.config.display.transparency = alpha;
        }
        
        self.mark_config_dirty();
        Ok(())
    }
    
    /// Check if window is always on top
    pub fn is_always_on_top(&self) -> bool {
        self.config.behavior.always_on_top
    }
    
    /// Get current window position
    pub fn window_position(&self) -> (i32, i32) {
        self.config.display.position
    }
    
    /// Set window position
    pub fn set_window_position(&mut self, x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.config.display.position = (x, y);
        
        // Update current monitor based on new position
        self.display_context.update_current_monitor(x, y);
        
        if self.config.behavior.remember_position {
            self.mark_config_dirty();
        }
        
        Ok(())
    }
    
    /// Drag window to a new position
    pub fn drag_window_to(&mut self, x: i32, y: i32) {
        // Constrain position to monitor bounds
        let constrained_pos = self.display_context.constrain_position(x, y, 200, 100);
        let _ = self.set_window_position(constrained_pos.0, constrained_pos.1);
    }
    
    /// Handle mouse enter event
    pub fn handle_mouse_enter(&mut self) {
        if !self.ui_state.is_hovered {
            self.ui_state.is_hovered = true;
            self.ui_state.controls_visible = true;
            self.mark_interaction();
        }
    }
    
    /// Handle mouse leave event
    pub fn handle_mouse_leave(&mut self) {
        if self.ui_state.is_hovered && !self.ui_state.is_dragging {
            self.ui_state.is_hovered = false;
            self.ui_state.controls_visible = false;
            self.mark_interaction();
        }
    }
    
    /// Handle drag start
    pub fn handle_drag_start(&mut self, x: i32, y: i32) {
        self.ui_state.is_dragging = true;
        let current_pos = self.window_position();
        self.ui_state.drag_offset = Some((
            (current_pos.0 - x) as f32,
            (current_pos.1 - y) as f32,
        ));
        self.mark_interaction();
    }
    
    /// Handle drag move
    pub fn handle_drag_move(&mut self, x: i32, y: i32) {
        if let Some((offset_x, offset_y)) = self.ui_state.drag_offset {
            let new_x = x + offset_x as i32;
            let new_y = y + offset_y as i32;
            self.drag_window_to(new_x, new_y);
        }
    }
    
    /// Handle drag end
    pub fn handle_drag_end(&mut self) {
        self.ui_state.is_dragging = false;
        self.ui_state.drag_offset = None;
        self.mark_interaction();
    }
    
    /// Handle DPI change
    pub fn handle_dpi_change(&mut self, new_scale: f32) {
        self.display_context.dpi_scale = new_scale;
        
        // Position in logical coordinates should remain the same
        // Physical position will be recalculated by window manager
    }
    
    /// Handle hotkey activation
    pub fn handle_hotkey(&mut self, keys: &str) {
        match keys {
            keys if Some(keys.to_string()) == self.config.hotkeys.toggle_visibility => {
                self.set_window_visible(!self.is_window_visible());
            }
            keys if Some(keys.to_string()) == self.config.hotkeys.start_stop => {
                match self.timer_state() {
                    TimerState::Stopped | TimerState::Finished => {
                        // Use last timer duration or default
                        let duration = if self.timer.original_duration.is_zero() {
                            Duration::from_secs(300) // 5 minutes default
                        } else {
                            self.timer.original_duration
                        };
                        let _ = self.start_timer(duration);
                    }
                    TimerState::Running { .. } => {
                        let _ = self.pause_timer();
                    }
                    TimerState::Paused { .. } => {
                        let _ = self.resume_timer();
                    }
                }
            }
            keys if Some(keys.to_string()) == self.config.hotkeys.reset => {
                self.reset_timer();
            }
            _ => {
                // Unknown hotkey
            }
        }
    }
    
    /// Check if controls are visible
    pub fn are_controls_visible(&self) -> bool {
        self.ui_state.controls_visible && self.config.display.show_controls
    }
    
    /// Get list of currently visible controls
    pub fn get_visible_controls(&self) -> Vec<String> {
        if !self.are_controls_visible() {
            return Vec::new();
        }
        
        let mut controls = Vec::new();
        
        match self.timer_state() {
            TimerState::Stopped | TimerState::Finished => {
                controls.push("start".to_string());
                controls.push("reset".to_string());
            }
            TimerState::Running { .. } => {
                controls.push("pause".to_string());
                controls.push("reset".to_string());
            }
            TimerState::Paused { .. } => {
                controls.push("resume".to_string());
                controls.push("reset".to_string());
            }
        }
        
        controls
    }
    
    /// Get current configuration
    pub fn get_configuration(&self) -> Configuration {
        self.config.clone()
    }
    
    /// Apply new configuration
    pub fn apply_configuration(&mut self, config: Configuration) {
        self.config = config;
        self.mark_config_dirty();
    }
    
    /// Check if configuration needs saving
    pub fn is_config_dirty(&self) -> bool {
        self.config_dirty
    }
    
    /// Mark configuration as saved
    pub fn mark_config_saved(&mut self) {
        self.config_dirty = false;
    }
    
    /// Simulate config being saved (for testing)
    pub fn is_config_saved(&self) -> bool {
        !self.config_dirty
    }
    
    /// Save configuration (placeholder for actual implementation)
    pub fn save_configuration(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // This will be implemented by the config manager service
        self.mark_config_saved();
        Ok(())
    }
    
    /// Get monitors information
    pub fn get_monitors(&self) -> Vec<crate::models::display::MonitorInfo> {
        self.display_context.monitors.clone()
    }
    
    /// Get current DPI scale
    pub fn get_dpi_scale(&self) -> f32 {
        self.display_context.dpi_scale
    }
    
    /// Get memory usage information (placeholder)
    pub fn get_memory_usage(&self) -> MemoryInfo {
        // This would be implemented with actual memory profiling
        MemoryInfo {
            resident_mb: 15, // Placeholder value
        }
    }
    
    /// Mark user interaction timestamp
    fn mark_interaction(&mut self) {
        self.ui_state.last_interaction = Some(std::time::Instant::now());
    }
    
    /// Mark configuration as needing save
    fn mark_config_dirty(&mut self) {
        self.config_dirty = true;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl UiState {
    /// Create new UI state with default values
    pub fn new() -> Self {
        Self {
            is_visible: true,
            is_hovered: false,
            is_dragging: false,
            drag_offset: None,
            context_menu_open: false,
            settings_window_open: false,
            controls_visible: false,
            last_interaction: None,
        }
    }
    
    /// Check if UI is in an interactive state
    pub fn is_interactive(&self) -> bool {
        self.is_hovered || self.is_dragging || self.context_menu_open || self.settings_window_open
    }
    
    /// Get time since last interaction
    pub fn time_since_interaction(&self) -> Option<Duration> {
        self.last_interaction.map(|instant| instant.elapsed())
    }
    
    /// Check if UI should auto-hide based on inactivity
    pub fn should_auto_hide(&self, timeout: Duration) -> bool {
        if self.is_interactive() {
            return false;
        }
        
        self.time_since_interaction()
            .map(|elapsed| elapsed > timeout)
            .unwrap_or(true)
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory usage information
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub resident_mb: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_state_creation() {
        let app = AppState::new();
        
        assert!(matches!(app.timer_state(), TimerState::Stopped));
        assert!(app.is_window_visible());
        assert!(!app.is_timer_finished());
        assert!(!app.was_notification_triggered());
        assert!(app.is_running);
    }
    
    #[test]
    fn test_timer_operations() {
        let mut app = AppState::new();
        
        // Start timer
        let result = app.start_timer(Duration::from_secs(60));
        assert!(result.is_ok());
        assert!(matches!(app.timer_state(), TimerState::Running { .. }));
        
        // Pause timer
        let result = app.pause_timer();
        assert!(result.is_ok());
        assert!(matches!(app.timer_state(), TimerState::Paused { .. }));
        
        // Resume timer
        let result = app.resume_timer();
        assert!(result.is_ok());
        assert!(matches!(app.timer_state(), TimerState::Running { .. }));
        
        // Reset timer
        app.reset_timer();
        assert!(matches!(app.timer_state(), TimerState::Stopped));
    }
    
    #[test]
    fn test_hover_behavior() {
        let mut app = AppState::new();
        
        let base_transparency = app.window_transparency();
        
        // Mouse enter should change transparency
        app.handle_mouse_enter();
        assert!(app.ui_state.is_hovered);
        assert!(app.are_controls_visible());
        
        let hover_transparency = app.window_transparency();
        assert!(hover_transparency > base_transparency);
        
        // Mouse leave should restore transparency
        app.handle_mouse_leave();
        assert!(!app.ui_state.is_hovered);
        assert!(!app.are_controls_visible());
        
        let final_transparency = app.window_transparency();
        assert_eq!(final_transparency, base_transparency);
    }
    
    #[test]
    fn test_drag_behavior() {
        let mut app = AppState::new();
        
        let initial_position = app.window_position();
        
        // Start drag
        app.handle_drag_start(initial_position.0, initial_position.1);
        assert!(app.ui_state.is_dragging);
        assert!(app.ui_state.drag_offset.is_some());
        
        // Move during drag
        app.handle_drag_move(initial_position.0 + 100, initial_position.1 + 50);
        
        let new_position = app.window_position();
        assert_ne!(new_position, initial_position);
        
        // End drag
        app.handle_drag_end();
        assert!(!app.ui_state.is_dragging);
        assert!(app.ui_state.drag_offset.is_none());
    }
    
    #[test]
    fn test_configuration_management() {
        let mut app = AppState::new();
        
        assert!(!app.is_config_dirty());
        
        // Modify configuration
        let mut config = app.get_configuration();
        config.display.transparency = 0.5;
        app.apply_configuration(config);
        
        assert!(app.is_config_dirty());
        
        // Save configuration
        let result = app.save_configuration();
        assert!(result.is_ok());
        assert!(!app.is_config_dirty());
    }
    
    #[test]
    fn test_hotkey_handling() {
        let mut app = AppState::new();
        
        // Test toggle visibility
        app.handle_hotkey("Ctrl+Alt+T");
        assert!(!app.is_window_visible());
        
        app.handle_hotkey("Ctrl+Alt+T");
        assert!(app.is_window_visible());
        
        // Test start/stop
        app.handle_hotkey("Ctrl+Alt+S"); // Should start with default duration
        assert!(matches!(app.timer_state(), TimerState::Running { .. }));
        
        app.handle_hotkey("Ctrl+Alt+S"); // Should pause
        assert!(matches!(app.timer_state(), TimerState::Paused { .. }));
        
        app.handle_hotkey("Ctrl+Alt+S"); // Should resume
        assert!(matches!(app.timer_state(), TimerState::Running { .. }));
        
        // Test reset
        app.handle_hotkey("Ctrl+Alt+R");
        assert!(matches!(app.timer_state(), TimerState::Stopped));
    }
    
    #[test]
    fn test_controls_visibility() {
        let mut app = AppState::new();
        
        // Controls should not be visible initially
        assert!(!app.are_controls_visible());
        
        // Hover should show controls
        app.handle_mouse_enter();
        assert!(app.are_controls_visible());
        
        let controls = app.get_visible_controls();
        assert!(controls.contains(&"start".to_string()));
        assert!(controls.contains(&"reset".to_string()));
        
        // Start timer and check controls change
        app.start_timer(Duration::from_secs(60)).unwrap();
        let running_controls = app.get_visible_controls();
        assert!(running_controls.contains(&"pause".to_string()));
        assert!(!running_controls.contains(&"start".to_string()));
    }
    
    #[test]
    fn test_ui_state_interaction_tracking() {
        let ui_state = UiState::new();
        
        // Initially no interaction
        assert!(ui_state.time_since_interaction().is_none());
        
        // Should auto-hide when no interaction
        assert!(ui_state.should_auto_hide(Duration::from_secs(5)));
        
        // Interactive state should prevent auto-hide
        let mut interactive_state = ui_state.clone();
        interactive_state.is_hovered = true;
        assert!(!interactive_state.should_auto_hide(Duration::from_secs(5)));
    }
}