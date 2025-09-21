# GhostTimer - Development Context for Claude

**Project**: Windows Desktop Timer Widget  
**Language**: Rust  
**Updated**: 2025-09-21

## Project Overview

GhostTimer is a transparent, always-on-top Windows desktop timer widget designed to be non-intrusive during full-screen video watching. It features:

- Semi-transparent overlay that doesn't block content
- Always-on-top behavior for constant visibility
- Hover interaction for opacity changes
- Draggable positioning with persistence
- Background color detection for text contrast
- Global hotkey support
- Minimal resource usage

## Current Development Status

**Phase**: Planning Complete - Ready for Implementation  
**Branch**: `001-i-want-to`  
**Architecture**: Single Rust desktop application with modular design

## Tech Stack

```toml
[dependencies]
egui = "0.23"           # Immediate mode GUI
eframe = "0.23"         # Application framework
winit = "0.28"          # Window management
windows = "0.52"        # Windows API bindings
serde = "1.0"           # Serialization
serde_json = "1.0"      # JSON config files
global-hotkey = "0.4"   # System-wide hotkeys
dirs = "5.0"            # Standard directories
```

## Key Architecture Components

### Core Modules
- `models/timer.rs` - Timer state machine and logic
- `models/config.rs` - Configuration data structures
- `services/window_manager.rs` - Transparency and positioning
- `services/background_detector.rs` - Color detection
- `services/config_manager.rs` - Persistence
- `services/hotkey_manager.rs` - Global hotkey handling

### Interfaces (Contracts)
- `TimerControl` - Timer operations (start/pause/reset)
- `WindowManager` - Transparency, positioning, always-on-top
- `BackgroundDetector` - Color sampling and contrast calculation
- `ConfigManager` - Load/save/validate configuration
- `HotkeyManager` - Global hotkey registration

## Development Patterns

### TDD Approach
1. Write failing contract tests first
2. Implement minimal code to pass tests
3. Refactor while keeping tests green
4. Focus on interfaces before implementation details

### Windows API Integration
- Use `windows-rs` for safe Windows API access
- Key APIs: `SetLayeredWindowAttributes`, `WS_EX_LAYERED`, `WS_EX_TOPMOST`
- Handle DPI awareness through winit events
- Error handling for platform-specific operations

### Performance Requirements
- Memory: <20MB resident
- CPU: <1% idle, <3% during updates
- UI: 60 FPS capability, update only on state changes
- Startup: <500ms cold start

## Configuration

**Location**: `%APPDATA%/GhostTimer/config.json`

**Structure**:
```rust
struct Configuration {
    display: DisplayConfig,      // transparency, position, colors
    behavior: BehaviorConfig,    // always_on_top, persistence
    hotkeys: HotkeyConfig,       // key combinations
    notifications: NotificationConfig,  // sound, visual, system
}
```

## Current File Structure

```
src/
├── main.rs              # eframe application entry
├── models/              # Data structures and business logic
│   ├── timer.rs         # Timer state machine
│   └── config.rs        # Configuration types
├── services/            # Platform and infrastructure services
│   ├── window_manager.rs    # Window control
│   ├── background_detector.rs  # Color detection
│   ├── config_manager.rs     # Persistence
│   └── hotkey_manager.rs     # Global hotkeys
└── lib.rs              # Library exports

tests/
├── unit/               # Fast, isolated tests
├── integration/        # Realistic behavior tests
└── contract/          # Interface compliance tests

specs/001-i-want-to/
├── spec.md            # Feature requirements
├── plan.md            # Implementation plan
├── research.md        # Technical decisions
├── data-model.md      # Data structures
├── quickstart.md      # Development guide
└── contracts/         # Interface definitions
```

## Key Implementation Notes

### Timer Logic
- Use `std::time::Instant` for monotonic timing
- State machine: Stopped → Running → Paused/Finished
- Update frequency: 1 Hz for efficiency
- Handle system sleep/wake gracefully

### Window Management
- egui with custom eframe setup for transparency
- WS_EX_LAYERED for per-pixel alpha
- winit for DPI awareness and event handling
- Position stored in logical pixels, converted at runtime

### Background Detection
- Sample screen pixels in 3x3 grid around timer
- Calculate luminance: `0.299*R + 0.587*G + 0.114*B`
- Update only on position changes (performance)
- Fallback to white text on detection failure

## Common Commands

```bash
# Development
cargo run                    # Start application
cargo test                   # Run all tests
cargo test --lib            # Unit tests only
cargo check                  # Fast compilation check

# Debugging
RUST_LOG=debug cargo run     # Debug logging
cargo run -- --debug-ui     # Debug interface

# Release
cargo build --release       # Optimized build
cargo clippy                 # Linting
cargo fmt                    # Formatting
```

## Recent Changes

**2025-09-21**: Initial planning phase completed
- Feature specification defined
- Technical research completed
- Architecture and interfaces designed
- Contract tests defined (failing, ready for implementation)
- Development environment and quickstart guide created

## Next Steps

1. Execute `/tasks` command to generate detailed implementation tasks
2. Implement timer core logic following TDD approach
3. Add Windows-specific window management
4. Integrate egui UI with transparency support
5. Add configuration persistence and hotkey support

## Notes for Claude

- **Focus on TDD**: Always write tests before implementation
- **Windows-specific**: This is Windows-only, use platform APIs freely
- **Performance-critical**: Timer updates must be efficient
- **User experience**: Transparency and non-intrusiveness are key
- **Error handling**: Graceful degradation for platform failures

Current work is ready for implementation phase following the contracts and test-driven development approach.