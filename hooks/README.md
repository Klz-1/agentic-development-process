# Claude Code Hooks for Agentic Development

This directory contains Claude Code hooks that automate session management for both **Master Orchestrator** and **Subagent** roles.

## Role-Specific Hooks

| Hook | Role | Event | Purpose |
|------|------|-------|---------|
| `master-session-start.sh` | Master | SessionStart | Overview of all phases, blockers, completed work |
| `master-session-end.sh` | Master | Stop | Save orchestration state |
| `subagent-session-start.sh` | Subagent | SessionStart | Phase-specific context, Master notes, current task |
| `subagent-session-end.sh` | Subagent | Stop | Save phase session state |

## Installation

### For Master Orchestrator (Main Repository)

```bash
# From the main project repository
mkdir -p .claude/hooks

# Copy Master hooks
cp /path/to/agentic-development-process/hooks/master-session-start.sh .claude/hooks/
cp /path/to/agentic-development-process/hooks/master-session-end.sh .claude/hooks/
chmod +x .claude/hooks/*.sh

# Use Master settings template
cp /path/to/agentic-development-process/templates/claude-settings-master.json .claude/settings.json
```

### For Subagents (Phase Worktrees)

Each phase worktree needs its own hooks:

```bash
# From a phase worktree (e.g., .worktrees/phase-1)
mkdir -p .claude/hooks

# Copy Subagent hooks
cp /path/to/agentic-development-process/hooks/subagent-session-start.sh .claude/hooks/
cp /path/to/agentic-development-process/hooks/subagent-session-end.sh .claude/hooks/
chmod +x .claude/hooks/*.sh

# Use Subagent settings template
cp /path/to/agentic-development-process/templates/claude-settings-subagent.json .claude/settings.json
```

### Quick Setup Script

```bash
#!/bin/bash
# setup-hooks.sh - Run from main project directory

ADP_PATH="/path/to/agentic-development-process"

# Setup Master hooks in main repo
mkdir -p .claude/hooks
cp "$ADP_PATH/hooks/master-session-start.sh" .claude/hooks/
cp "$ADP_PATH/hooks/master-session-end.sh" .claude/hooks/
chmod +x .claude/hooks/*.sh
cp "$ADP_PATH/templates/claude-settings-master.json" .claude/settings.json
echo "✓ Master hooks installed"

# Setup Subagent hooks in each worktree
for wt in .worktrees/*/; do
    if [ -d "$wt" ]; then
        mkdir -p "$wt/.claude/hooks"
        cp "$ADP_PATH/hooks/subagent-session-start.sh" "$wt/.claude/hooks/"
        cp "$ADP_PATH/hooks/subagent-session-end.sh" "$wt/.claude/hooks/"
        chmod +x "$wt/.claude/hooks/"*.sh
        cp "$ADP_PATH/templates/claude-settings-subagent.json" "$wt/.claude/settings.json"
        echo "✓ Subagent hooks installed in $wt"
    fi
done
```

## What Each Hook Does

### Master Orchestrator Hooks

**master-session-start.sh** provides:
- Overview table of ALL phase statuses
- Summary stats (total, in progress, blocked, complete)
- Immediate attention items highlighted
- Blocked phase details with BLOCKERS.md content
- Completed phases ready for review
- Open PR status (if `gh` CLI available)
- Recent activity across all phases

**master-session-end.sh** saves:
- Phase status snapshot
- Recent activity summary
- Updates `.coordination/MASTER-SESSION-STATE.md`

### Subagent Hooks

**subagent-session-start.sh** provides:
- MASTER-NOTES.md content (guidance from Master)
- Previous session state
- Task list and current task
- Active blockers warning
- Git status and recent commits
- Subagent guidelines reminder

**subagent-session-end.sh** saves:
- Current task and status
- Uncommitted changes
- Recent commits
- Updates `.phase-status/SESSION-STATE.md`

## Settings Templates

| Template | Use For |
|----------|---------|
| `claude-settings-master.json` | Main repository (Master Orchestrator) |
| `claude-settings-subagent.json` | Phase worktrees (Subagents) |

## Example Output

### Master Session Start
```
## Master Orchestrator Session

**Role:** Master Orchestrator
**Project:** my-saas-app

### Phase Status Overview

| Phase | Branch | Status | Action Needed |
|-------|--------|--------|---------------|
| phase-1 | `feature/phase-1-foundation` | ✅ Complete | **Review & create PR** |
| phase-2 | `feature/phase-2-auth` | 🚨 BLOCKED | **Resolve blocker** |
| phase-3 | `feature/phase-3-dashboard` | In Progress | |

### ⚠️ Immediate Attention Required

- phase-1: Ready for review
- phase-2: BLOCKED - Check BLOCKERS.md
```

### Subagent Session Start
```
## Subagent Session - phase-2

**Role:** Subagent (Phase Worker)
**Phase:** phase-2
**Branch:** `feature/phase-2-auth`

### 📋 Master Orchestrator Notes

**IMPORTANT:** Review guidance from the Master Orchestrator:

[Content of MASTER-NOTES.md]

### Phase Tasks

**Progress:** 3/6 tasks complete

**Current Task:** Implement password reset flow
```

## Troubleshooting

### Hooks not running

1. Check permissions: `chmod +x .claude/hooks/*.sh`
2. Verify path in settings.json matches actual hook location
3. Check Claude Code logs for errors

### Wrong context shown

1. Verify you're using the correct hook for your role
2. Check that `.phase-status/` exists (for subagents)
3. Check that `.worktrees/` exists (for master)

### Settings not loading

1. Ensure `.claude/settings.json` exists in the right location
2. Verify JSON syntax is valid
3. Restart Claude Code session
