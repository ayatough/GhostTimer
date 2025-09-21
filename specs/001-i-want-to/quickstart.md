# Quickstart Guide: Windows Desktop Timer Widget

**Date**: 2025-09-21  
**Feature**: Windows Desktop Timer Widget  
**Branch**: 001-i-want-to

## Development Environment Setup

### Prerequisites
- Windows 10/11 (development and target platform)
- Rust 1.75+ installed via [rustup](https://rustup.rs/)
- Git for version control
- VS Code or Rust IDE of choice

### Project Initialization

1. **Create new Rust project** (if not already exists):
   ```bash
   cargo new ghost_timer --bin
   cd ghost_timer
   ```

2. **Add dependencies to Cargo.toml**:
   ```toml
   [dependencies]
   egui = "0.23"
   eframe = "0.23"
   winit = "0.28"
   windows = "0.52"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   global-hotkey = "0.4"
   dirs = "5.0"
   
   [dev-dependencies]
   tokio-test = "0.4"
   ```

3. **Create project structure**:
   ```bash
   mkdir src/models src/services src/cli src/lib
   mkdir tests/contract tests/integration tests/unit
   ```

## Quick Start Implementation

### Step 1: Basic Timer Logic (TDD)

1. **Create failing test** (`tests/unit/timer_tests.rs`):
   ```rust
   use ghost_timer::models::Timer;
   use std::time::Duration;
   
   #[test]
   fn timer_starts_with_valid_duration() {
       let mut timer = Timer::new();
       let result = timer.start(Duration::from_secs(60));
       assert!(result.is_ok());
       assert!(timer.is_running());
   }
   ```

2. **Run test (should fail)**:
   ```bash
   cargo test timer_starts_with_valid_duration
   ```

3. **Implement minimal Timer** (`src/models/timer.rs`):
   ```rust
   use std::time::{Duration, Instant};
   
   pub struct Timer {
       // Implementation follows data-model.md
   }
   
   impl Timer {
       pub fn new() -> Self { /* ... */ }
       pub fn start(&mut self, duration: Duration) -> Result<(), TimerError> { /* ... */ }
       pub fn is_running(&self) -> bool { /* ... */ }
   }
   ```

### Step 2: Basic Window with Transparency

1. **Create failing window test** (`tests/integration/window_tests.rs`):
   ```rust
   #[test]
   fn window_shows_with_transparency() {
       // Test window creation with transparency
       panic!("Not implemented");
   }
   ```

2. **Implement basic eframe app** (`src/main.rs`):
   ```rust
   use eframe::egui;
   
   fn main() -> Result<(), eframe::Error> {
       let options = eframe::NativeOptions {
           transparent: true,
           always_on_top: true,
           decorated: false,
           initial_window_size: Some(egui::vec2(200.0, 100.0)),
           ..Default::default()
       };
       
       eframe::run_native(
           "GhostTimer",
           options,
           Box::new(|_cc| Box::new(TimerApp::default())),
       )
   }
   
   #[derive(Default)]
   struct TimerApp;
   
   impl eframe::App for TimerApp {
       fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
           egui::CentralPanel::default().show(ctx, |ui| {
               ui.label("05:00"); // Placeholder timer display
           });
       }
   }
   ```

### Step 3: Test-Driven Development Loop

**Red-Green-Refactor cycle**:

1. **Write a failing test** for next feature
2. **Run test** to confirm it fails: `cargo test`
3. **Implement minimal code** to make test pass
4. **Run test** to confirm it passes: `cargo test`
5. **Refactor** code while keeping tests green
6. **Repeat** for next feature

## User Story Validation

### Story 1: Timer during video playback
**Test**: Start video, activate timer, verify no video interference

**Manual test steps**:
1. Open YouTube video in full-screen
2. Run `cargo run` to start timer app
3. Set 2-minute timer
4. Verify timer is visible but transparent
5. Verify video continues playing normally
6. Wait for timer completion
7. Verify notification doesn't interrupt video

### Story 2: Hover interaction
**Test**: Hover over timer increases opacity

**Manual test steps**:
1. Start timer application
2. Position mouse away from timer - verify low opacity
3. Move mouse over timer area - verify increased opacity
4. Move mouse away - verify opacity returns to low level

### Story 3: Position persistence
**Test**: Timer remembers position between sessions

**Manual test steps**:
1. Start timer, drag to new position
2. Close application
3. Restart application
4. Verify timer appears in same position as before

## Development Commands

### Essential Commands
```bash
# Run application
cargo run

# Run all tests
cargo test

# Run specific test
cargo test timer_starts_with_valid_duration

# Run with debug output
RUST_LOG=debug cargo run

# Build release version
cargo build --release

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Testing Strategy
```bash
# Unit tests (fast, isolated)
cargo test --lib

# Integration tests (slower, realistic)
cargo test --test integration

# Contract tests (interface compliance)
cargo test --test contract
```

## Configuration

### Default Config Location
- `%APPDATA%/GhostTimer/config.json`

### Sample Configuration
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

## Troubleshooting

### Common Issues

**Timer window not appearing**:
- Check if always_on_top is working: `cargo run -- --debug`
- Verify transparency isn't set to 0.0
- Check monitor bounds in multi-monitor setup

**Hotkeys not working**:
- Run as administrator (Windows UAC may block global hotkeys)
- Check for conflicting hotkey registrations
- Verify hotkey format in config.json

**High CPU usage**:
- Check timer tick frequency (should be 1 Hz)
- Verify egui repaint requests are minimal
- Profile with `cargo flamegraph` if needed

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=ghost_timer=debug cargo run

# Show additional debug UI
cargo run -- --debug-ui
```

## Performance Targets

- **Startup time**: < 500ms cold start
- **Memory usage**: < 20MB resident
- **CPU usage**: < 1% idle, < 3% during timer updates
- **UI responsiveness**: < 16ms frame time (60 FPS)

## Next Steps

1. **Complete TDD implementation** following tasks.md
2. **Add Windows API integration** for transparency and always-on-top
3. **Implement configuration persistence** using serde_json
4. **Add hotkey support** with global-hotkey crate
5. **Optimize performance** to meet targets
6. **Add system notification support**
7. **Implement background color detection**

---

**Status**: ✅ Ready for implementation  
**Next Phase**: Task generation via `/tasks` command