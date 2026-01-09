# Task: Wire Up Real tmux Integration

## Context

- Project: /Users/klz/Desktop/Prototypes/agentic-development-process/hive
- Current state: UI works with mock data, need real tmux integration
- Files: src/tmux/\*.rs, src/app.rs

## Tasks

### 1. Implement TmuxClient::list_sessions()

In src/tmux/client.rs:

- Run `tmux list-sessions -F "#{session_name}:#{session_path}:#{session_activity}:#{session_id}"`
- Parse output into Vec<Session>
- Handle "no server running" error gracefully

### 2. Implement output capture

In src/tmux/output.rs:

- Run `tmux capture-pane -t {session} -p -S -100`
- Return Vec<String> of output lines
- Poll on tick events (every 500ms)

### 3. Implement resource monitoring

- Get pane PID: `tmux list-panes -t {session} -F "#{pane_pid}"`
- Use sysinfo crate to get CPU/memory for that PID

### 4. Implement session interaction

- send_keys(): `tmux send-keys -t {session} "{keys}"`
- send_interrupt(): `tmux send-keys -t {session} C-c`
- attach(): Restore terminal, exec `tmux attach -t {session}`

### 5. Wire to App state

- Add Vec<Session> to App struct
- Update on tick events
- Replace mock data in sessions panel

## Verification

```bash
cargo check
cargo clippy -- -D warnings
cargo build --release
./target/release/hive  # Should show real tmux sessions
```

## Completion

Output: TMUX_REAL_COMPLETE
