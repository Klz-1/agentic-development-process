#!/bin/bash
#
# master-session-end.sh - Claude Code Stop Hook for Master Orchestrator
#
# This hook runs when the Master Orchestrator's Claude session ends.
# It saves the orchestration state and updates the master dashboard.
#

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
COORDINATION_DIR="$PROJECT_DIR/.coordination"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Ensure coordination directory exists
mkdir -p "$COORDINATION_DIR"

# Create/update master session state
STATE_FILE="$COORDINATION_DIR/MASTER-SESSION-STATE.md"

# Gather phase statuses
WORKTREES_DIR="$PROJECT_DIR/.worktrees"
PHASE_SUMMARY=""

if [ -d "$WORKTREES_DIR" ]; then
    for phase_dir in "$WORKTREES_DIR"/*/; do
        if [ -d "$phase_dir" ]; then
            phase_name=$(basename "$phase_dir")

            if [ -f "$phase_dir/.phase-status/BLOCKERS.md" ]; then
                status="BLOCKED"
            elif [ -f "$phase_dir/.phase-status/COMPLETED.md" ]; then
                status="Complete"
            else
                status="In Progress"
            fi

            PHASE_SUMMARY="$PHASE_SUMMARY\n- $phase_name: $status"
        fi
    done
fi

# Get recent Master actions from git
RECENT_ACTIONS=$(git log --oneline -5 --all 2>/dev/null | head -5)

# Save master session state
cat > "$STATE_FILE" << EOF
# Master Orchestrator Session State

**Last Session:** $TIMESTAMP
**Mode:** Auto-saved

---

## Phase Status Snapshot
$PHASE_SUMMARY

---

## Recent Activity

\`\`\`
$RECENT_ACTIONS
\`\`\`

---

## Session Notes

*Auto-saved at session end. Review phase worktrees for detailed status.*

---

## Next Session Priorities

1. Check for blocked phases
2. Review completed phases
3. Answer pending questions
4. Monitor progress

EOF

# Update the main progress dashboard if it exists
DASHBOARD="$PROJECT_DIR/docs/PROGRESS.md"
if [ -f "$DASHBOARD" ]; then
    # Add session log entry if section exists
    if grep -q "## Master Session Log" "$DASHBOARD" 2>/dev/null; then
        echo "" >> "$DASHBOARD"
        echo "### $TIMESTAMP" >> "$DASHBOARD"
        echo "- Session ended (auto-saved)" >> "$DASHBOARD"
    fi
fi

echo "Master session state saved: $STATE_FILE"
exit 0
