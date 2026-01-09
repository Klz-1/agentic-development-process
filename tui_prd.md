# PRD: Agent Monitor TUI

> A production-grade terminal-based interface for monitoring and interacting with multiple tmux sessions running AI agents, with integrated file exploration and git awareness.

**Status**: Requirements Finalized
**Last Updated**: 2026-01-09

---

## 1. Problem Statement

**What pain are we solving?**

When running multiple AI agents (Claude Code, custom scripts, etc.) across different projects, there's no unified way to:

- Monitor their progress and status at scale (10+ concurrent sessions)
- Quickly inspect files they've created or modified
- Context-switch between agent workspaces without losing track
- Send commands or interact with agents without manual tmux attach/detach
- Get alerted when agents need attention or complete their work

**Current workaround:**
`tmux ls` + manual `tmux attach` + separate file browser/editor + constant context switching

**Target user:** Non-developer power user running parallel AI agents for productivity/automation tasks.

---

## 2. Requirements

### 2.1 Agent & Session Context

| Requirement          | Specification                                            |
| -------------------- | -------------------------------------------------------- |
| Agent types          | Claude Code (primary), extensible for future agent types |
| Session organization | One tmux session per agent instance                      |
| Concurrent sessions  | 10+ sessions (UI must scale gracefully)                  |
| Agent lifecycle      | Mixed: some quick tasks, some long-running operations    |
| Naming convention    | Configurable, default: `{project}-{agent-id}`            |

### 2.2 Monitoring Requirements

| Requirement        | Specification                                                                                                        |
| ------------------ | -------------------------------------------------------------------------------------------------------------------- |
| Status at a glance | Activity state (running/idle/error/completed), progress indicators, last output preview, resource usage (CPU/memory) |
| Live output        | Real-time tail of selected session's terminal output                                                                 |
| Interaction level  | Full interaction: send arbitrary commands, Ctrl+C, signals                                                           |
| Quick attach       | Press Enter to detach TUI and attach directly to tmux session                                                        |
| Alerts             | Visual (color/icon changes) + sound alerts for errors/completion                                                     |

### 2.3 File Explorer Requirements

| Requirement     | Specification                                               |
| --------------- | ----------------------------------------------------------- |
| File operations | Quick edits in-TUI + full editor on demand                  |
| Git integration | Essential: show modified/staged/untracked files, view diffs |
| Project roots   | Single configurable directory (e.g., `~/projects`)          |
| File types      | All files, with syntax highlighting for code                |
| Search          | Fuzzy file finder (fzf-style quick jump by name)            |
| External editor | Launch `$EDITOR` for full editing sessions                  |

### 2.4 Navigation & UX

| Requirement      | Specification                                                      |
| ---------------- | ------------------------------------------------------------------ |
| Keybinding style | Arrow keys + intuitive shortcuts (discoverable, beginner-friendly) |
| Layout           | Customizable/tiling panels (user-configurable arrangement)         |
| Mouse support    | Full: click to select, scroll, resize panels, hover hints          |
| Command palette  | Yes, fuzzy finder for actions                                      |
| Color scheme     | Respect terminal theme + configurable palette                      |
| Discoverability  | Hover tooltips, status bar hints, `?` for help overlay             |

### 2.5 Technical Specifications

| Requirement    | Specification                                                                            |
| -------------- | ---------------------------------------------------------------------------------------- |
| Language       | Rust + Ratatui                                                                           |
| Distribution   | Single binary (no runtime dependencies)                                                  |
| Config format  | TOML                                                                                     |
| Remote support | Pattern A (TUI over SSH) works automatically; Pattern B (local TUI, remote agents) as v2 |
| Platforms      | macOS (primary), Linux                                                                   |

### 2.6 Scope

#### MVP (v1.0)

1. **Session list panel** - All tmux sessions with status, activity, resource usage
2. **Live output panel** - Real-time tail of selected session
3. **File explorer panel** - Tree view with git status indicators
4. **Quick attach** - Enter to attach to tmux session
5. **Fuzzy file finder** - Quick jump to files
6. **Basic interaction** - Send commands to sessions
7. **Visual alerts** - Status changes reflected in UI
8. **TOML configuration** - Project roots, keybindings, theme

#### v2.0 (Extended)

- **Agent launcher wizard** - Step through: project → model → task → launch
- **Remote Pattern B** - Monitor agents on remote machines via SSH
- **Sound alerts** - Audio cues for attention-needed events
- **Content search** - Ripgrep across project files
- **Full git operations** - Commit, stage, diff, log from TUI

#### Anti-Goals (Explicitly NOT doing)

