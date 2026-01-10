# Hive 🐝

> Terminal UI for monitoring AI agent sessions

A production-grade TUI for monitoring multiple tmux sessions running AI agents, with integrated file exploration and git awareness.

## Features

- **Multi-session Monitoring** - View all tmux sessions with status, CPU/memory usage, and progress indicators
- **Live Output Streaming** - Real-time streaming of session output with auto-scroll
- **File Explorer** - Browse project files with git status indicators
- **Fuzzy Finder** - Quick search for files, commands, and sessions with nucleo-powered matching
- **External Editor** - Open files in your preferred editor with `e`
- **Configurable Layout** - Customize panel arrangement and sizes
- **Visual Alerts** - Get notified when sessions have errors (with optional terminal bell)
- **Workspace Tree** - Hierarchical view of projects and workspaces

## Installation

### From source

```bash
git clone https://github.com/youruser/hive.git
cd hive
cargo install --path .
```

### Requirements

- Rust 1.70+ (for building)
- tmux (for session monitoring)
- A terminal with Unicode support

## Quick Start

1. Start some tmux sessions:

   ```bash
   tmux new-session -d -s project/feature-1
   tmux new-session -d -s project/feature-2
   ```

2. Run hive:

   ```bash
   hive
   ```

3. Navigate with:
   - `Tab` / `Shift+Tab` - Switch panels
   - `↑` / `↓` or `j` / `k` - Navigate within panel
   - `Enter` - Primary action
   - `?` - Show help

## Keyboard Shortcuts

| Key                    | Action                                  |
| ---------------------- | --------------------------------------- |
| `Tab` / `Shift+Tab`    | Switch between panels                   |
| `↑` / `↓` or `j` / `k` | Navigate up/down                        |
| `Enter`                | Primary action (attach, expand, select) |
| `?`                    | Show help overlay                       |
| `/`                    | Open fuzzy finder                       |
| `e`                    | Open file in editor (Files panel)       |
| `.`                    | Toggle hidden files (Files panel)       |
| `p`                    | Toggle auto-scroll (Output panel)       |
| `q`                    | Quit                                    |
| `Esc`                  | Cancel / close overlay / clear alert    |

See `?` in the app for full keybinding reference.

## Configuration

Create `~/.config/hive/config.toml`:

```toml
[general]
project_root = "~/projects"
refresh_rate_ms = 500
layout_order = "sessions-output-files"  # Panel arrangement

[sessions]
show_resource_usage = true
show_progress = true
output_buffer_lines = 1000

[files]
show_hidden = false
git_status = true
syntax_highlighting = true
preview_lines = 20

[alerts]
visual = true
sound = false  # Enable terminal bell on errors
```

### Layout Options

The `layout_order` setting controls panel arrangement:

- `sessions-output-files` (default): Sessions | Output | Files
- `output-sessions-files`: Output | Sessions | Files
- `files-output-sessions`: Files | Output | Sessions
- `sessions-files-output`: Sessions | Files | Output

## Command Line Options

```
USAGE:
    hive [OPTIONS]

OPTIONS:
    -d, --debug      Enable debug logging to ~/.local/share/hive/hive.log
    -V, --version    Print version information
    -h, --help       Print this help message
```

## Debug Mode

Run with debug logging to troubleshoot issues:

```bash
hive --debug
```

Logs are written to `~/.local/share/hive/hive.log`.

## Architecture

```
hive/
├── src/
│   ├── main.rs      # Entry point, CLI args, logging
│   ├── app.rs       # Application state, event handling
│   ├── config.rs    # Configuration loading
│   ├── event.rs     # Input event handling
│   ├── ui/          # UI components
│   │   ├── tab_bar.rs     # Workspace tabs
│   │   ├── sessions.rs    # Workspaces tree panel
│   │   ├── output.rs      # Session output panel
│   │   ├── files.rs       # File explorer panel
│   │   ├── status_bar.rs  # Status bar with hints
│   │   ├── help.rs        # Help overlay
│   │   └── fuzzy.rs       # Fuzzy finder
│   ├── files/       # File explorer
│   ├── tmux/        # tmux integration
│   └── utils/       # Utilities (ring buffer, etc.)
└── tests/           # Integration tests
```

## Development

```bash
# Run in development
cargo run

# Run with logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Run with verbose test output
cargo test -- --nocapture

# Build release
cargo build --release

# Check for issues
cargo clippy
```

## License

MIT

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request
