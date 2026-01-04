#!/bin/bash
#
# subagent-session-start.sh - Claude Code SessionStart Hook for Subagents
#
# This hook runs when a Subagent starts a Claude Code session in a phase worktree.
# It provides the specific phase context and guidance from the Master.
#
# Output is fed directly to Claude as context.
#

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"

# Check if we're in a phase directory
if [ ! -d "$PROJECT_DIR/.phase-status" ] && [ ! -d ".phase-status" ]; then
    echo "## Subagent Session - Not in Phase Directory"
    echo ""
    echo "**Warning:** Not in a phase worktree directory."
    echo ""
    echo "Navigate to a phase worktree (e.g., \`.worktrees/phase-1\`) to begin work."
    exit 0
fi

# Determine phase directory
if [ -d ".phase-status" ]; then
    PHASE_DIR="$(pwd)"
else
    PHASE_DIR="$PROJECT_DIR"
fi

PHASE_STATUS_DIR="$PHASE_DIR/.phase-status"
PHASE_NAME=$(basename "$PHASE_DIR")
BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")

echo "## Subagent Session - $PHASE_NAME"
echo ""
echo "**Role:** Subagent (Phase Worker)"
echo "**Phase:** $PHASE_NAME"
echo "**Branch:** \`$BRANCH\`"
echo "**Time:** $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# CRITICAL: Check for MASTER-NOTES first
if [ -f "$PHASE_STATUS_DIR/MASTER-NOTES.md" ]; then
    echo "### 📋 Master Orchestrator Notes"
    echo ""
    echo "**IMPORTANT:** Review guidance from the Master Orchestrator:"
    echo ""
    echo '```markdown'
    cat "$PHASE_STATUS_DIR/MASTER-NOTES.md"
    echo '```'
    echo ""

    # Check if updated since last session
    if [ -f "$PHASE_STATUS_DIR/SESSION-STATE.md" ]; then
        notes_time=$(stat -c %Y "$PHASE_STATUS_DIR/MASTER-NOTES.md" 2>/dev/null || stat -f %m "$PHASE_STATUS_DIR/MASTER-NOTES.md" 2>/dev/null)
        state_time=$(stat -c %Y "$PHASE_STATUS_DIR/SESSION-STATE.md" 2>/dev/null || stat -f %m "$PHASE_STATUS_DIR/SESSION-STATE.md" 2>/dev/null)

        if [ "$notes_time" -gt "$state_time" ]; then
            echo "⚠️ **NEW UPDATES since your last session - read carefully!**"
            echo ""
        fi
    fi
fi

# Previous session state
if [ -f "$PHASE_STATUS_DIR/SESSION-STATE.md" ]; then
    echo "### Previous Session State"
    echo ""

    working_on=$(grep "^\*\*Working On:\*\*" "$PHASE_STATUS_DIR/SESSION-STATE.md" | sed 's/\*\*Working On:\*\* //')
    status=$(grep "^\*\*Status:\*\*" "$PHASE_STATUS_DIR/SESSION-STATE.md" | sed 's/\*\*Status:\*\* //')
    current_task=$(grep "^\*\*Current Task:\*\*" "$PHASE_STATUS_DIR/SESSION-STATE.md" | sed 's/\*\*Current Task:\*\* //')

    echo "- **Last working on:** ${working_on:-Not specified}"
    echo "- **Status:** ${status:-Not specified}"
    echo "- **Current task:** ${current_task:-Not specified}"
    echo ""

    # Priority actions
    if grep -q "### Priority Actions" "$PHASE_STATUS_DIR/SESSION-STATE.md"; then
        echo "**Priority for this session:**"
        sed -n '/### Priority Actions/,/###/p' "$PHASE_STATUS_DIR/SESSION-STATE.md" | grep -E "^[0-9]|^-|^Continue" | head -5
        echo ""
    fi
fi

# Task progress
if [ -f "$PHASE_STATUS_DIR/PROGRESS.md" ]; then
    echo "### Phase Tasks"
    echo ""

    completed=$(grep -c '\[x\]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null || echo 0)
    total=$(grep -c '\[.\]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null || echo 0)

    echo "**Progress:** $completed/$total tasks complete"
    echo ""

    # Show task list
    echo "**Task List:**"
    grep '^\- \[' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null | head -15
    echo ""

    # Highlight current task
    current=$(grep '^\- \[ \]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null | head -1 | sed 's/^- \[ \] //')
    if [ -n "$current" ]; then
        echo "**Current Task:** $current"
        echo ""
    fi
fi

# Check for existing blockers
if [ -f "$PHASE_STATUS_DIR/BLOCKERS.md" ]; then
    echo "### 🚨 Active Blockers"
    echo ""
    echo "**You have unresolved blockers.** The Master Orchestrator has been notified."
    echo ""
    echo '```markdown'
    cat "$PHASE_STATUS_DIR/BLOCKERS.md"
    echo '```'
    echo ""
    echo "**Wait for Master guidance before proceeding, or remove BLOCKERS.md if resolved.**"
    echo ""
fi

# Git status
echo "### Git Status"
echo ""

uncommitted=$(git status --porcelain 2>/dev/null | wc -l | tr -d ' ')
if [ "$uncommitted" -gt 0 ]; then
    echo "**Uncommitted changes:** $uncommitted files"
    echo ""
    echo '```'
    git status --short 2>/dev/null | head -10
    echo '```'
else
    echo "Working directory clean."
fi
echo ""

# Recent commits
echo "### Recent Commits"
echo ""
echo '```'
git log --oneline -5 2>/dev/null || echo "No commits yet"
echo '```'
echo ""

# Subagent guidelines reminder
echo "---"
echo ""
echo "### Subagent Guidelines"
echo ""
echo "**Remember:**"
echo "1. ✅ Test locally before committing"
echo "2. ✅ Update PROGRESS.md as you work"
echo "3. ✅ Create BLOCKERS.md if stuck (Master will be alerted)"
echo "4. ✅ Run all quality gates before marking complete"
echo "5. ❌ Do NOT merge to develop (Master does this)"
echo "6. ❌ Do NOT skip tests"
echo ""
echo "**When complete:** Create COMPLETED.md with test evidence."
