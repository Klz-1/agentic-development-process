# Hive

> Terminal UI for monitoring AI agent sessions

A production-grade TUI for monitoring multiple tmux sessions running AI agents, with integrated file exploration and git awareness.

## Features

- **Session Monitoring** - View all tmux sessions with status, CPU/memory usage, and progress indicators
- **Live Output** - Real-time streaming of session output
- **File Explorer** - Browse project files with git status indicators
- **Quick Attach** - Jump directly into any session
- **Full Interaction** - Send commands and signals to sessions

## Installation

### From source

```bash
git clone https://github.com/youruser/hive.git
cd hive
cargo install --path .
```

### Homebrew (coming soon)

```bash
brew install hive
```

## Usage

```bash
# Start hive
hive

# With custom config
hive --config ~/.config/hive/config.toml
```

## Keyboard Shortcuts

| Key     | Action            |
| ------- | ----------------- |
| `Tab`   | Cycle panel focus |
| `↑/↓`   | Navigate          |
| `Enter` | Primary action    |
| `/`     | Fuzzy finder      |
| `?`     | Help              |
| `q`     | Quit              |

See `?` in the app for full keybinding reference.

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

## Requirements

- tmux 3.0+
- Rust 1.70+ (for building)

## License

MIT
