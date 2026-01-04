#!/bin/bash
#
# subagent-session-end.sh - Claude Code Stop Hook for Subagents
#
# This hook runs when a Subagent's Claude session ends.
# It saves the phase-specific session state.
#

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"

# Check if we're in a phase directory
if [ ! -d "$PROJECT_DIR/.phase-status" ] && [ ! -d ".phase-status" ]; then
    exit 0
fi

# Determine phase directory
if [ -d ".phase-status" ]; then
    PHASE_DIR="$(pwd)"
else
    PHASE_DIR="$PROJECT_DIR"
fi

PHASE_STATUS_DIR="$PHASE_DIR/.phase-status"
STATE_FILE="$PHASE_STATUS_DIR/SESSION-STATE.md"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Gather state information
PHASE_NAME=$(basename "$PHASE_DIR")
BRANCH=$(git -C "$PHASE_DIR" branch --show-current 2>/dev/null || echo "unknown")
LAST_COMMIT=$(git -C "$PHASE_DIR" log --oneline -1 2>/dev/null || echo "No commits")
UNCOMMITTED=$(git -C "$PHASE_DIR" status --porcelain 2>/dev/null | wc -l | tr -d ' ')

# Get current task from PROGRESS.md
CURRENT_TASK=""
PROGRESS_FILE="$PHASE_STATUS_DIR/PROGRESS.md"
if [ -f "$PROGRESS_FILE" ]; then
    CURRENT_TASK=$(grep '^\- \[ \]' "$PROGRESS_FILE" 2>/dev/null | head -1 | sed 's/^- \[ \] //')
    COMPLETED=$(grep -c '\[x\]' "$PROGRESS_FILE" 2>/dev/null || echo 0)
    TOTAL=$(grep -c '\[.\]' "$PROGRESS_FILE" 2>/dev/null || echo 0)
    PROGRESS="$COMPLETED/$TOTAL"
else
    PROGRESS="unknown"
fi

# Infer status
if [ -f "$PHASE_STATUS_DIR/COMPLETED.md" ]; then
    STATUS="Phase complete - awaiting review"
elif [ -f "$PHASE_STATUS_DIR/BLOCKERS.md" ]; then
    STATUS="BLOCKED - awaiting Master guidance"
elif [ "$UNCOMMITTED" -gt 0 ]; then
    STATUS="In progress - $UNCOMMITTED uncommitted changes"
else
    STATUS="In progress"
fi

# Infer what was being worked on from last commit
WORKING_ON=$(git -C "$PHASE_DIR" log --format=%s -1 2>/dev/null || echo "See commits")

# Get modified files
MODIFIED_FILES=$(git -C "$PHASE_DIR" status --porcelain 2>/dev/null | head -10)

# Recent commits
RECENT_COMMITS=$(git -C "$PHASE_DIR" log --oneline -5 2>/dev/null)

# Save session state
cat > "$STATE_FILE" << EOF
# Session State - $PHASE_NAME

**Saved:** $TIMESTAMP
**Branch:** $BRANCH
**Progress:** $PROGRESS tasks
**Mode:** Auto-saved (Subagent)

---

## Last Session Summary

**Working On:** $WORKING_ON

**Status:** $STATUS

**Current Task:** ${CURRENT_TASK:-"Check PROGRESS.md"}

---

## For Next Session

### Priority Actions
1. ${CURRENT_TASK:-"Review PROGRESS.md for next task"}
2. Check MASTER-NOTES.md for any new guidance
3. Continue implementation

### Blockers/Concerns
$([ -f "$PHASE_STATUS_DIR/BLOCKERS.md" ] && echo "See BLOCKERS.md" || echo "None")

---

## Technical State

### Last Commit
\`\`\`
$LAST_COMMIT
\`\`\`

### Recent Commits
\`\`\`
$RECENT_COMMITS
\`\`\`

### Uncommitted Changes
**Count:** $UNCOMMITTED files

\`\`\`
$MODIFIED_FILES
\`\`\`

---

*Auto-saved at session end. Run session-init to recover context.*
EOF

echo "Subagent session state saved: $STATE_FILE"
exit 0
