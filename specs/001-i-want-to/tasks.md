# Tasks: Windows Desktop Timer Widget

**Input**: Design documents from `/specs/001-i-want-to/`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/

## Execution Flow (main)
```
1. Load plan.md from feature directory
   ✓ Loaded: Rust desktop app with egui/eframe, winit, windows-rs
   ✓ Extract: tech stack, libraries, single project structure
2. Load optional design documents:
   ✓ data-model.md: Extract entities → Timer, Configuration, DisplayContext, AppState
   ✓ contracts/: 3 files → TimerControl, WindowManager, ConfigManager tests
   ✓ research.md: Extract decisions → Windows API, JSON config, hotkeys
3. Generate tasks by category:
   ✓ Setup: Rust project init, dependencies, Cargo.toml
   ✓ Tests: 3 contract tests, 3 integration scenarios from quickstart
   ✓ Core: 4 model structs, 4 service modules, main app
   ✓ Integration: Windows API, hotkeys, persistence
   ✓ Polish: unit tests, performance validation, cleanup
4. Apply task rules:
   ✓ Different files = mark [P] for parallel
   ✓ Same file = sequential (no [P])
   ✓ Tests before implementation (TDD)
5. Number tasks sequentially (T001-T025)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness:
   ✓ All contracts have tests? Yes (3/3)
   ✓ All entities have models? Yes (4/4)
   ✓ All Windows APIs covered? Yes
9. Return: SUCCESS (25 tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root
- Paths assume single Rust project structure per plan.md

## Phase 3.1: Setup
- [x] T001 Create Rust project structure (src/models/, src/services/, src/cli/, tests/unit/, tests/integration/, tests/contract/)
- [x] T002 Initialize Cargo.toml with dependencies: egui 0.23, eframe 0.23, winit 0.28, windows 0.52, serde 1.0, serde_json 1.0, global-hotkey 0.4, dirs 5.0
- [x] T003 [P] Configure cargo fmt and clippy in .cargo/config.toml

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**
- [x] T004 [P] Contract test TimerControl trait in tests/contract/test_timer_interface.rs
- [x] T005 [P] Contract test WindowManager trait in tests/contract/test_window_interface.rs  
- [x] T006 [P] Contract test ConfigManager trait in tests/contract/test_config_interface.rs
- [x] T007 [P] Integration test timer during video playbook scenario in tests/integration/test_timer_transparency.rs
- [x] T008 [P] Integration test hover interaction scenario in tests/integration/test_hover_behavior.rs
- [x] T009 [P] Integration test position persistence scenario in tests/integration/test_position_persistence.rs

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [x] T010 [P] Timer model and state machine in src/models/timer.rs
- [x] T011 [P] Configuration structs with serde in src/models/config.rs
- [x] T012 [P] DisplayContext and MonitorInfo in src/models/display.rs
- [x] T013 [P] AppState and UiState in src/models/app_state.rs
- [ ] T014 [P] TimerControl service implementation in src/services/timer_service.rs
- [ ] T015 [P] WindowManager service with Windows API in src/services/window_manager.rs
- [ ] T016 [P] ConfigManager service with JSON persistence in src/services/config_manager.rs
- [ ] T017 BackgroundDetector service with pixel sampling in src/services/background_detector.rs
- [ ] T018 HotkeyManager service with global-hotkey crate in src/services/hotkey_manager.rs
- [x] T019 Main eframe application in src/main.rs integrating all services
- [ ] T020 CLI interface for testing and configuration in src/cli/mod.rs

## Phase 3.4: Integration
- [ ] T021 Windows API transparency integration (SetLayeredWindowAttributes, WS_EX_LAYERED)
- [ ] T022 Always-on-top behavior (WS_EX_TOPMOST) and DPI awareness
- [ ] T023 Global hotkey registration and event handling
- [ ] T024 Configuration file persistence to %APPDATA%/GhostTimer/config.json

## Phase 3.5: Polish
- [ ] T025 [P] Unit tests for timer state transitions in tests/unit/test_timer_logic.rs
- [ ] T026 [P] Unit tests for configuration validation in tests/unit/test_config_validation.rs
- [ ] T027 Performance validation (<20MB memory, <1% CPU idle) via cargo bench
- [ ] T028 [P] Error handling and logging throughout application
- [ ] T029 Code cleanup, remove dead code, optimize imports
- [ ] T030 Manual testing following quickstart.md scenarios

## Dependencies
- Setup (T001-T003) before everything
- Tests (T004-T009) before implementation (T010-T024)
- Models (T010-T013) before services (T014-T018)
- Services before main app (T019-T020)
- Core implementation before integration (T021-T024)
- Everything before polish (T025-T030)

## Parallel Example
```
# Launch T004-T006 together (contract tests):
Task: "Contract test TimerControl trait in tests/contract/test_timer_interface.rs"
Task: "Contract test WindowManager trait in tests/contract/test_window_interface.rs"
Task: "Contract test ConfigManager trait in tests/contract/test_config_interface.rs"

# Launch T007-T009 together (integration tests):
Task: "Integration test timer transparency in tests/integration/test_timer_transparency.rs"
Task: "Integration test hover behavior in tests/integration/test_hover_behavior.rs"
Task: "Integration test position persistence in tests/integration/test_position_persistence.rs"

# Launch T010-T013 together (models):
Task: "Timer model and state machine in src/models/timer.rs"
Task: "Configuration structs with serde in src/models/config.rs"
Task: "DisplayContext and MonitorInfo in src/models/display.rs"
Task: "AppState and UiState in src/models/app_state.rs"

# Launch T014-T016 together (core services):
Task: "TimerControl service implementation in src/services/timer_service.rs"
Task: "WindowManager service with Windows API in src/services/window_manager.rs"
Task: "ConfigManager service with JSON persistence in src/services/config_manager.rs"
```

## Notes
- [P] tasks = different files, no dependencies
- Verify tests fail before implementing (TDD requirement)
- Use cargo test to run tests after each implementation task
- Focus on Windows API integration for transparency and positioning
- Test on real Windows system with video playback scenarios
- Commit after each completed task

## Task Generation Rules
*Applied during main() execution*

1. **From Contracts**:
   - timer_interface.rs → T004 contract test [P]
   - window_interface.rs → T005 contract test [P] 
   - config_interface.rs → T006 contract test [P]
   
2. **From Data Model**:
   - Timer entity → T010 model task [P]
   - Configuration entity → T011 model task [P]
   - DisplayContext entity → T012 model task [P]
   - AppState entity → T013 model task [P]
   
3. **From User Stories (quickstart.md)**:
   - Timer during video → T007 integration test [P]
   - Hover interaction → T008 integration test [P]
   - Position persistence → T009 integration test [P]

4. **Ordering**:
   - Setup → Tests → Models → Services → Main App → Integration → Polish
   - Windows API dependencies handled in integration phase

## Validation Checklist
*GATE: Checked by main() before returning*

- [x] All contracts have corresponding tests (T004-T006)
- [x] All entities have model tasks (T010-T013)
- [x] All tests come before implementation (T004-T009 before T010+)
- [x] Parallel tasks truly independent (different files, marked [P])
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task
- [x] Windows-specific requirements covered (transparency, always-on-top)
- [x] TDD approach enforced with failing tests first