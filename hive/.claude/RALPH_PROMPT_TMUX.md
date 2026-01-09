# Task: Hive TUI - Phase 2: tmux Integration

## Context

- Project: /Users/klz/Desktop/Prototypes/agentic-development-process/hive
- Stack: Rust + tmux CLI + sysinfo crate + Tokio
- Primary files: src/tmux/\*.rs

## Overview

Complete the tmux integration layer for Hive. The skeleton exists but needs real implementation for session discovery, output streaming, and interaction.

## Tasks (Complete ALL)

### Task 1: Fix Module Compilation

- Ensure src/tmux/mod.rs and all submodules compile
- Fix any type errors or missing imports
- Run `cargo check` to verify

### Task 2: Session Discovery

- Complete TmuxClient::list_sessions() in src/tmux/client.rs
- Parse tmux list-sessions output correctly
- Handle case when tmux server isn't running
- Extract session name, path, and activity timestamp

### Task 3: Output Streaming

- Implement efficient output capture in src/tmux/output.rs
- Use ring buffer to store last N lines
- Implement diff detection to only capture new lines
- Handle session disconnection gracefully

### Task 4: Resource Monitoring

- Add sysinfo integration in src/tmux/client.rs or new file
- Get CPU% and memory for pane PID
- Cache process info to avoid excessive polling
- Handle case when process no longer exists

### Task 5: Session Status Detection

- Implement logic to detect session status:
  - Running: recent activity, process active
  - Idle: no recent activity (configurable threshold)
  - Error: detect error patterns in output (optional)
  - Completed: detect completion patterns (optional)
- Update SessionStatus enum usage

### Task 6: Session Interaction

- Complete send_keys() and send_interrupt()
- Test that commands are properly escaped
- Implement attach() that properly suspends TUI

## Technical Requirements

- Use async/await with tokio for all I/O
- No blocking calls in async functions
- Proper error handling with anyhow::Result
- Efficient polling (don't hammer tmux)

## Testing Guidelines

1. Run `cargo check` after each change
2. Run `cargo clippy` for linting
3. Manual testing with real tmux sessions:
   ```bash
   # Create test sessions
   tmux new-session -d -s test1 "htop"
   tmux new-session -d -s test2 "watch date"
   ```
4. Verify sessions are discovered
5. Verify output is captured
6. Test send_keys works

## Verification Commands

```bash
cd /Users/klz/Desktop/Prototypes/agentic-development-process/hive
cargo check
cargo clippy -- -D warnings
cargo test --lib

# Test tmux commands manually
tmux list-sessions -F "#{session_name}:#{session_path}:#{session_activity}"
```

## Permissions

### Allowed:

- Modify any file in src/tmux/
- Add helper modules in src/tmux/
- Run cargo and tmux commands
- Create test tmux sessions

### NOT Allowed:

- Modify UI code (src/ui/)
- Add new crate dependencies without asking
- Modify files outside src/tmux/
- Git operations
- Kill user's tmux sessions

## Completion Criteria

When ALL of the following are true:

- [ ] `cargo check` passes with no errors
- [ ] `cargo clippy -- -D warnings` passes
- [ ] TmuxClient::list_sessions() returns real sessions
- [ ] Output capture works for active sessions
- [ ] Resource monitoring returns CPU/memory data
- [ ] send_keys() successfully sends commands to sessions

Output: <promise>HIVE_TMUX_COMPLETE</promise>
