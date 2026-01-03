# Claude Code Hooks for Agentic Development

This directory contains Claude Code hooks that automate session management.

## Hooks Included

| Hook | Event | Purpose |
|------|-------|---------|
| `session-start.sh` | SessionStart | Automatically provides phase context at session start |
| `session-end.sh` | Stop | Automatically saves session state when Claude finishes |

## Installation

### Option 1: Copy to Your Project

Copy the hooks to your project's `.claude/hooks/` directory:

```bash
mkdir -p .claude/hooks
cp hooks/session-start.sh .claude/hooks/
cp hooks/session-end.sh .claude/hooks/
chmod +x .claude/hooks/*.sh
```

Then add to `.claude/settings.json`:

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/session-start.sh"
          }
        ]
      }
    ],
    "Stop": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/session-end.sh"
          }
        ]
      }
    ]
  }
}
```

### Option 2: Reference Directly

If you have the agentic-development-process repo available, reference the hooks directly:

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/agentic-development-process/hooks/session-start.sh"
          }
        ]
      }
    ],
    "Stop": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/agentic-development-process/hooks/session-end.sh"
          }
        ]
      }
    ]
  }
}
```

### Option 3: User-Wide Installation

Add to `~/.claude/settings.json` for all projects:

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "~/.claude/hooks/session-start.sh"
          }
        ]
      }
    ],
    "Stop": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "~/.claude/hooks/session-end.sh"
          }
        ]
      }
    ]
  }
}
```

## What the Hooks Do

### session-start.sh (SessionStart)

When a Claude Code session starts, this hook:

1. **Detects if you're in a phase worktree** - Provides phase-specific context
2. **Shows previous session state** - What was being worked on, current status
3. **Highlights MASTER-NOTES updates** - Alerts if new guidance since last session
4. **Shows blockers** - Warns if there are unresolved blockers
5. **Displays progress** - Current task and completion status
6. **Lists uncommitted changes** - So you know what's pending
7. **Shows recent commits** - Context on recent work

The output is fed directly to Claude as context, so Claude knows exactly where you left off.

### session-end.sh (Stop)

When Claude finishes responding, this hook:

1. **Auto-saves session state** - Creates/updates SESSION-STATE.md
2. **Infers context from git** - Captures what was worked on
3. **Records technical state** - Uncommitted files, recent commits
4. **Runs silently** - No output to avoid noise

This ensures session state is always preserved, even if you forget to save manually.

## Customization

### Timeout

Add a timeout to prevent hanging:

```json
{
  "type": "command",
  "command": ".claude/hooks/session-start.sh",
  "timeout": 30
}
```

### Conditional Execution

The hooks automatically detect if they're in a phase directory and skip silently if not. No configuration needed.

## Environment Variables

The hooks use these environment variables:

- `CLAUDE_PROJECT_DIR` - Project root (set by Claude Code)
- `PWD` - Current working directory (fallback)

## Troubleshooting

### Hook not running

1. Check the hook is executable: `chmod +x .claude/hooks/*.sh`
2. Verify the path in settings.json is correct
3. Check Claude Code logs for errors

### Wrong context shown

1. Make sure you're in the correct phase directory
2. Check that `.phase-status/` directory exists
3. Verify SESSION-STATE.md was created by previous session

### Session not saving

1. Check that `scripts/session-save.sh` exists and is executable
2. Verify the script path is correct for your directory structure
3. Run manually to check for errors: `./scripts/session-save.sh --auto`
