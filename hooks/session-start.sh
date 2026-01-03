#!/bin/bash
#
# session-start.sh - Claude Code SessionStart Hook
#
# This hook runs automatically when a Claude Code session starts.
# It provides context about the current phase and session state.
#
# Output is fed directly to Claude as context.
#

# Find project root (where .claude directory is)
PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"

# Check if we're in a phase worktree
PHASE_STATUS_DIR=""
if [ -d "$PROJECT_DIR/.phase-status" ]; then
    PHASE_STATUS_DIR="$PROJECT_DIR/.phase-status"
elif [ -d ".phase-status" ]; then
    PHASE_STATUS_DIR=".phase-status"
    PROJECT_DIR="$(pwd)"
fi

# If not in a phase directory, check if we're in main repo with worktrees
if [ -z "$PHASE_STATUS_DIR" ] && [ -d "$PROJECT_DIR/.worktrees" ]; then
    echo "## Agentic Development Session"
    echo ""
    echo "**Location:** Main repository (not in a phase worktree)"
    echo ""
    echo "### Active Phase Worktrees:"
    for wt in "$PROJECT_DIR/.worktrees"/*/; do
        if [ -d "$wt/.phase-status" ]; then
            phase_name=$(basename "$wt")
            branch=$(git -C "$wt" branch --show-current 2>/dev/null || echo "unknown")
            echo "- **$phase_name** (branch: \`$branch\`)"

            # Check for blockers
            if [ -f "$wt/.phase-status/BLOCKERS.md" ]; then
                echo "  - ⚠️ Has blockers - needs attention"
            fi

            # Check for completion
            if [ -f "$wt/.phase-status/COMPLETED.md" ]; then
                echo "  - ✅ Marked complete - ready for review"
            fi
        fi
    done
    echo ""
    echo "**Tip:** Navigate to a phase worktree to work on that phase."
    exit 0
fi

# Not in an agentic project
if [ -z "$PHASE_STATUS_DIR" ]; then
    exit 0
fi

# We're in a phase directory - provide full context
PHASE_NAME=$(basename "$PROJECT_DIR")
BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")

echo "## Agentic Development Session - Phase Context"
echo ""
echo "**Phase:** $PHASE_NAME"
echo "**Branch:** \`$BRANCH\`"
echo "**Time:** $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# Check for previous session state
if [ -f "$PHASE_STATUS_DIR/SESSION-STATE.md" ]; then
    echo "### Previous Session State"
    echo ""

    # Extract key info from session state
    working_on=$(grep "^\*\*Working On:\*\*" "$PHASE_STATUS_DIR/SESSION-STATE.md" | sed 's/\*\*Working On:\*\* //')
    status=$(grep "^\*\*Status:\*\*" "$PHASE_STATUS_DIR/SESSION-STATE.md" | sed 's/\*\*Status:\*\* //')
    current_task=$(grep "^\*\*Current Task:\*\*" "$PHASE_STATUS_DIR/SESSION-STATE.md" | sed 's/\*\*Current Task:\*\* //')

    echo "- **Last working on:** $working_on"
    echo "- **Status:** $status"
    echo "- **Current task:** $current_task"
    echo ""

    # Show priority actions if present
    if grep -q "### Priority Actions" "$PHASE_STATUS_DIR/SESSION-STATE.md"; then
        echo "### Priority Actions"
        sed -n '/### Priority Actions/,/###/p' "$PHASE_STATUS_DIR/SESSION-STATE.md" | grep -E "^[0-9]|^-|^Continue" | head -5
        echo ""
    fi
fi

# Check for MASTER-NOTES updates
if [ -f "$PHASE_STATUS_DIR/MASTER-NOTES.md" ]; then
    # Check if newer than session state
    if [ -f "$PHASE_STATUS_DIR/SESSION-STATE.md" ]; then
        notes_time=$(stat -c %Y "$PHASE_STATUS_DIR/MASTER-NOTES.md" 2>/dev/null || stat -f %m "$PHASE_STATUS_DIR/MASTER-NOTES.md" 2>/dev/null)
        state_time=$(stat -c %Y "$PHASE_STATUS_DIR/SESSION-STATE.md" 2>/dev/null || stat -f %m "$PHASE_STATUS_DIR/SESSION-STATE.md" 2>/dev/null)

        if [ "$notes_time" -gt "$state_time" ]; then
            echo "### ⚠️ NEW Master Notes (Updated Since Last Session)"
            echo ""
            echo "**IMPORTANT:** Review MASTER-NOTES.md for new guidance from the Master Orchestrator."
            echo ""
            # Show last section
            echo "Recent content:"
            echo '```'
            tail -20 "$PHASE_STATUS_DIR/MASTER-NOTES.md"
            echo '```'
            echo ""
        fi
    fi
fi

# Check for blockers
if [ -f "$PHASE_STATUS_DIR/BLOCKERS.md" ]; then
    echo "### 🚨 Active Blockers"
    echo ""
    echo "This phase has unresolved blockers. Review BLOCKERS.md before continuing."
    echo ""
fi

# Show progress
if [ -f "$PHASE_STATUS_DIR/PROGRESS.md" ]; then
    completed=$(grep -c '\[x\]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null || echo 0)
    total=$(grep -c '\[.\]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null || echo 0)

    if [ "$total" -gt 0 ]; then
        echo "### Progress"
        echo ""
        echo "**Tasks:** $completed/$total complete"

        # Show current/next task
        current_task=$(grep '^\- \[ \]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null | head -1 | sed 's/^- \[ \] //')
        if [ -n "$current_task" ]; then
            echo "**Next task:** $current_task"
        fi
        echo ""
    fi
fi

# Check for uncommitted changes
uncommitted=$(git status --porcelain 2>/dev/null | wc -l | tr -d ' ')
if [ "$uncommitted" -gt 0 ]; then
    echo "### Uncommitted Changes"
    echo ""
    echo "**$uncommitted files** with uncommitted changes."
    echo ""
fi

# Recent commits
echo "### Recent Commits"
echo '```'
git log --oneline -3 2>/dev/null || echo "No commits yet"
echo '```'
echo ""

echo "---"
echo ""
echo "**Session initialized.** Review the above context and continue with the current task."
