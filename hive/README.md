# Hive

> Terminal UI for monitoring AI agent sessions

A production-grade TUI for monitoring multiple tmux sessions running AI agents, with integrated file exploration and git awareness.

![Hive Screenshot](docs/screenshot.png)

## Features

- **Multi-session Monitoring** - View all tmux sessions with status indicators
- **Live Output Streaming** - Real-time session output with auto-scroll
- **File Explorer** - Browse project files with git status indicators
- **Dark Theme** - Clean, modern dark theme (requires true color terminal)
- **Mouse Support** - Click to select, scroll, and resize panels
- **Fuzzy Finder** - Quick search with nucleo-powered matching
- **External Editor** - Open files in your preferred editor
- **Configurable** - Customize layout, colors, and behavior

## Requirements

- **Rust 1.70+** (for building)
- **tmux** (for session monitoring)
- **True color terminal** - Ghostty, iTerm2, Alacritty, Kitty, or WezTerm
  - Note: macOS Terminal.app has limited color support

## Installation

```bash
git clone https://github.com/youruser/hive.git
cd hive
cargo install --path .
```

## Quick Start

1. Start some tmux sessions:

   ```bash
   tmux new-session -d -s myproject/feature-1
   tmux new-session -d -s myproject/feature-2
   ```

2. Run hive:
   ```bash
   hive
   ```

## Keyboard Shortcuts

| Key                 | Action                      |
| ------------------- | --------------------------- |
| `Tab` / `Shift+Tab` | Switch panels               |
| `↑/↓` or `j/k`      | Navigate                    |
| `Enter`             | Select / attach / expand    |
| `?`                 | Help overlay                |
| `/`                 | Fuzzy finder                |
| `e`                 | Open in editor (Files)      |
| `.`                 | Toggle hidden files         |
| `p`                 | Toggle auto-scroll (Output) |
| `q` / `Esc`         | Quit                        |

## Mouse Support

- **Click** - Focus panel and select item
- **Scroll** - Scroll within panel
- **Drag borders** - Resize panels

## Configuration

Create `~/.config/hive/config.toml`:

```toml
[general]
project_root = "~/projects"
refresh_rate_ms = 500

[sessions]
show_resource_usage = true
output_buffer_lines = 1000

[files]
show_hidden = false
git_status = true

[alerts]
visual = true
sound = false
```

## Terminal Compatibility

For the best experience, use a terminal with **true color (24-bit)** support:

| Terminal     | True Color | Recommended |
| ------------ | ---------- | ----------- |
| Ghostty      | ✅         | ✅          |
| iTerm2       | ✅         | ✅          |
| Alacritty    | ✅         | ✅          |
| Kitty        | ✅         | ✅          |
| WezTerm      | ✅         | ✅          |
| Terminal.app | ❌         | ❌          |

If colors look wrong, check:

```bash
echo $COLORTERM  # Should be "truecolor"
```

## Command Line

```
hive [OPTIONS]

OPTIONS:
    -d, --debug      Enable debug logging
    -V, --version    Print version
    -h, --help       Print help
```

Debug logs: `~/.local/share/hive/hive.log`

## Architecture

```
src/
├── main.rs          # Entry point
├── app.rs           # Application state
├── config.rs        # Configuration
├── theme.rs         # Color palette
├── event.rs         # Input handling
├── ui/
│   ├── sessions.rs  # Workspaces panel
│   ├── output.rs    # Output panel
│   ├── files.rs     # File explorer
│   ├── tab_bar.rs   # Tab bar
│   ├── status_bar.rs
│   ├── help.rs
│   └── fuzzy.rs
├── tmux/            # tmux integration
├── files/           # File tree & git
└── utils/           # Ring buffer, etc.
```

## Development

```bash
cargo run              # Run
cargo run -- --debug   # Run with logging
cargo test             # Test
cargo clippy           # Lint
cargo build --release  # Release build
```

## License

MIT
