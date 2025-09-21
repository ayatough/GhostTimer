// Display context and monitor information models
use std::time::Instant;
use serde::{Deserialize, Serialize};

pub use crate::models::config::Color;

/// Runtime information about the display environment
#[derive(Debug, Clone)]
pub struct DisplayContext {
    pub monitors: Vec<MonitorInfo>,
    pub current_monitor: usize,
    pub dpi_scale: f32,
    pub background_color: Option<Color>,
    pub last_background_sample: Option<Instant>,
}

/// Information about a monitor/display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorInfo {
    pub handle: String,              // Monitor identifier
    pub bounds: (i32, i32, i32, i32), // x, y, width, height
    pub dpi: u32,
    pub scale_factor: f32,
    pub is_primary: bool,
}

impl DisplayContext {
    /// Create a new display context with empty monitor list
    pub fn new() -> Self {
        Self {
            monitors: Vec::new(),
            current_monitor: 0,
            dpi_scale: 1.0,
            background_color: None,
            last_background_sample: None,
        }
    }
    
    /// Add a monitor to the context
    pub fn add_monitor(&mut self, monitor: MonitorInfo) {
        self.monitors.push(monitor);
    }
    
    /// Get the primary monitor
    pub fn primary_monitor(&self) -> Option<&MonitorInfo> {
        self.monitors.iter().find(|m| m.is_primary)
    }
    
    /// Get the current monitor
    pub fn current_monitor(&self) -> Option<&MonitorInfo> {
        self.monitors.get(self.current_monitor)
    }
    
    /// Find monitor containing the given point
    pub fn monitor_at_point(&self, x: i32, y: i32) -> Option<(usize, &MonitorInfo)> {
        for (index, monitor) in self.monitors.iter().enumerate() {
            let (mx, my, mw, mh) = monitor.bounds;
            if x >= mx && x < mx + mw && y >= my && y < my + mh {
                return Some((index, monitor));
            }
        }
        None
    }
    
    /// Set the current monitor based on position
    pub fn update_current_monitor(&mut self, x: i32, y: i32) {
        if let Some((index, _)) = self.monitor_at_point(x, y) {
            self.current_monitor = index;
            
            // Update DPI scale to match current monitor
            if let Some(monitor) = self.monitors.get(index) {
                self.dpi_scale = monitor.scale_factor;
            }
        }
    }
    
    /// Check if it's time to resample background color
    pub fn should_resample_background(&self) -> bool {
        match self.last_background_sample {
            None => true,
            Some(last_sample) => last_sample.elapsed().as_secs() >= 5, // Resample every 5 seconds max
        }
    }
    
    /// Update background color sample
    pub fn set_background_color(&mut self, color: Option<Color>) {
        self.background_color = color;
        self.last_background_sample = Some(Instant::now());
    }
    
    /// Convert logical coordinates to physical coordinates
    pub fn logical_to_physical(&self, logical_x: i32, logical_y: i32) -> (i32, i32) {
        let physical_x = (logical_x as f32 * self.dpi_scale) as i32;
        let physical_y = (logical_y as f32 * self.dpi_scale) as i32;
        (physical_x, physical_y)
    }
    
    /// Convert physical coordinates to logical coordinates
    pub fn physical_to_logical(&self, physical_x: i32, physical_y: i32) -> (i32, i32) {
        let logical_x = (physical_x as f32 / self.dpi_scale) as i32;
        let logical_y = (physical_y as f32 / self.dpi_scale) as i32;
        (logical_x, logical_y)
    }
    
    /// Check if a position is within any monitor bounds
    pub fn is_position_valid(&self, x: i32, y: i32) -> bool {
        self.monitor_at_point(x, y).is_some()
    }
    
    /// Constrain position to monitor bounds
    pub fn constrain_position(&self, x: i32, y: i32, window_width: i32, window_height: i32) -> (i32, i32) {
        // Try to find a suitable monitor
        let target_monitor = self.monitor_at_point(x, y)
            .map(|(_, monitor)| monitor)
            .or_else(|| self.current_monitor())
            .or_else(|| self.primary_monitor())
            .or_else(|| self.monitors.first());
        
        if let Some(monitor) = target_monitor {
            let (mx, my, mw, mh) = monitor.bounds;
            
            // Constrain to monitor bounds with some padding for window size
            let constrained_x = (x).max(mx).min(mx + mw - window_width.max(50));
            let constrained_y = (y).max(my).min(my + mh - window_height.max(50));
            
            (constrained_x, constrained_y)
        } else {
            // Fallback if no monitors available
            (x.max(0), y.max(0))
        }
    }
}