- **Not an IDE** - No LSP, autocomplete, intellisense, or code intelligence
- **Not a terminal emulator** - Use quick-attach for full terminal access
- **Not a process manager** - Won't restart crashed agents automatically (v1)

---

## 3. Architecture

### 3.1 UI Layout

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Agent Monitor TUI                                        [?] Help  [⚙] Config│
├───────────────────┬─────────────────────────┬───────────────────────────────┤
│ Sessions          │ Output                  │ Files                         │
│ ─────────────────│ ───────────────────────│ ─────────────────────────────│
│                   │                         │ ~/projects/webapp             │
│ ● webapp-agent    │ [15:42:03] Running      │ ├── src/                      │
│   CPU: 12% MEM: 2%│ tests...                │ │   ├── main.rs               │
│   ▓▓▓▓▓▓░░░░ 60%  │                         │ │   ├── lib.rs          [M]   │
│                   │ [15:42:05] ✓ 42/42      │ │   └── utils.rs        [M]   │
│ ○ api-refactor    │ tests passed            │ ├── tests/                    │
│   Idle 5m         │                         │ │   └── integration.rs  [+]   │
│                   │ [15:42:06] Building     │ ├── Cargo.toml               │
│ ● docs-update     │ release...              │ └── README.md                │
│   CPU: 8% MEM: 1% │                         │                               │
│                   │ [15:42:10] Completed    │ ─────────────────────────────│
│ ⚠ data-migration  │ successfully            │ Preview: src/lib.rs           │
│   Error - check   │                         │ ─────────────────────────────│
│                   │ > _                     │  1│ use std::collections::*;  │
│ ○ test-runner     │                         │  2│ use crate::utils;         │
│   Completed ✓     │                         │  3│                           │
│                   │                         │  4│ pub fn process() {        │
├───────────────────┴─────────────────────────┴───────────────────────────────┤
│ 3 active │ 1 error │ 1 idle │ webapp-agent │ src/lib.rs:42 │ ↑↓ Navigate    │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Panel Behaviors

| Panel        | Focus Behavior                         | Key Actions                                                   |
| ------------ | -------------------------------------- | ------------------------------------------------------------- |
| **Sessions** | ↑↓ select session, output/files update | Enter: attach, Space: expand details, d: send interrupt       |
| **Output**   | Auto-scrolls, can pause                | Enter: send command, Ctrl+C: interrupt, p: pause scroll       |
| **Files**    | ↑↓ navigate tree                       | Enter: preview/edit, e: external editor, Tab: expand/collapse |

### 3.3 Key Bindings (Discoverable Style)

| Key     | Action                  | Context                                           |
| ------- | ----------------------- | ------------------------------------------------- |
| `Tab`   | Cycle panel focus       | Global                                            |
| `↑/↓`   | Navigate list           | Sessions, Files                                   |
| `Enter` | Primary action          | Attach (sessions), Open (files), Send (output)    |
| `Space` | Secondary action        | Expand details (sessions), Toggle preview (files) |
| `e`     | Edit file               | Files panel                                       |
| `d`     | Send interrupt (Ctrl+C) | Sessions panel                                    |
| `p`     | Pause/resume scroll     | Output panel                                      |
| `/`     | Fuzzy finder            | Global                                            |
| `?`     | Help overlay            | Global                                            |
| `q`     | Quit                    | Global                                            |
| `Esc`   | Cancel/back             | Global                                            |

_Mouse: Click to select/focus, scroll wheel for navigation, drag to resize panels_

### 3.4 Component Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Agent Monitor TUI                        │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   Config    │  │    State    │  │   Events    │             │
│  │   Loader    │  │   Manager   │  │    Bus      │             │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘             │
│         │                │                │                     │
│  ┌──────▼────────────────▼────────────────▼──────┐             │
│  │                   App Core                     │             │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐         │             │
│  │  │ Session │ │ Output  │ │  File   │         │             │
│  │  │ Manager │ │ Stream  │ │ Explorer│         │             │
│  │  └────┬────┘ └────┬────┘ └────┬────┘         │             │
│  └───────┼───────────┼───────────┼───────────────┘             │
│          │           │           │                              │
│  ┌───────▼───────────▼───────────▼───────────────┐             │
│  │                  UI Layer                      │             │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐         │             │
│  │  │Sessions │ │ Output  │ │  Files  │         │             │
│  │  │  Panel  │ │  Panel  │ │  Panel  │         │             │
│  │  └─────────┘ └─────────┘ └─────────┘         │             │
│  └───────────────────────────────────────────────┘             │
│                           │                                     │
│  ┌────────────────────────▼─────────────────────┐              │
│  │              External Integrations            │              │
│  │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────────┐ │              │
│  │  │ tmux │  │ git  │  │ $EDITOR│ │ System  │ │              │
│  │  │ IPC  │  │ CLI  │  │       │ │ Metrics │ │              │
│  │  └──────┘  └──────┘  └──────┘  └──────────┘ │              │
│  └───────────────────────────────────────────────┘             │
└─────────────────────────────────────────────────────────────────┘
```

### 3.5 Data Models

```rust
// Core session representation
struct Session {
    id: String,
    name: String,
    project_root: PathBuf,
    status: SessionStatus,
    resource_usage: ResourceUsage,
    last_activity: DateTime<Utc>,
    output_buffer: RingBuffer<String>,
}

enum SessionStatus {
    Running { progress: Option<Progress> },
    Idle { since: DateTime<Utc> },
    Error { message: String },
    Completed { at: DateTime<Utc> },
}

struct Progress {
    current: u32,
    total: u32,
    label: Option<String>,
}

struct ResourceUsage {
    cpu_percent: f32,
    memory_mb: u32,
}

// File explorer state
struct FileTree {
    root: PathBuf,
    entries: Vec<FileEntry>,
    git_status: HashMap<PathBuf, GitStatus>,
}

struct FileEntry {
    path: PathBuf,
    kind: FileKind,
    git_status: Option<GitStatus>,
}

enum GitStatus {
    Modified,
    Staged,
    Untracked,
    Conflicted,
}
```

---

## 4. Technical Implementation

### 4.1 Dependencies

| Crate            | Purpose              |
| ---------------- | -------------------- |
| `ratatui`        | TUI framework        |
| `crossterm`      | Terminal backend     |
| `tokio`          | Async runtime        |
| `tmux_interface` | tmux IPC             |
| `git2`           | Git operations       |
| `notify`         | File system watching |
| `sysinfo`        | CPU/memory metrics   |
| `nucleo`         | Fuzzy matching       |
| `toml`           | Config parsing       |
| `directories`    | XDG paths            |
| `rodio`          | Sound alerts (v2)    |

### 4.2 tmux Integration

```bash
# List sessions with format
tmux list-sessions -F "#{session_name}:#{session_path}:#{session_activity}"

# Capture pane output (last N lines)
tmux capture-pane -t {session} -p -S -{lines}

# Send keys to session
tmux send-keys -t {session} "{command}" Enter

# Get pane PID for resource monitoring
tmux list-panes -t {session} -F "#{pane_pid}"
```

### 4.3 Configuration

```toml
# ~/.config/agent-monitor/config.toml

[general]
project_root = "~/projects"
refresh_rate_ms = 500
default_layout = "three-panel"  # or "two-panel", "custom"

[sessions]
show_resource_usage = true
show_progress = true
output_buffer_lines = 1000

[files]
show_hidden = false
git_status = true
syntax_highlighting = true
preview_lines = 20

[keybindings]
quit = "q"
help = "?"
fuzzy_find = "/"
# ... customizable

[theme]
# Uses terminal colors by default
# accent = "#7C3AED"
# error = "#EF4444"
# success = "#22C55E"

[alerts]
visual = true
sound = false  # v2
```

---

## 5. Name

**`hive`** - Evokes monitoring a colony of workers, fits the multi-agent paradigm.

Alternatives considered: `agentctl`, `swarm-tui`, `overseer`, `mission-control`

---

## 6. Success Metrics

| Metric                   | Target                           |
| ------------------------ | -------------------------------- |
| Session list render      | < 16ms (60fps) with 20+ sessions |
| Output streaming latency | < 100ms from tmux to display     |
| File tree load           | < 200ms for 1000-file project    |
| Memory usage             | < 50MB baseline                  |
| Binary size              | < 10MB                           |
| Startup time             | < 500ms                          |

---

## 7. Risks & Mitigations

| Risk                       | Mitigation                                               |
| -------------------------- | -------------------------------------------------------- |
| tmux API changes           | Pin tmux version requirements, test on multiple versions |
| High CPU from polling      | Use efficient polling intervals, pause when not focused  |
| Large output buffers       | Ring buffer with configurable size, virtual scrolling    |
| Git operations blocking UI | Run git commands in background tokio tasks               |

---

## 8. Open Items

- [ ] Decide on exact progress indicator parsing (Claude Code specific vs generic)
- [ ] Design agent launcher wizard UX (v2)
- [ ] SSH integration architecture for Pattern B (v2)
- [ ] Sound alert selection/customization (v2)
