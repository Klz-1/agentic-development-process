# Hive Implementation Plan

> Step-by-step guide to building the Agent Monitor TUI

**Project**: `hive`
**Target**: Production-grade Rust TUI for monitoring tmux agent sessions

---

## Phase Overview

| Phase | Focus                | Deliverable                                      | Status      |
| ----- | -------------------- | ------------------------------------------------ | ----------- |
| 0     | Project Setup        | Cargo project, CI, basic structure               | ✅ Complete |
| 1     | Core TUI Shell       | Three-panel layout, navigation, keyboard/mouse   | ✅ Complete |
| 2     | tmux Integration     | Session discovery, output streaming, interaction | ✅ Complete |
| 3     | File Explorer        | Tree view, git status, preview                   | ✅ Complete |
| 4     | Polish & Features    | Fuzzy finder, config, alerts, editor integration | ✅ Complete |
| 5     | Production Hardening | Testing, docs, release pipeline                  | ⏳ Pending  |

---

## Current Status (Updated 2026-01-10)

**Phase 4: Complete** ✅

| Feature | Status | Notes |
| ------- | ------ | ----- |
| 4.1 Fuzzy Finder | ✅ Done | Nucleo-powered fuzzy matching with highlighting |
| 4.2 Configuration | ✅ Done | TOML config loads from `~/.config/hive/config.toml` |
| 4.3 Help Overlay | ✅ Done | `?` shows shortcuts, Esc dismisses |
| 4.4 External Editor | ✅ Done | `e` opens file in $EDITOR/$VISUAL/fallback |
| 4.5 Visual Alerts | ✅ Done | Color indicators + terminal bell support |
| 4.6 Status Bar | ✅ Enhanced | Model info, contextual hints, build status |
| 4.7 Customizable Layout | ✅ Done | Panel widths + layout order configurable |

**Recent Implementations:**
- Tab bar component for workspace switching
- Hierarchical workspaces tree with project grouping
- Enhanced status bar with model info and key hints
- Progress/stats display in output panel (duration, data metrics)
- Nucleo fuzzy matching with match highlighting
- External editor integration ($EDITOR, $VISUAL, fallbacks)
- Terminal bell alerts for errors (configurable)
- Layout order configuration option

---

## Phase 0: Project Setup

### 0.1 Initialize Cargo Project

```bash
cargo new hive
cd hive
```

### 0.2 Set Up Dependencies

```toml
# Cargo.toml
[package]
name = "hive"
version = "0.1.0"
edition = "2021"
description = "Terminal UI for monitoring AI agent sessions"
license = "MIT"
repository = "https://github.com/youruser/hive"

[dependencies]
# TUI
ratatui = "0.29"
crossterm = "0.28"

# Async
tokio = { version = "1", features = ["full"] }

# tmux
tmux_interface = "0.3"

# Git
git2 = "0.19"

# File watching
notify = "7"

# System metrics
sysinfo = "0.32"

# Fuzzy matching
nucleo = "0.5"

# Config
toml = "0.8"
serde = { version = "1", features = ["derive"] }
directories = "5"

# Error handling
anyhow = "1"
thiserror = "2"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
pretty_assertions = "1"
tempfile = "3"
```

### 0.3 Project Structure

```
hive/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs              # Entry point
│   ├── app.rs               # App state & event loop
│   ├── config.rs            # TOML config loading
│   ├── event.rs             # Event handling (keyboard, mouse, tick)
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── layout.rs        # Panel arrangement
│   │   ├── sessions.rs      # Sessions panel
│   │   ├── output.rs        # Output panel
│   │   ├── files.rs         # File explorer panel
│   │   ├── fuzzy.rs         # Fuzzy finder overlay
│   │   ├── help.rs          # Help overlay
│   │   └── status_bar.rs    # Bottom status bar
│   ├── tmux/
│   │   ├── mod.rs
│   │   ├── session.rs       # Session data structures
│   │   ├── client.rs        # tmux IPC
│   │   └── output.rs        # Output capture & streaming
│   ├── files/
│   │   ├── mod.rs
│   │   ├── tree.rs          # File tree builder
│   │   ├── git.rs           # Git status integration
│   │   └── preview.rs       # File preview
│   └── utils/
│       ├── mod.rs
│       └── ring_buffer.rs   # Ring buffer for output
├── tests/
│   ├── tmux_tests.rs
│   └── config_tests.rs
└── .github/
    └── workflows/
        └── ci.yml
```

### 0.4 CI Setup

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo test
      - run: cargo build --release
