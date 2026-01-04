#!/bin/bash
#
# session-start.sh - Claude Code SessionStart Hook (Auto-detecting)
#
# This hook automatically detects whether you're the Master Orchestrator
# or a Subagent based on the directory structure:
#
#   - Main repo (has .worktrees/) → Master Orchestrator
#   - Phase worktree (has .phase-status/) → Subagent
#
# Install ONCE and it works for both roles.
#

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"

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
        # Check if this is a worktree (not main working tree)
        local git_dir=$(git rev-parse --git-dir 2>/dev/null)
        if [[ "$git_dir" == *"/worktrees/"* ]]; then
            echo "subagent"
            return
        fi
    fi

    # Default: unknown (not an agentic project)
    echo "unknown"
}

ROLE=$(detect_role)

# ============================================================================
# MASTER ORCHESTRATOR SESSION START
# ============================================================================

master_session_start() {
    echo "## Master Orchestrator Session"
    echo ""
    echo "**Role:** Master Orchestrator"
    echo "**Project:** $(basename "$PROJECT_DIR")"
    echo "**Time:** $(date '+%Y-%m-%d %H:%M:%S')"
    echo ""

    # Find worktrees directory
    local WORKTREES_DIR="$PROJECT_DIR/.worktrees"
    [ ! -d "$WORKTREES_DIR" ] && WORKTREES_DIR=".worktrees"

    if [ ! -d "$WORKTREES_DIR" ]; then
        echo "### No Phase Worktrees Found"
        echo ""
        echo "Create phase worktrees to begin: \`git worktree add .worktrees/phase-1 -b feature/phase-1-name\`"
        return
    fi

    # Count and categorize phases
    local TOTAL=0 BLOCKED=0 COMPLETED=0 IN_PROGRESS=0
    local NEEDS_ATTENTION=()

    echo "### Phase Status Overview"
    echo ""
    echo "| Phase | Branch | Status | Action Needed |"
    echo "|-------|--------|--------|---------------|"

    for phase_dir in "$WORKTREES_DIR"/*/; do
        [ ! -d "$phase_dir" ] && continue

        local phase_name=$(basename "$phase_dir")
        TOTAL=$((TOTAL + 1))

        local branch=$(git -C "$phase_dir" branch --show-current 2>/dev/null || echo "unknown")
        local status="In Progress"
        local action=""

        if [ -f "$phase_dir/.phase-status/BLOCKERS.md" ]; then
            status="🚨 BLOCKED"
            action="**Resolve blocker**"
            BLOCKED=$((BLOCKED + 1))
            NEEDS_ATTENTION+=("$phase_name: BLOCKED - needs guidance")
        elif [ -f "$phase_dir/.phase-status/COMPLETED.md" ]; then
            status="✅ Complete"
            action="**Review & create PR**"
            COMPLETED=$((COMPLETED + 1))
            NEEDS_ATTENTION+=("$phase_name: Ready for review")
        elif [ -f "$phase_dir/.phase-status/QUESTIONS.md" ]; then
            status="❓ Has Questions"
            action="**Answer questions**"
            NEEDS_ATTENTION+=("$phase_name: Has questions")
        else
            IN_PROGRESS=$((IN_PROGRESS + 1))
        fi

        echo "| $phase_name | \`$branch\` | $status | $action |"
    done

    echo ""
    echo "### Summary"
    echo ""
    echo "- **Total Phases:** $TOTAL"
    echo "- **In Progress:** $IN_PROGRESS"
    echo "- **Completed:** $COMPLETED"
    echo "- **Blocked:** $BLOCKED"
    echo ""

    if [ ${#NEEDS_ATTENTION[@]} -gt 0 ]; then
        echo "### ⚠️ Immediate Attention Required"
        echo ""
        for item in "${NEEDS_ATTENTION[@]}"; do
            echo "- $item"
        done
        echo ""
    fi

    # Show blocked details
    if [ $BLOCKED -gt 0 ]; then
        echo "### 🚨 Blocked Phases - Details"
        echo ""
        for phase_dir in "$WORKTREES_DIR"/*/; do
            if [ -f "$phase_dir/.phase-status/BLOCKERS.md" ]; then
                echo "#### $(basename "$phase_dir")"
                echo '```'
                head -20 "$phase_dir/.phase-status/BLOCKERS.md"
                echo '```'
                echo ""
            fi
        done
    fi

    echo "---"
    echo ""
    echo "**Master Responsibilities:** Check blockers → Review completed phases → Answer questions → Monitor progress"
}

