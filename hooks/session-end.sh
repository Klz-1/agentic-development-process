#!/bin/bash
#
# session-end.sh - Claude Code Stop Hook for Session State Preservation
#
# This hook runs when Claude finishes responding.
# It auto-saves session state for the next session.
#
# Uses --quiet mode to minimize output.
#

# Find project root
PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"

# Check if we're in a phase worktree
if [ ! -d "$PROJECT_DIR/.phase-status" ] && [ ! -d ".phase-status" ]; then
    # Not in a phase directory, skip
    exit 0
fi

# Determine phase directory
if [ -d ".phase-status" ]; then
    PHASE_DIR="$(pwd)"
else
    PHASE_DIR="$PROJECT_DIR"
fi

# Find session-save script
SCRIPT_LOCATIONS=(
    "$PHASE_DIR/scripts/session-save.sh"
    "$PHASE_DIR/../scripts/session-save.sh"
    "./scripts/session-save.sh"
    "../scripts/session-save.sh"
)

SAVE_SCRIPT=""
for loc in "${SCRIPT_LOCATIONS[@]}"; do
    if [ -x "$loc" ]; then
        SAVE_SCRIPT="$loc"
        break
    fi
done

if [ -n "$SAVE_SCRIPT" ]; then
    # Run in quiet auto mode
    "$SAVE_SCRIPT" --quiet "$PHASE_DIR" 2>/dev/null
fi

# Always exit successfully
exit 0