impl Default for DisplayContext {
    fn default() -> Self {
        Self::new()
    }
}

impl MonitorInfo {
    /// Create a new monitor info
    pub fn new(
        handle: String,
        bounds: (i32, i32, i32, i32),
        dpi: u32,
        scale_factor: f32,
        is_primary: bool,
    ) -> Self {
        Self {
            handle,
            bounds,
            dpi,
            scale_factor,
            is_primary,
        }
    }
    
    /// Get monitor width
    pub fn width(&self) -> i32 {
        self.bounds.2
    }
    
    /// Get monitor height
    pub fn height(&self) -> i32 {
        self.bounds.3
    }
    
    /// Get monitor center point
    pub fn center(&self) -> (i32, i32) {
        let (x, y, w, h) = self.bounds;
        (x + w / 2, y + h / 2)
    }
    
    /// Check if a point is within this monitor
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        let (mx, my, mw, mh) = self.bounds;
        x >= mx && x < mx + mw && y >= my && y < my + mh
    }
    
    /// Get DPI category for this monitor
    pub fn dpi_category(&self) -> DpiCategory {
        match self.dpi {
            0..=96 => DpiCategory::Standard,
            97..=144 => DpiCategory::High,
            145..=192 => DpiCategory::VeryHigh,
            _ => DpiCategory::Ultra,
        }
    }
}

/// DPI categories for monitor classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpiCategory {
    Standard,  // 96 DPI (100% scaling)
    High,      // 120 DPI (125% scaling)
    VeryHigh,  // 144-192 DPI (150-200% scaling)
    Ultra,     // >192 DPI (>200% scaling)
}

impl std::fmt::Display for DpiCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DpiCategory::Standard => write!(f, "Standard DPI"),
            DpiCategory::High => write!(f, "High DPI"),
            DpiCategory::VeryHigh => write!(f, "Very High DPI"),
            DpiCategory::Ultra => write!(f, "Ultra High DPI"),
        }
    }
}

/// Window positioning hints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionHint {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    Custom(i32, i32),
}