# ============================================================================
# SUBAGENT SESSION START
# ============================================================================

subagent_session_start() {
    # Find phase directory
    local PHASE_DIR="$PROJECT_DIR"
    [ -d ".phase-status" ] && PHASE_DIR="$(pwd)"

    local PHASE_STATUS_DIR="$PHASE_DIR/.phase-status"
    local PHASE_NAME=$(basename "$PHASE_DIR")
    local BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")

    echo "## Subagent Session - $PHASE_NAME"
    echo ""
    echo "**Role:** Subagent (Phase Worker)"
    echo "**Phase:** $PHASE_NAME"
    echo "**Branch:** \`$BRANCH\`"
    echo "**Time:** $(date '+%Y-%m-%d %H:%M:%S')"
    echo ""

    # CRITICAL: Show MASTER-NOTES first
    if [ -f "$PHASE_STATUS_DIR/MASTER-NOTES.md" ]; then
        echo "### 📋 Master Orchestrator Notes"
        echo ""
        echo "**IMPORTANT:** Review guidance from the Master:"
        echo ""
        echo '```markdown'
        cat "$PHASE_STATUS_DIR/MASTER-NOTES.md"
        echo '```'
        echo ""
    fi

    # Previous session state
    if [ -f "$PHASE_STATUS_DIR/SESSION-STATE.md" ]; then
        echo "### Previous Session"
        echo ""
        local working_on=$(grep "^\*\*Working On:\*\*" "$PHASE_STATUS_DIR/SESSION-STATE.md" | sed 's/\*\*Working On:\*\* //')
        local status=$(grep "^\*\*Status:\*\*" "$PHASE_STATUS_DIR/SESSION-STATE.md" | sed 's/\*\*Status:\*\* //')
        echo "- **Working on:** ${working_on:-Not specified}"
        echo "- **Status:** ${status:-Not specified}"
        echo ""
    fi

    # Task progress
    if [ -f "$PHASE_STATUS_DIR/PROGRESS.md" ]; then
        local completed=$(grep -c '\[x\]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null || echo 0)
        local total=$(grep -c '\[.\]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null || echo 0)
        local current=$(grep '^\- \[ \]' "$PHASE_STATUS_DIR/PROGRESS.md" 2>/dev/null | head -1 | sed 's/^- \[ \] //')

        echo "### Tasks"
        echo ""
        echo "**Progress:** $completed/$total complete"
        [ -n "$current" ] && echo "**Current:** $current"
        echo ""
    fi

    # Blockers warning
    if [ -f "$PHASE_STATUS_DIR/BLOCKERS.md" ]; then
        echo "### 🚨 Active Blockers"
        echo ""
        echo "**You have unresolved blockers.** Wait for Master guidance."
        echo ""
    fi

    # Git status
    local uncommitted=$(git status --porcelain 2>/dev/null | wc -l | tr -d ' ')
    if [ "$uncommitted" -gt 0 ]; then
        echo "### Git Status"
        echo ""
        echo "**$uncommitted uncommitted changes**"
        echo ""
    fi

    echo "---"
    echo ""
    echo "**Remember:** Test before commit • Update PROGRESS.md • Create BLOCKERS.md if stuck • Do NOT merge"
}

# ============================================================================
# MAIN
# ============================================================================

case "$ROLE" in
    master)
        master_session_start
        ;;
    subagent)
        subagent_session_start
        ;;
    *)
        # Not an agentic project, exit silently
        exit 0
        ;;
esac
