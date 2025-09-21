# Implementation Plan: Windows Desktop Timer Widget

**Branch**: `001-i-want-to` | **Date**: 2025-09-21 | **Spec**: [specs/001-i-want-to/spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-i-want-to/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   ✓ Loaded: Windows Desktop Timer Widget specification
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   ✓ Detected Project Type: single (Rust desktop application)
   ✓ Set Structure Decision: Option 1 (single project)
3. Fill the Constitution Check section based on the content of the constitution document.
   ✓ Applied default principles (constitution template found)
4. Evaluate Constitution Check section below
   ✓ No violations detected in approach
   ✓ Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   ✓ Research phase complete - resolving NEEDS CLARIFICATION items
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, CLAUDE.md
   ✓ Design phase complete with all artifacts
7. Re-evaluate Constitution Check section
   ✓ No new violations after design
   ✓ Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
   ✓ Task planning approach documented
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
Primary requirement: Create a transparent, always-on-top Windows desktop timer widget that doesn't interfere with full-screen video watching. Technical approach uses Rust with egui/eframe for the GUI, winit for window management, and Windows API for transparency and layered window controls. The application requires minimal resource usage and persistent configuration storage.

## Technical Context
**Language/Version**: Rust 1.75+  
**Primary Dependencies**: egui 0.23+, eframe 0.23+, winit 0.28+, windows-rs 0.52+  
**Storage**: Local config file (JSON/TOML) for user preferences  
**Testing**: cargo test with integration tests  
**Target Platform**: Windows 10/11 desktop  
**Project Type**: single - desktop application  
**Performance Goals**: <5% CPU usage, <50MB memory, 60fps UI refresh  
**Constraints**: Must maintain transparency, always-on-top behavior, minimal interference with other applications  
**Scale/Scope**: Single-user desktop application, <10k LOC, minimal UI surface

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Core Principles Applied**:
- Library-First: Timer logic, window management, and configuration will be separate modules
- CLI Interface: Basic CLI for configuration and testing (though primary interface is GUI)
- Test-First: TDD approach with timer logic tests before implementation
- Integration Testing: Window behavior, transparency, and persistence testing
- Simplicity: Minimal dependencies, clear separation of concerns

**Compliance Status**: ✅ PASS - Approach aligns with simplicity and modularity principles

## Project Structure

### Documentation (this feature)
```
specs/001-i-want-to/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 1: Single project (DEFAULT)
src/
├── models/              # Timer state, configuration structs
├── services/            # Timer logic, window management, config persistence
├── cli/                # Command-line interface for testing/config
└── lib/                # Core library modules

tests/
├── contract/           # API contract tests (minimal for desktop app)
├── integration/        # Window behavior, persistence tests
└── unit/              # Timer logic, configuration tests
```

**Structure Decision**: Option 1 (single project) - Desktop application with modular Rust structure

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - Windows API integration patterns for transparency and always-on-top behavior
   - egui/eframe best practices for minimal resource usage
   - Configuration persistence strategies for desktop applications
   - Cross-monitor positioning and DPI awareness

2. **Generate and dispatch research agents**:
   ```
   Task: "Research Windows layered window implementation in Rust"
   Task: "Find best practices for egui performance optimization"
   Task: "Research configuration persistence patterns for Rust desktop apps"
   Task: "Find patterns for cross-monitor DPI handling in Rust"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Timer State: current time, running status, original duration
   - Configuration: transparency, position, hotkeys, notification preferences
   - Display Context: monitor info, DPI settings, background detection data

2. **Generate API contracts** from functional requirements:
   - Internal module interfaces (no external API for desktop app)
   - Timer control interface: start/pause/reset/configure
   - Window management interface: position, transparency, hover behavior
   - Configuration interface: load/save/validate user preferences

3. **Generate contract tests** from contracts:
   - Timer state transition tests
   - Configuration validation tests
   - Window behavior tests
   - Tests must fail initially (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Timer countdown during video playback
   - Hover interaction for opacity change
   - Drag and position persistence
   - Right-click menu functionality

5. **Update agent file incrementally**:
   - Update CLAUDE.md with current Rust project context
   - Add egui/winit patterns and Windows API knowledge
   - Preserve existing manual additions
   - Keep under 150 lines for efficiency

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, CLAUDE.md

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each module interface → contract test task [P]
- Each entity → model creation task [P] 
- Each user interaction → integration test task
- Implementation tasks to make tests pass

**Ordering Strategy**:
- TDD order: Tests before implementation 
- Dependency order: Core timer logic → Window management → UI integration
- Mark [P] for parallel execution (independent modules)

**Estimated Output**: 20-25 numbered, ordered tasks in tasks.md

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*No constitution violations requiring justification*

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command)
- [x] Phase 1: Design complete (/plan command)
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented (none required)

---
*Based on Constitution v2.1.1 - See `/memory/constitution.md`*