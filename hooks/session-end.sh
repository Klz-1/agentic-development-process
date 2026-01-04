#!/bin/bash
#
# session-end.sh - Claude Code Stop Hook (Auto-detecting)
#
# This hook automatically detects whether you're the Master Orchestrator
# or a Subagent based on the directory structure and saves appropriate state.
#
# Install ONCE and it works for both roles.
#

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# ============================================================================
# ROLE DETECTION
# ============================================================================

detect_role() {
    # Check if we're in a phase worktree (Subagent)
    if [ -d "$PROJECT_DIR/.phase-status" ] || [ -d ".phase-status" ]; then
        echo "subagent"
        return
    fi

    # Check if we're in main repo with worktrees (Master)
    if [ -d "$PROJECT_DIR/.worktrees" ] || [ -d ".worktrees" ]; then
        echo "master"
        return
    fi

    # Check if current dir is inside a .worktrees path (Subagent)
    if [[ "$PROJECT_DIR" == *".worktrees"* ]] || [[ "$(pwd)" == *".worktrees"* ]]; then
        echo "subagent"
        return
    fi

    # Check git worktree status
    if git rev-parse --is-inside-work-tree &>/dev/null; then
        local git_dir=$(git rev-parse --git-dir 2>/dev/null)
        if [[ "$git_dir" == *"/worktrees/"* ]]; then
            echo "subagent"
            return
        fi
    fi

    echo "unknown"
}

ROLE=$(detect_role)

# ============================================================================
# MASTER SESSION END
# ============================================================================

master_session_end() {
    local COORDINATION_DIR="$PROJECT_DIR/.coordination"
    [ ! -d "$COORDINATION_DIR" ] && COORDINATION_DIR=".coordination"
    mkdir -p "$COORDINATION_DIR"

    local STATE_FILE="$COORDINATION_DIR/MASTER-SESSION-STATE.md"
    local WORKTREES_DIR="$PROJECT_DIR/.worktrees"
    [ ! -d "$WORKTREES_DIR" ] && WORKTREES_DIR=".worktrees"

    # Gather phase statuses
    local PHASE_SUMMARY=""
    if [ -d "$WORKTREES_DIR" ]; then
        for phase_dir in "$WORKTREES_DIR"/*/; do
            [ ! -d "$phase_dir" ] && continue
            local phase_name=$(basename "$phase_dir")
            local status="In Progress"

            [ -f "$phase_dir/.phase-status/BLOCKERS.md" ] && status="BLOCKED"
            [ -f "$phase_dir/.phase-status/COMPLETED.md" ] && status="Complete"

            PHASE_SUMMARY="$PHASE_SUMMARY
- $phase_name: $status"
        done
    fi

    cat > "$STATE_FILE" << EOF
# Master Orchestrator Session State

**Saved:** $TIMESTAMP
**Mode:** Auto-saved

---

## Phase Status Snapshot
$PHASE_SUMMARY

---

## Next Session Priorities

1. Check for blocked phases
2. Review completed phases
3. Answer pending questions
4. Monitor progress

EOF

    echo "Master session state saved"
}

# ============================================================================
# SUBAGENT SESSION END
# ============================================================================

subagent_session_end() {
    local PHASE_DIR="$PROJECT_DIR"
    [ -d ".phase-status" ] && PHASE_DIR="$(pwd)"

    local PHASE_STATUS_DIR="$PHASE_DIR/.phase-status"
    mkdir -p "$PHASE_STATUS_DIR"

    local STATE_FILE="$PHASE_STATUS_DIR/SESSION-STATE.md"
    local PHASE_NAME=$(basename "$PHASE_DIR")
    local BRANCH=$(git -C "$PHASE_DIR" branch --show-current 2>/dev/null || echo "unknown")
    local LAST_COMMIT=$(git -C "$PHASE_DIR" log --oneline -1 2>/dev/null || echo "No commits")
    local UNCOMMITTED=$(git -C "$PHASE_DIR" status --porcelain 2>/dev/null | wc -l | tr -d ' ')

    # Get current task
    local CURRENT_TASK=""
    local PROGRESS="unknown"
    if [ -f "$PHASE_STATUS_DIR/PROGRESS.md" ]; then
        CURRENT_TASK=$(grep '^\- \[ \]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null | head -1 | sed 's/^- \[ \] //')
        local completed=$(grep -c '\[x\]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null || echo 0)
        local total=$(grep -c '\[.\]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null || echo 0)
        PROGRESS="$completed/$total"
    fi

    # Infer status
    local STATUS="In progress"
    [ -f "$PHASE_STATUS_DIR/COMPLETED.md" ] && STATUS="Phase complete"
    [ -f "$PHASE_STATUS_DIR/BLOCKERS.md" ] && STATUS="BLOCKED"
    [ "$UNCOMMITTED" -gt 0 ] && STATUS="$STATUS - $UNCOMMITTED uncommitted"

    # Infer working on from last commit
    local WORKING_ON=$(git -C "$PHASE_DIR" log --format=%s -1 2>/dev/null || echo "See commits")

    # Recent commits
    local RECENT_COMMITS=$(git -C "$PHASE_DIR" log --oneline -5 2>/dev/null)

    cat > "$STATE_FILE" << EOF
# Session State - $PHASE_NAME

**Saved:** $TIMESTAMP
**Branch:** $BRANCH
**Progress:** $PROGRESS tasks
**Mode:** Auto-saved

---

## Last Session Summary

**Working On:** $WORKING_ON

**Status:** $STATUS

**Current Task:** ${CURRENT_TASK:-"Check PROGRESS.md"}

---

## For Next Session

1. ${CURRENT_TASK:-"Review PROGRESS.md for next task"}
2. Check MASTER-NOTES.md for guidance
3. Continue implementation

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

### Uncommitted Changes: $UNCOMMITTED files

EOF

    echo "Subagent session state saved"
}

# ============================================================================
# MAIN
# ============================================================================

case "$ROLE" in
    master)
        master_session_end
        ;;
    subagent)
        subagent_session_end
        ;;
    *)
        exit 0
        ;;
esac