```

### 0.5 Milestone Checkpoint

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes (empty tests)
- [ ] `cargo clippy` clean
- [ ] CI passing

---

## Phase 1: Core TUI Shell

### 1.1 Basic App Loop

**File: `src/main.rs`**

```rust
// Skeleton - actual implementation will be fuller
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize terminal
    // Run app loop
    // Restore terminal on exit
}
```

### 1.2 Event System

**File: `src/event.rs`**

Key events to handle:

- Keyboard input (arrows, Enter, Tab, letters)
- Mouse events (click, scroll, drag)
- Tick events (for refresh)
- Resize events

### 1.3 Three-Panel Layout

**File: `src/ui/layout.rs`**

Implement using Ratatui's `Layout::horizontal()` and `Layout::vertical()`:

- Sessions panel: 20% width (min 20 chars)
- Output panel: 40% width
- Files panel: 40% width
- Status bar: 1 row fixed at bottom

### 1.4 Panel Focus System

Track which panel has focus:

```rust
enum Panel {
    Sessions,
    Output,
    Files,
}
```

Tab cycles focus, visual indicator shows active panel.

### 1.5 Basic Navigation

- Arrow keys navigate within focused panel
- Tab switches panels
- Mouse click focuses panel and selects item

### 1.6 Milestone Checkpoint

- [ ] App starts and shows three empty panels
- [ ] Tab cycles focus (visual indicator changes)
- [ ] Arrow keys work in each panel
- [ ] Mouse click focuses panels
- [ ] `q` quits cleanly
- [ ] Terminal restored on exit/crash

---

## Phase 2: tmux Integration

### 2.1 Session Discovery

**File: `src/tmux/client.rs`**

```rust
pub async fn list_sessions() -> Result<Vec<TmuxSession>> {
    // Run: tmux list-sessions -F "#{session_name}:#{session_path}:#{session_activity}"
    // Parse output into Vec<TmuxSession>
}
```

### 2.2 Session Data Model

**File: `src/tmux/session.rs`**

```rust
pub struct TmuxSession {
    pub name: String,
    pub path: PathBuf,
    pub last_activity: DateTime<Utc>,
    pub pane_pid: Option<u32>,
}

pub struct SessionState {
    pub session: TmuxSession,
    pub status: SessionStatus,
    pub resource_usage: ResourceUsage,
    pub output_buffer: RingBuffer<String>,
}
```

### 2.3 Output Streaming

**File: `src/tmux/output.rs`**

Poll `tmux capture-pane` at configurable interval (default 500ms):

```rust
pub async fn capture_output(session: &str, lines: usize) -> Result<Vec<String>> {
    // Run: tmux capture-pane -t {session} -p -S -{lines}
}
```

### 2.4 Resource Monitoring

Use `sysinfo` crate to get CPU/memory for pane PID:

```rust
pub fn get_process_stats(pid: u32) -> Option<ResourceUsage> {
    // Query sysinfo for process CPU/memory
}
```

### 2.5 Session Interaction

**File: `src/tmux/client.rs`**

```rust
pub async fn send_keys(session: &str, keys: &str) -> Result<()> {
    // Run: tmux send-keys -t {session} "{keys}" Enter
}

pub async fn send_interrupt(session: &str) -> Result<()> {
    // Run: tmux send-keys -t {session} C-c
}

pub async fn attach(session: &str) -> Result<()> {
    // Restore terminal, exec: tmux attach -t {session}
}
```

### 2.6 Sessions Panel UI

**File: `src/ui/sessions.rs`**

Display for each session:

- Status icon (●/○/⚠/✓)
- Session name
- Resource usage (CPU%, MEM%)
- Progress bar (if available)
- Time since last activity (for idle)

### 2.7 Output Panel UI

**File: `src/ui/output.rs`**

- Display ring buffer contents
- Auto-scroll to bottom (toggleable)
- Timestamps on each line
- Command input line at bottom

### 2.8 Milestone Checkpoint

- [ ] Sessions panel shows real tmux sessions
- [ ] Selecting session updates output panel
- [ ] Output panel streams real-time output
- [ ] CPU/memory shown for each session
- [ ] Can send Ctrl+C to session
- [ ] Enter attaches to selected session
- [ ] Status icons reflect session state

---

## Phase 3: File Explorer

### 3.1 File Tree Builder

**File: `src/files/tree.rs`**

```rust
pub struct FileTree {
    root: PathBuf,
    entries: Vec<FileEntry>,
    expanded: HashSet<PathBuf>,
}

pub struct FileEntry {
    path: PathBuf,
    kind: FileKind,
    depth: usize,
}

pub enum FileKind {
    Directory,
    File,
}
```

Lazy loading: only expand directories when requested.

### 3.2 Git Status Integration

**File: `src/files/git.rs`**

```rust
pub fn get_git_status(repo_path: &Path) -> Result<HashMap<PathBuf, GitStatus>> {
    // Use git2 to get status of all files
    // Return map of path -> status
}
```

### 3.3 File Preview

**File: `src/files/preview.rs`**

- Read first N lines of file
- Basic syntax highlighting (or none for MVP)
- Handle binary files gracefully

### 3.4 Files Panel UI

**File: `src/ui/files.rs`**

- Tree view with indentation
- Git status indicators: [M] modified, [+] added, [?] untracked
- Expandable directories (Tab or Enter)
- File preview in bottom section of panel

### 3.5 Project Root Detection

When session is selected, detect project root:

1. Use session's working directory
2. Walk up to find `.git` directory
3. Fall back to configured project_root

### 3.6 Milestone Checkpoint

- [ ] Selecting session shows its project files
- [ ] Can navigate file tree with arrows
- [ ] Directories expand/collapse
- [ ] Git status shows modified/staged/untracked
- [ ] File preview shows in panel
- [ ] Hidden files toggleable

---

## Phase 4: Polish & Features

### 4.1 Fuzzy Finder

**File: `src/ui/fuzzy.rs`**

Overlay modal using `nucleo` for matching:

- `/` opens fuzzy finder
- Type to filter files/sessions/commands
- Enter selects, Esc closes

### 4.2 Configuration System

**File: `src/config.rs`**

```rust
#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub sessions: SessionsConfig,
    pub files: FilesConfig,
    pub keybindings: KeybindingsConfig,
    pub theme: ThemeConfig,
    pub alerts: AlertsConfig,
}

