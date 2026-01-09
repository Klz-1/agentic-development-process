# Task: Hive TUI - Phase 1: Core UI Shell

## Context

- Project: /Users/klz/Desktop/Prototypes/agentic-development-process/hive
- Stack: Rust + Ratatui + Crossterm + Tokio
- Primary files: src/ui/\*.rs, src/app.rs, src/event.rs

## Overview

Complete the TUI shell for Hive - a terminal UI for monitoring AI agent sessions. The UI skeleton exists but needs full implementation with proper interactivity.

## Tasks (Complete ALL)

### Task 1: Fix Module Compilation

- Ensure all modules compile without errors
- Fix any missing imports or type errors
- Run `cargo check` to verify

### Task 2: Complete Layout System

- Implement resizable panels in src/ui/layout.rs
- Add constraint calculations for minimum panel widths
- Support horizontal drag-to-resize between panels
- Store panel sizes in app state

### Task 3: Enhance Panel Rendering

- Sessions panel (src/ui/sessions.rs): Add scrolling for long lists, selection highlight
- Output panel (src/ui/output.rs): Add scroll position indicator, input line at bottom
- Files panel (src/ui/files.rs): Add tree indentation characters, expandable indicators

### Task 4: Mouse Support

- Enable mouse capture in terminal setup (src/app.rs)
- Handle mouse click to focus panels and select items
- Handle mouse scroll in each panel
- Handle mouse drag for panel resizing

### Task 5: Help Overlay

- Complete help.rs with full keyboard reference
- Add toggle state in App struct
- Render overlay when `?` pressed, dismiss on Esc

### Task 6: Fuzzy Finder Stub

- Create src/ui/fuzzy.rs with basic overlay structure
- Add state management for fuzzy finder open/closed
- Implement basic input handling (will be connected later)

## Technical Requirements

- Use Ratatui 0.29 patterns and widgets
- Follow existing code style in the project
- Keep UI responsive (no blocking operations in render)
- All state changes through App struct methods

## Testing Guidelines

1. Run `cargo check` after each file change
2. Run `cargo clippy` to catch issues
3. Run `cargo build` to ensure full compilation
4. Manual testing: Run the binary and verify:
   - Three panels render correctly
   - Tab cycles focus (cyan border indicates focus)
   - Arrow keys navigate within panels
   - `q` quits cleanly
   - `?` shows/hides help overlay

## Verification Commands

```bash
cd /Users/klz/Desktop/Prototypes/agentic-development-process/hive
cargo check
cargo clippy -- -D warnings
cargo build --release
```

## Permissions

### Allowed:

- Modify any file in src/ui/
- Modify src/app.rs and src/event.rs
- Add new UI-related modules
- Run cargo commands

### NOT Allowed:

- Modify tmux/ or files/ business logic
- Add new crate dependencies
- Delete existing code without replacement
- Git operations

## Completion Criteria

When ALL of the following are true:

- [ ] `cargo check` passes with no errors
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo build --release` succeeds
- [ ] All three panels render with proper borders
- [ ] Tab cycles focus between panels
- [ ] Mouse click focuses panels
- [ ] Help overlay toggles with `?`

Output: <promise>HIVE_UI_COMPLETE</promise>