impl PositionHint {
    /// Calculate actual position based on monitor bounds and window size
    pub fn calculate_position(
        &self,
        monitor: &MonitorInfo,
        window_width: i32,
        window_height: i32,
    ) -> (i32, i32) {
        let (mx, my, mw, mh) = monitor.bounds;
        let margin = 50; // Margin from screen edges
        
        match self {
            PositionHint::TopLeft => (mx + margin, my + margin),
            PositionHint::TopRight => (mx + mw - window_width - margin, my + margin),
            PositionHint::BottomLeft => (mx + margin, my + mh - window_height - margin),
            PositionHint::BottomRight => (
                mx + mw - window_width - margin,
                my + mh - window_height - margin,
            ),
            PositionHint::Center => (
                mx + (mw - window_width) / 2,
                my + (mh - window_height) / 2,
            ),
            PositionHint::Custom(x, y) => (*x, *y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_monitor() -> MonitorInfo {
        MonitorInfo::new(
            "TEST_MONITOR".to_string(),
            (0, 0, 1920, 1080),
            96,
            1.0,
            true,
        )
    }
    
    fn create_test_context() -> DisplayContext {
        let mut context = DisplayContext::new();
        context.add_monitor(create_test_monitor());
        context
    }
    
    #[test]
    fn test_display_context_creation() {
        let context = DisplayContext::new();
        assert!(context.monitors.is_empty());
        assert_eq!(context.current_monitor, 0);
        assert_eq!(context.dpi_scale, 1.0);
        assert!(context.background_color.is_none());
    }
    
    #[test]
    fn test_monitor_info_creation() {
        let monitor = create_test_monitor();
        assert_eq!(monitor.handle, "TEST_MONITOR");
        assert_eq!(monitor.bounds, (0, 0, 1920, 1080));
        assert_eq!(monitor.width(), 1920);
        assert_eq!(monitor.height(), 1080);
        assert_eq!(monitor.center(), (960, 540));
        assert!(monitor.is_primary);
    }
    
    #[test]
    fn test_monitor_contains_point() {
        let monitor = create_test_monitor();
        
        assert!(monitor.contains_point(500, 500));
        assert!(monitor.contains_point(0, 0));
        assert!(monitor.contains_point(1919, 1079));
        assert!(!monitor.contains_point(-1, 0));
        assert!(!monitor.contains_point(1920, 1080));
        assert!(!monitor.contains_point(2000, 2000));
    }
    
    #[test]
    fn test_monitor_at_point() {
        let context = create_test_context();
        
        assert!(context.monitor_at_point(500, 500).is_some());
        assert!(context.monitor_at_point(-100, -100).is_none());
        assert!(context.monitor_at_point(2000, 2000).is_none());
    }
    
    #[test]
    fn test_coordinate_conversion() {
        let mut context = create_test_context();
        context.dpi_scale = 1.25;
        
        let (physical_x, physical_y) = context.logical_to_physical(100, 200);
        assert_eq!((physical_x, physical_y), (125, 250));
        
        let (logical_x, logical_y) = context.physical_to_logical(125, 250);
        assert_eq!((logical_x, logical_y), (100, 200));
    }
    
    #[test]
    fn test_position_constraining() {
        let context = create_test_context();
        
        // Position within bounds should remain unchanged
        let (x, y) = context.constrain_position(100, 100, 200, 150);
        assert_eq!((x, y), (100, 100));
        
        // Position outside bounds should be constrained
        let (x, y) = context.constrain_position(-100, -100, 200, 150);
        assert!(x >= 0 && y >= 0);
        
        let (x, y) = context.constrain_position(2000, 2000, 200, 150);
        assert!(x < 1920 && y < 1080);
    }
    
    #[test]
    fn test_dpi_categories() {
        let monitor_96 = MonitorInfo::new("Test".to_string(), (0, 0, 1920, 1080), 96, 1.0, true);
        assert_eq!(monitor_96.dpi_category(), DpiCategory::Standard);
        
        let monitor_144 = MonitorInfo::new("Test".to_string(), (0, 0, 1920, 1080), 144, 1.5, true);
        assert_eq!(monitor_144.dpi_category(), DpiCategory::High);
        
        let monitor_192 = MonitorInfo::new("Test".to_string(), (0, 0, 1920, 1080), 192, 2.0, true);
        assert_eq!(monitor_192.dpi_category(), DpiCategory::VeryHigh);
        
        let monitor_288 = MonitorInfo::new("Test".to_string(), (0, 0, 1920, 1080), 288, 3.0, true);
        assert_eq!(monitor_288.dpi_category(), DpiCategory::Ultra);
    }
    
    #[test]
    fn test_position_hints() {
        let monitor = create_test_monitor();
        let window_size = (200, 150);
        
        let top_left = PositionHint::TopLeft.calculate_position(&monitor, window_size.0, window_size.1);
        assert_eq!(top_left, (50, 50));
        
        let center = PositionHint::Center.calculate_position(&monitor, window_size.0, window_size.1);
        assert_eq!(center, (860, 465));
        
        let custom = PositionHint::Custom(300, 400).calculate_position(&monitor, window_size.0, window_size.1);
        assert_eq!(custom, (300, 400));
    }
    
    #[test]
    fn test_background_sampling_timing() {
        let mut context = create_test_context();
        
        // Should resample when no previous sample
        assert!(context.should_resample_background());
        
        // Set background color
        context.set_background_color(Some(Color::WHITE));
        
        // Should not resample immediately
        assert!(!context.should_resample_background());
        
        // Simulate time passing (we can't actually wait 5 seconds in unit test)
        // In real implementation, this would be tested with mock time
    }
    
    #[test]
    fn test_multi_monitor_setup() {
        let mut context = DisplayContext::new();
        
        // Add primary monitor
        let primary = MonitorInfo::new("PRIMARY".to_string(), (0, 0, 1920, 1080), 96, 1.0, true);
        context.add_monitor(primary);
        
        // Add secondary monitor
        let secondary = MonitorInfo::new("SECONDARY".to_string(), (1920, 0, 1920, 1080), 144, 1.5, false);
        context.add_monitor(secondary);
        
        assert_eq!(context.monitors.len(), 2);
        assert!(context.primary_monitor().is_some());
        assert_eq!(context.primary_monitor().unwrap().handle, "PRIMARY");
        
        // Test monitor detection
        assert_eq!(context.monitor_at_point(500, 500).unwrap().1.handle, "PRIMARY");
        assert_eq!(context.monitor_at_point(2500, 500).unwrap().1.handle, "SECONDARY");
    }
}