pub fn load_config() -> Result<Config> {
    // Check ~/.config/hive/config.toml
    // Fall back to defaults
}
```

### 4.3 Help Overlay

**File: `src/ui/help.rs`**

- `?` shows keyboard shortcuts
- Contextual (shows keys for current panel)
- Esc to dismiss

### 4.4 External Editor Integration

```rust
pub fn open_in_editor(path: &Path) -> Result<()> {
    // Suspend TUI
    // Run $EDITOR or fallback
    // Resume TUI after exit
}
```

### 4.5 Visual Alerts

- Status bar color changes on errors
- Session list highlights changed items
- Optional: terminal bell on error

### 4.6 Status Bar

**File: `src/ui/status_bar.rs`**

Show:

- Session counts (X active, Y idle, Z error)
- Current session name
- Current file path
- Key hints for current context

### 4.7 Customizable Layout

Allow users to configure panel arrangement:

```toml
[general]
layout = "sessions-output-files"  # or "output-sessions-files", etc.
```

### 4.8 Milestone Checkpoint

- [ ] Config file loads from `~/.config/hive/config.toml`
- [ ] Fuzzy finder works for files
- [ ] Help overlay shows shortcuts
- [ ] `e` opens file in external editor
- [ ] Status bar shows context info
- [ ] Visual alerts on session errors

---

## Phase 5: Production Hardening

### 5.1 Error Handling

- Graceful degradation when tmux unavailable
- User-friendly error messages
- Logging to file for debugging

### 5.2 Testing Strategy

**Unit Tests:**

- Config parsing
- Ring buffer behavior
- File tree building
- tmux output parsing

**Integration Tests:**

- Full app startup/shutdown
- tmux interaction (requires tmux in CI)

**Manual Test Matrix:**

- [ ] macOS + iTerm2
- [ ] macOS + Terminal.app
- [ ] Linux + Alacritty
- [ ] SSH session
- [ ] Small terminal (80x24)
- [ ] Large terminal (200x60)

### 5.3 Performance Optimization

- Profile with `cargo flamegraph`
- Ensure <16ms render time
- Optimize file tree for large repos
- Virtual scrolling for long output

### 5.4 Documentation

**README.md:**

- Installation instructions
- Quick start guide
- Configuration reference
- Screenshots/GIFs

**Man page:**

```bash
cargo install --path . && hive --generate-man > hive.1
```

### 5.5 Release Pipeline

```yaml
# .github/workflows/release.yml
# Triggered on version tags
# Builds for:
#   - x86_64-apple-darwin
#   - aarch64-apple-darwin
#   - x86_64-unknown-linux-gnu
# Creates GitHub release with binaries
```

### 5.6 Milestone Checkpoint

- [ ] All tests passing
- [ ] CI builds release binaries
- [ ] README complete with screenshots
- [ ] `cargo install hive` works
- [ ] Homebrew formula (optional)
- [ ] Performance targets met

---

## Implementation Order

For each phase, implement in this order:

1. **Data layer first** - Define structs, implement core logic
2. **Integration second** - Connect to external systems (tmux, git)
3. **UI last** - Build UI components that consume the data

This ensures you can test each layer independently.

---

## Quick Start Commands

```bash
# Create project
cargo new hive && cd hive

# Add dependencies (copy Cargo.toml above)

# Run in development
cargo run

# Run with logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Build release
cargo build --release

# Install locally
cargo install --path .
```

---

## Development Tips

1. **Use `ratatui`'s examples** - The repo has excellent examples for each widget type

2. **Test tmux commands manually first** - Run them in shell before implementing

3. **Start with hardcoded data** - Get UI working before connecting to tmux

4. **Implement quit early** - Make sure `q` always works to avoid terminal corruption

5. **Use `crossterm::execute!` for cleanup** - Ensure terminal is restored on panic

6. **Profile early** - Don't wait until the end to check performance

---

## Resources

- [Ratatui Book](https://ratatui.rs/introduction/)
- [Ratatui Examples](https://github.com/ratatui-org/ratatui/tree/main/examples)
- [tmux_interface docs](https://docs.rs/tmux_interface)
- [git2 docs](https://docs.rs/git2)
- [Crossterm docs](https://docs.rs/crossterm)

---

## Next Action

Start Phase 0: Run `cargo new hive` and set up the project structure.
