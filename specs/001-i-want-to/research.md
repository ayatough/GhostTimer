# Research: Windows Desktop Timer Widget

**Date**: 2025-09-21  
**Feature**: Windows Desktop Timer Widget  
**Branch**: 001-i-want-to

## Research Questions Resolved

### 1. Windows API Integration for Transparency and Always-On-Top

**Decision**: Use `windows-rs` crate with `WS_EX_LAYERED` and `WS_EX_TOPMOST` extended window styles
**Rationale**: 
- `windows-rs` provides safe Rust bindings to Windows API
- `WS_EX_LAYERED` enables per-window transparency via `SetLayeredWindowAttributes`
- `WS_EX_TOPMOST` ensures window stays above all other windows
- Well-documented pattern for overlay applications

**Alternatives considered**:
- Raw Windows API calls via `winapi` crate (deprecated, unsafe)
- Third-party overlay libraries (additional dependencies, less control)

**Implementation approach**:
```rust
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowLongPtrW, SetLayeredWindowAttributes,
    GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TOPMOST, LWA_ALPHA
};
```

### 2. egui/eframe Performance Optimization

**Decision**: Use `egui` 0.23+ with custom repaint scheduling and minimal UI updates
**Rationale**:
- egui provides immediate-mode GUI suitable for simple overlays
- Built-in support for transparency and custom rendering
- Excellent integration with winit for window management
- Low overhead when UI is static (timer display)

**Performance strategies**:
- Request repaint only when timer value changes (1-second intervals)
- Use `egui::Context::request_repaint_after()` for scheduled updates
- Minimize allocations in hot paths
- Cache rendered text when possible

**Alternatives considered**:
- Native Windows controls (limited transparency support)
- web-based overlay with webview (excessive overhead)
- ImGui-rs (more complex integration)

### 3. Configuration Persistence Strategy

**Decision**: JSON file in `%APPDATA%/GhostTimer/config.json` using `serde_json`
**Rationale**:
- JSON is human-readable and easily debuggable
- serde provides excellent serialization with minimal boilerplate
- Standard Windows application data location
- Simple file-based approach suitable for single-user desktop app

**Configuration structure**:
```rust
#[derive(Serialize, Deserialize)]
struct Config {
    transparency: f32,        // 0.0-1.0
    position: (i32, i32),     // Screen coordinates
    hotkeys: HotkeyConfig,
    notifications: NotificationConfig,
}
```

**Alternatives considered**:
- Windows Registry (complex API, harder to debug)
- TOML format (less familiar to users)
- INI files (limited data structure support)

### 4. Cross-Monitor DPI Handling

**Decision**: Use `winit`'s DPI-aware window management with scale factor monitoring
**Rationale**:
- winit handles DPI changes automatically through events
- egui integrates DPI scaling seamlessly
- Positions stored as logical pixels, converted to physical at runtime
- Handles monitor switching gracefully

**Implementation approach**:
```rust
// Monitor DPI changes
WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
    // Update stored positions and sizes
    config.apply_scale_factor(scale_factor);
}
```

**Alternatives considered**:
- Manual DPI detection via Windows API (complex, error-prone)
- Fixed pixel positioning (breaks on high-DPI monitors)
- Per-monitor DPI awareness without winit (reinventing the wheel)

### 5. Hotkey Implementation

**Decision**: Use `global-hotkey` crate for system-wide hotkey registration
**Rationale**:
- Cross-platform hotkey support (future Linux/macOS potential)
- Safe wrapper around Windows RegisterHotKey API
- Event-driven architecture integrates with egui event loop
- Handles hotkey conflicts gracefully

**Default hotkeys**:
- `Ctrl+Alt+T`: Toggle timer visibility
- `Ctrl+Alt+S`: Start/Stop timer
- `Ctrl+Alt+R`: Reset timer

**Alternatives considered**:
- Raw Windows RegisterHotKey API (platform-specific, unsafe)
- Keyboard hooks (global system interference, security concerns)
- Application-only hotkeys (limited when app not focused)

### 6. Background Color Detection

**Decision**: Sample screen pixels around timer position using Windows Desktop API
**Rationale**:
- GetPixel() API provides direct access to screen content
- Sample multiple points around timer for average color
- Calculate luminance to determine text contrast automatically
- Update only when timer position changes

**Algorithm**:
1. Sample 9 points in 3x3 grid around timer center
2. Calculate average RGB values
3. Convert to perceived luminance: `0.299*R + 0.587*G + 0.114*B`
4. Use white text if luminance < 128, black text otherwise

**Alternatives considered**:
- Desktop wallpaper analysis (doesn't account for running applications)
- Manual color selection (poor user experience)
- Fixed text color (readability issues)

### 7. Timer State Management

**Decision**: Use `std::time::Instant` with tick-based updates and state machine
**Rationale**:
- Instant provides monotonic, high-precision timing
- State machine prevents invalid transitions
- Tick-based updates enable pause/resume functionality
- No external dependencies required

**State machine**:
```
Stopped -> Running (start)
Running -> Paused (pause)
Running -> Finished (countdown reaches zero)
Paused -> Running (resume)
Paused -> Stopped (reset)
Finished -> Stopped (reset)
```

**Alternatives considered**:
- `std::time::SystemTime` (affected by system clock changes)
- External timer crates (unnecessary complexity)
- Thread-based timing (resource overhead)

## Dependencies Finalized

```toml
[dependencies]
egui = "0.23"
eframe = "0.23"
winit = "0.28"
windows = "0.52"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
global-hotkey = "0.4"
dirs = "5.0"  # For standard config directory
```

## Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Timer Core    │    │  Window Manager │    │  Configuration  │
│                 │    │                 │    │                 │
│ • State machine │    │ • Transparency  │    │ • Persistence   │
│ • Tick updates  │    │ • Always on top │    │ • Validation    │
│ • Notifications │    │ • DPI handling  │    │ • Defaults      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │   Application   │
                    │                 │
                    │ • Event loop    │
                    │ • UI rendering  │
                    │ • Hotkey handling│
                    └─────────────────┘
```

## Performance Considerations

- **Memory**: Estimated 10-20MB resident memory (egui + Windows API overhead)
- **CPU**: <1% during idle, <3% during active timer updates
- **Startup**: Target <500ms cold start time
- **Battery impact**: Minimal due to infrequent updates and GPU-accelerated rendering

## Security Considerations

- Configuration file permissions limited to current user
- Hotkey registration fails gracefully if keys already registered
- No network access or sensitive data handling required
- Standard Windows application security model

---

**Status**: ✅ Complete - All NEEDS CLARIFICATION items resolved  
**Next Phase**: Design & Contracts (data-model.md, contracts/, quickstart.md)