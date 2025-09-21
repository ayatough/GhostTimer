# Data Model: Windows Desktop Timer Widget

**Date**: 2025-09-21  
**Feature**: Windows Desktop Timer Widget  
**Branch**: 001-i-want-to

## Core Entities

### 1. Timer State
Represents the current state and behavior of the countdown timer.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TimerState {
    Stopped,
    Running { 
        started_at: Instant,
        remaining_duration: Duration,
    },
    Paused {
        remaining_duration: Duration,
    },
    Finished,
}

#[derive(Debug, Clone)]
pub struct Timer {
    pub state: TimerState,
    pub original_duration: Duration,
    pub completion_time: Option<Instant>,
}
```

**Validation Rules**:
- `original_duration` must be > 0 seconds and <= 24 hours
- `remaining_duration` must be >= 0 and <= `original_duration`
- State transitions must follow valid state machine rules
- `started_at` must be valid `Instant` when state is `Running`

**State Transitions**:
```
Stopped → Running: start(duration)
Running → Paused: pause()
Running → Finished: tick() when remaining = 0
Paused → Running: resume()
Paused → Stopped: reset()
Finished → Stopped: reset()
```

### 2. Configuration
Stores user preferences and application settings.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub display: DisplayConfig,
    pub behavior: BehaviorConfig,
    pub hotkeys: HotkeyConfig,
    pub notifications: NotificationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub transparency: f32,           // 0.0 (transparent) to 1.0 (opaque)
    pub hover_transparency: f32,     // Transparency when hovered
    pub position: (i32, i32),        // Screen coordinates (logical pixels)
    pub text_color: Option<Color>,   // None = auto-detect, Some = manual
    pub show_controls: bool,         // Show start/pause buttons
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
    pub toggle_visibility: Option<String>,  // e.g., "Ctrl+Alt+T"
    pub start_stop: Option<String>,         // e.g., "Ctrl+Alt+S"
    pub reset: Option<String>,              // e.g., "Ctrl+Alt+R"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub sound_enabled: bool,
    pub visual_flash: bool,
    pub system_notification: bool,
    pub sound_file: Option<String>,
}
```

**Validation Rules**:
- `transparency` and `hover_transparency` must be in range [0.0, 1.0]
- `hover_transparency` should be >= `transparency` for usability
- `position` coordinates must be within available screen bounds
- Hotkey strings must be valid key combinations
- `sound_file` path must exist if specified

**Default Values**:
- `transparency`: 0.3 (70% transparent)
- `hover_transparency`: 0.8 (20% transparent)
- `position`: (100, 100) - top-left corner with margin
- `always_on_top`: true
- `auto_detect_background`: true

### 3. Display Context
Runtime information about the display environment.

```rust
#[derive(Debug, Clone)]
pub struct DisplayContext {
    pub monitors: Vec<MonitorInfo>,
    pub current_monitor: usize,
    pub dpi_scale: f32,
    pub background_color: Option<Color>,
    pub last_background_sample: Option<Instant>,
}

#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub handle: String,              // Monitor identifier
    pub bounds: (i32, i32, i32, i32), // x, y, width, height
    pub dpi: u32,
    pub scale_factor: f32,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
```

**Validation Rules**:
- `current_monitor` must be valid index into `monitors` vec
- `dpi_scale` must be > 0.0
- Monitor bounds must not be negative width/height
- Background color sampling should not occur more than once per 5 seconds

### 4. Application State
Overall application runtime state and event handling.

```rust
#[derive(Debug)]
pub struct AppState {
    pub timer: Timer,
    pub config: Configuration,
    pub display_context: DisplayContext,
    pub ui_state: UiState,
    pub is_running: bool,
}

#[derive(Debug, Clone)]
pub struct UiState {
    pub is_visible: bool,
    pub is_hovered: bool,
    pub is_dragging: bool,
    pub drag_offset: Option<(f32, f32)>,
    pub context_menu_open: bool,
    pub settings_window_open: bool,
}
```

**Business Rules**:
- Timer can only be started when in `Stopped` or `Finished` state
- Configuration changes should be persisted immediately
- Background color detection should only occur when position changes
- UI hover state affects transparency immediately
- Window visibility can be toggled via hotkey even when timer is running

## Data Flow

### Timer Operations
```
User Input → Timer::start/pause/reset → State Change → UI Update → Config Persist
```

### Configuration Updates
```
Settings UI → Configuration::update → Validation → Persist to File → Apply to Runtime
```

### Background Detection
```
Position Change → Sample Screen Pixels → Calculate Luminance → Update Text Color → Repaint UI
```

### Hotkey Handling
```
Global Hotkey → Event → Action Dispatch → State Update → UI Refresh
```

## Persistence Strategy

### Configuration File Location
- Windows: `%APPDATA%/GhostTimer/config.json`
- File format: JSON with pretty printing for manual editing

### File Structure
```json
{
  "version": "1.0",
  "display": {
    "transparency": 0.3,
    "hover_transparency": 0.8,
    "position": [100, 100],
    "text_color": null,
    "show_controls": true
  },
  "behavior": {
    "always_on_top": true,
    "remember_position": true,
    "auto_detect_background": true,
    "minimize_to_tray": false
  },
  "hotkeys": {
    "toggle_visibility": "Ctrl+Alt+T",
    "start_stop": "Ctrl+Alt+S",
    "reset": "Ctrl+Alt+R"
  },
  "notifications": {
    "sound_enabled": true,
    "visual_flash": true,
    "system_notification": true,
    "sound_file": null
  }
}
```

### Migration Strategy
- Include version field for future migrations
- Graceful fallback to defaults for missing fields
- Preserve unknown fields for forward compatibility

## Error Handling

### Configuration Errors
- Invalid file format → Use defaults, backup corrupt file
- Invalid values → Use field defaults, log warnings
- File access errors → Continue with defaults, notify user

### Timer Errors
- Invalid duration input → Show validation message
- State machine violations → Log error, force valid state
- System time changes → Recalculate based on monotonic time

### Display Errors
- DPI changes → Recalculate positions and sizes
- Monitor disconnect → Move to primary monitor
- Background detection failure → Use default white text

---

**Status**: ✅ Complete  
**Next**: Contracts and API interfaces