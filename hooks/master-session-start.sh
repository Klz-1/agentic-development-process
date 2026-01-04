#!/bin/bash
#
# master-session-start.sh - Claude Code SessionStart Hook for Master Orchestrator
#
# This hook runs when the Master Orchestrator starts a Claude Code session.
# It provides an overview of all phases and highlights items needing attention.
#
# Output is fed directly to Claude as context.
#

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"

echo "## Master Orchestrator Session"
echo ""
echo "**Role:** Master Orchestrator"
echo "**Project:** $(basename "$PROJECT_DIR")"
echo "**Time:** $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# Check for .worktrees directory
WORKTREES_DIR="$PROJECT_DIR/.worktrees"
if [ ! -d "$WORKTREES_DIR" ]; then
    echo "### No Phase Worktrees Found"
    echo ""
    echo "No `.worktrees/` directory detected. Either:"
    echo "- This is a new project - create phase worktrees to begin"
    echo "- You're in the wrong directory"
    echo ""
    exit 0
fi

# Count phases
TOTAL_PHASES=0
BLOCKED_PHASES=0
COMPLETED_PHASES=0
IN_PROGRESS_PHASES=0
NEEDS_ATTENTION=()

echo "### Phase Status Overview"
echo ""
echo "| Phase | Branch | Status | Action Needed |"
echo "|-------|--------|--------|---------------|"

for phase_dir in "$WORKTREES_DIR"/*/; do
    if [ ! -d "$phase_dir" ]; then
        continue
    fi

    phase_name=$(basename "$phase_dir")
    TOTAL_PHASES=$((TOTAL_PHASES + 1))

    # Get branch
    branch=$(git -C "$phase_dir" branch --show-current 2>/dev/null || echo "unknown")

    # Determine status
    status="In Progress"
    action=""

    if [ -f "$phase_dir/.phase-status/BLOCKERS.md" ]; then
        status="🚨 BLOCKED"
        action="**Resolve blocker**"
        BLOCKED_PHASES=$((BLOCKED_PHASES + 1))
        NEEDS_ATTENTION+=("$phase_name: BLOCKED - Check BLOCKERS.md")
    elif [ -f "$phase_dir/.phase-status/COMPLETED.md" ]; then
        status="✅ Complete"
        action="**Review & create PR**"
        COMPLETED_PHASES=$((COMPLETED_PHASES + 1))
        NEEDS_ATTENTION+=("$phase_name: Ready for review")
    elif [ -f "$phase_dir/.phase-status/QUESTIONS.md" ]; then
        status="❓ Has Questions"
        action="**Answer questions**"
        NEEDS_ATTENTION+=("$phase_name: Has questions")
    else
        IN_PROGRESS_PHASES=$((IN_PROGRESS_PHASES + 1))

        # Check for recent activity
        if [ -f "$phase_dir/.phase-status/PROGRESS.md" ]; then
            last_update=$(stat -c %Y "$phase_dir/.phase-status/PROGRESS.md" 2>/dev/null || stat -f %m "$phase_dir/.phase-status/PROGRESS.md" 2>/dev/null)
            now=$(date +%s)
            hours_ago=$(( (now - last_update) / 3600 ))

            if [ $hours_ago -gt 2 ]; then
                action="Check progress (${hours_ago}h since update)"
            fi
        fi
    fi

    echo "| $phase_name | \`$branch\` | $status | $action |"
done

echo ""

# Summary stats
echo "### Summary"
echo ""
echo "- **Total Phases:** $TOTAL_PHASES"
echo "- **In Progress:** $IN_PROGRESS_PHASES"
echo "- **Completed:** $COMPLETED_PHASES"
echo "- **Blocked:** $BLOCKED_PHASES"
echo ""

# Attention needed section
if [ ${#NEEDS_ATTENTION[@]} -gt 0 ]; then
    echo "### ⚠️ Immediate Attention Required"
    echo ""
    for item in "${NEEDS_ATTENTION[@]}"; do
        echo "- $item"
    done
    echo ""
fi

# Show blocked phases details
if [ $BLOCKED_PHASES -gt 0 ]; then
    echo "### 🚨 Blocked Phases - Details"
    echo ""
    for phase_dir in "$WORKTREES_DIR"/*/; do
        if [ -f "$phase_dir/.phase-status/BLOCKERS.md" ]; then
            phase_name=$(basename "$phase_dir")
            echo "#### $phase_name"
            echo ""
            echo '```'
            head -30 "$phase_dir/.phase-status/BLOCKERS.md"
            echo '```'
            echo ""
        fi
    done
fi

# Show completed phases ready for review
if [ $COMPLETED_PHASES -gt 0 ]; then
    echo "### ✅ Phases Ready for Review"
    echo ""
    for phase_dir in "$WORKTREES_DIR"/*/; do
        if [ -f "$phase_dir/.phase-status/COMPLETED.md" ]; then
            phase_name=$(basename "$phase_dir")
            branch=$(git -C "$phase_dir" branch --show-current 2>/dev/null)
            echo "- **$phase_name** (branch: \`$branch\`)"

            # Show summary from COMPLETED.md
            summary=$(grep -A2 "## Summary" "$phase_dir/.phase-status/COMPLETED.md" 2>/dev/null | tail -2)
            if [ -n "$summary" ]; then
                echo "  $summary"
            fi
        fi
    done
    echo ""
fi

# Check for open PRs
echo "### Open Pull Requests"
echo ""
if command -v gh &> /dev/null; then
    prs=$(gh pr list --state open 2>/dev/null)
    if [ -n "$prs" ]; then
        echo '```'
        echo "$prs"
        echo '```'
    else
        echo "No open PRs found."
    fi
else
    echo "Install \`gh\` CLI to see PR status."
fi
echo ""

# Recent activity across all phases
echo "### Recent Activity (All Phases)"
echo ""
echo '```'
for phase_dir in "$WORKTREES_DIR"/*/; do
    if [ -d "$phase_dir/.git" ] || [ -f "$phase_dir/.git" ]; then
        phase_name=$(basename "$phase_dir")
        latest=$(git -C "$phase_dir" log --oneline -1 2>/dev/null)
        if [ -n "$latest" ]; then
            echo "$phase_name: $latest"
        fi
    fi
done
echo '```'
echo ""

echo "---"
echo ""
echo "**Master Orchestrator Responsibilities:**"
echo "1. Check blocked phases and provide guidance"
echo "2. Review completed phases and create PRs"
echo "3. Answer subagent questions"
echo "4. Monitor overall progress"
echo "5. Present merge recommendations to user"
