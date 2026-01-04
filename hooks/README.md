# Claude Code Hooks for Agentic Development

This directory contains Claude Code hooks that automate session management with **automatic role detection**.

## Auto-Detecting Hooks (Recommended)

The main hooks automatically detect your role:

| Hook | Purpose |
|------|---------|
| `session-start.sh` | Detects role and shows appropriate context |
| `session-end.sh` | Detects role and saves appropriate state |

### How Role Detection Works

```
┌─────────────────────────────────────────────────────────────────┐
│                     ROLE AUTO-DETECTION                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   Has .worktrees/ directory?                                    │
│        │                                                        │
│        ├── YES → Master Orchestrator                            │
│        │         Shows: All phases, blockers, PRs               │
│        │                                                        │
│        └── NO → Check for .phase-status/                        │
│                      │                                          │
│                      ├── YES → Subagent                         │
│                      │         Shows: Master notes, tasks       │
│                      │                                          │
│                      └── NO → Check if inside worktree path     │
│                                    │                            │
│                                    ├── YES → Subagent           │
│                                    └── NO → Not agentic project │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Installation

### Simple Setup (One Command)

```bash
# From your project directory
mkdir -p .claude/hooks
cp /path/to/agentic-development-process/hooks/session-start.sh .claude/hooks/
cp /path/to/agentic-development-process/hooks/session-end.sh .claude/hooks/
chmod +x .claude/hooks/*.sh
cp /path/to/agentic-development-process/templates/claude-settings.json .claude/settings.json
```

That's it! The same hooks work whether you're in the main repo or a worktree.

### Setup Script

```bash
#!/bin/bash
# setup-hooks.sh - Run from main project directory
ADP_PATH="/path/to/agentic-development-process"

# Install hooks in main repo
mkdir -p .claude/hooks
cp "$ADP_PATH/hooks/session-start.sh" .claude/hooks/
cp "$ADP_PATH/hooks/session-end.sh" .claude/hooks/
chmod +x .claude/hooks/*.sh
cp "$ADP_PATH/templates/claude-settings.json" .claude/settings.json

echo "✓ Hooks installed in main repo"

# Install same hooks in each worktree
for wt in .worktrees/*/; do
    if [ -d "$wt" ]; then
        mkdir -p "$wt/.claude/hooks"
        cp "$ADP_PATH/hooks/session-start.sh" "$wt/.claude/hooks/"
        cp "$ADP_PATH/hooks/session-end.sh" "$wt/.claude/hooks/"
        chmod +x "$wt/.claude/hooks/"*.sh
        cp "$ADP_PATH/templates/claude-settings.json" "$wt/.claude/settings.json"
        echo "✓ Hooks installed in $wt"
    fi
done
```

## What Each Role Sees

### Master Orchestrator (detected in main repo)

```markdown
## Master Orchestrator Session

**Role:** Master Orchestrator
**Project:** my-saas-app

### Phase Status Overview

| Phase | Branch | Status | Action Needed |
|-------|--------|--------|---------------|
| phase-1 | `feature/phase-1` | ✅ Complete | **Review & create PR** |
| phase-2 | `feature/phase-2` | 🚨 BLOCKED | **Resolve blocker** |
| phase-3 | `feature/phase-3` | In Progress | |

### ⚠️ Immediate Attention Required

- phase-1: Ready for review
- phase-2: BLOCKED - needs guidance
```

### Subagent (detected in worktree)

```markdown
## Subagent Session - phase-2

**Role:** Subagent (Phase Worker)
**Phase:** phase-2
**Branch:** `feature/phase-2-auth`

### 📋 Master Orchestrator Notes

[Full content of MASTER-NOTES.md]

### Tasks

**Progress:** 3/6 complete
**Current:** Implement password reset flow

---

**Remember:** Test before commit • Update PROGRESS.md • Do NOT merge
```

## Role-Specific Hooks (Alternative)

If you prefer explicit role separation, use these instead:

| Hook | Role | Event |
|------|------|-------|
| `master-session-start.sh` | Master | SessionStart |
| `master-session-end.sh` | Master | Stop |
| `subagent-session-start.sh` | Subagent | SessionStart |
| `subagent-session-end.sh` | Subagent | Stop |

With corresponding settings templates:
- `templates/claude-settings-master.json`
- `templates/claude-settings-subagent.json`

## Settings File

The `templates/claude-settings.json` configures Claude Code:

```json
{
  "hooks": {
    "SessionStart": [{
      "matcher": "*",
      "hooks": [{
        "type": "command",
        "command": ".claude/hooks/session-start.sh",
        "timeout": 30
      }]
    }],
    "Stop": [{
      "matcher": "*",
      "hooks": [{
        "type": "command",
        "command": ".claude/hooks/session-end.sh",
        "timeout": 10
      }]
    }]
  }
}
```

## Troubleshooting

### Wrong role detected

Check your directory structure:
- Main repo should have `.worktrees/` directory
- Worktrees should have `.phase-status/` directory

### Hooks not running

1. Check permissions: `chmod +x .claude/hooks/*.sh`
2. Verify `.claude/settings.json` exists
3. Restart Claude Code session

### No output shown

The hooks exit silently if not in an agentic project (no `.worktrees/` or `.phase-status/`).
