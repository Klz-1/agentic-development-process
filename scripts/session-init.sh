#!/bin/bash
#
# session-init.sh
# Run this script at the start of every subagent session for instant context recovery
#
# Usage: ./scripts/session-init.sh [phase-dir]
#
# If no phase-dir is provided, uses current directory
#

set -e

# Configuration
PHASE_DIR="${1:-$(pwd)}"
MAIN_REPO=""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'
BOLD='\033[1m'
DIM='\033[2m'

# Find main repo (parent of .worktrees or the repo itself)
find_main_repo() {
    local dir="$PHASE_DIR"

    # Check if we're in a worktree
    if [ -f "$dir/.git" ]; then
        # .git is a file in worktrees, contains path to main repo
        MAIN_REPO=$(cat "$dir/.git" | grep "gitdir:" | sed 's/gitdir: //' | sed 's/\.git\/worktrees\/.*//')
    elif [ -d "$dir/.git" ]; then
        MAIN_REPO="$dir"
    else
        # Try parent directories
        while [ "$dir" != "/" ]; do
            if [ -d "$dir/.git" ]; then
                MAIN_REPO="$dir"
                break
            fi
            dir=$(dirname "$dir")
        done
    fi
}

# Get phase name from directory
get_phase_name() {
    basename "$PHASE_DIR"
}

# Get current branch
get_branch() {
    git -C "$PHASE_DIR" branch --show-current 2>/dev/null || echo "unknown"
}

# Get last session state if exists
get_last_session_state() {
    local state_file="$PHASE_DIR/.phase-status/SESSION-STATE.md"
    if [ -f "$state_file" ]; then
        local last_update=$(stat -c %Y "$state_file" 2>/dev/null || stat -f %m "$state_file" 2>/dev/null)
        local now=$(date +%s)
        local diff=$((now - last_update))

        if [ $diff -lt 86400 ]; then  # Less than 24 hours
            echo "found"
            return
        fi
    fi
    echo "none"
}

# Check for MASTER-NOTES updates
check_master_notes() {
    local notes_file="$PHASE_DIR/.phase-status/MASTER-NOTES.md"
    local state_file="$PHASE_DIR/.phase-status/SESSION-STATE.md"

    if [ ! -f "$notes_file" ]; then
        echo "none"
        return
    fi

    local notes_time=$(stat -c %Y "$notes_file" 2>/dev/null || stat -f %m "$notes_file" 2>/dev/null)

    if [ -f "$state_file" ]; then
        local state_time=$(stat -c %Y "$state_file" 2>/dev/null || stat -f %m "$state_file" 2>/dev/null)
        if [ "$notes_time" -gt "$state_time" ]; then
            echo "updated"
            return
        fi
    else
        echo "unread"
        return
    fi

    echo "read"
}

# Check for conflicts with develop
check_conflicts() {
    cd "$PHASE_DIR"

    # Fetch latest develop silently
    git fetch origin develop 2>/dev/null || true

    # Try merge without committing
    if git merge origin/develop --no-commit --no-ff 2>/dev/null; then
        git merge --abort 2>/dev/null || true
        echo "clean"
    else
        git merge --abort 2>/dev/null || true
        echo "conflicts"
    fi
}

# Get test status
get_test_status() {
    cd "$PHASE_DIR"

    if [ -f "package.json" ]; then
        if grep -q '"test"' package.json && ! grep -q '"test":\s*"echo' package.json; then
            if npm test --silent 2>/dev/null; then
                echo "passing"
            else
                echo "failing"
            fi
        else
            echo "none"
        fi
    elif [ -f "pyproject.toml" ] || [ -f "setup.py" ]; then
        if command -v pytest &> /dev/null; then
            if pytest --quiet 2>/dev/null; then
                echo "passing"
            else
                echo "failing"
            fi
        else
            echo "unknown"
        fi
    else
        echo "unknown"
    fi
}

# Get progress from PROGRESS.md
get_progress() {
    local progress_file="$PHASE_DIR/.phase-status/PROGRESS.md"

    if [ ! -f "$progress_file" ]; then
        echo "0/0"
        return
    fi

    local completed=$(grep -c '\[x\]' "$progress_file" 2>/dev/null || echo 0)
    local total=$(grep -c '\[.\]' "$progress_file" 2>/dev/null || echo 0)

    echo "$completed/$total"
}

# Display the session init report
display_report() {
    clear

    echo ""
    echo -e "${BOLD}${MAGENTA}╔══════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BOLD}${MAGENTA}║                    SESSION INITIALIZATION                            ║${NC}"
    echo -e "${BOLD}${MAGENTA}╚══════════════════════════════════════════════════════════════════════╝${NC}"
    echo ""

    local phase_name=$(get_phase_name)
    local branch=$(get_branch)
    local progress=$(get_progress)

    echo -e "  ${BOLD}Phase:${NC}    $phase_name"
    echo -e "  ${BOLD}Branch:${NC}   $branch"
    echo -e "  ${BOLD}Progress:${NC} $progress tasks"
    echo -e "  ${BOLD}Time:${NC}     $(date '+%Y-%m-%d %H:%M:%S')"
    echo ""

    echo -e "${CYAN}────────────────────────────────────────────────────────────────────────${NC}"
    echo ""

    # Check for previous session state
    echo -e "${BOLD}Session Recovery:${NC}"
    local session_state=$(get_last_session_state)
    if [ "$session_state" = "found" ]; then
        echo -e "  ${GREEN}✓${NC} Previous session state found"
        echo ""
        echo -e "  ${DIM}Last session summary:${NC}"
        head -20 "$PHASE_DIR/.phase-status/SESSION-STATE.md" 2>/dev/null | sed 's/^/    /'
    else
        echo -e "  ${YELLOW}⚠${NC} No previous session state (first session or state expired)"
    fi
    echo ""

    echo -e "${CYAN}────────────────────────────────────────────────────────────────────────${NC}"
    echo ""

    # Check MASTER-NOTES
    echo -e "${BOLD}Master Communications:${NC}"
    local notes_status=$(check_master_notes)
    case "$notes_status" in
        updated)
            echo -e "  ${RED}★ NEW UPDATES in MASTER-NOTES.md - READ IMMEDIATELY${NC}"
            echo ""
            echo -e "  ${YELLOW}Recent content:${NC}"
            tail -20 "$PHASE_DIR/.phase-status/MASTER-NOTES.md" 2>/dev/null | sed 's/^/    /'
            ;;
        unread)
            echo -e "  ${YELLOW}⚠${NC} MASTER-NOTES.md exists but may be unread"
            ;;
        read)
            echo -e "  ${GREEN}✓${NC} MASTER-NOTES.md - no new updates"
            ;;
        none)
            echo -e "  ${DIM}No MASTER-NOTES.md file yet${NC}"
            ;;
    esac
    echo ""

    echo -e "${CYAN}────────────────────────────────────────────────────────────────────────${NC}"
    echo ""

    # Recent commits
    echo -e "${BOLD}Recent Commits (last 5):${NC}"
    git -C "$PHASE_DIR" log --oneline -5 2>/dev/null | sed 's/^/  /' || echo "  No commits yet"
    echo ""

    echo -e "${CYAN}────────────────────────────────────────────────────────────────────────${NC}"
    echo ""

    # Conflict check
    echo -e "${BOLD}Develop Branch Status:${NC}"
    echo -e "  ${DIM}Checking for potential conflicts...${NC}"
    local conflict_status=$(check_conflicts)
    if [ "$conflict_status" = "clean" ]; then
        echo -e "  ${GREEN}✓${NC} No conflicts with develop branch"
    else
        echo -e "  ${RED}✗${NC} CONFLICTS DETECTED with develop branch"
        echo -e "  ${YELLOW}  Run: git fetch origin develop && git merge origin/develop${NC}"
        echo -e "  ${YELLOW}  Resolve conflicts before continuing work${NC}"
    fi
    echo ""

    echo -e "${CYAN}────────────────────────────────────────────────────────────────────────${NC}"
    echo ""

    # Test status (optional - can be slow)
    echo -e "${BOLD}Quick Checks:${NC}"

    # Check for uncommitted changes
    local uncommitted=$(git -C "$PHASE_DIR" status --porcelain 2>/dev/null | wc -l)
    if [ "$uncommitted" -gt 0 ]; then
        echo -e "  ${YELLOW}⚠${NC} $uncommitted uncommitted changes"
    else
        echo -e "  ${GREEN}✓${NC} Working directory clean"
    fi

    # Check for BLOCKERS.md
    if [ -f "$PHASE_DIR/.phase-status/BLOCKERS.md" ]; then
        echo -e "  ${RED}✗${NC} BLOCKERS.md exists - review and resolve"
    fi

    # Check for COMPLETED.md
    if [ -f "$PHASE_DIR/.phase-status/COMPLETED.md" ]; then
        echo -e "  ${GREEN}★${NC} COMPLETED.md exists - phase marked complete"
    fi

    echo ""
    echo -e "${CYAN}────────────────────────────────────────────────────────────────────────${NC}"
    echo ""

    # Current task from PROGRESS.md
    local progress_file="$PHASE_DIR/.phase-status/PROGRESS.md"
    if [ -f "$progress_file" ]; then
        echo -e "${BOLD}Current Task:${NC}"
        local current_task=$(grep -A1 "## Current Task" "$progress_file" 2>/dev/null | tail -1 | sed 's/^[[:space:]]*//')
        if [ -n "$current_task" ] && [ "$current_task" != "## Current Task" ]; then
            echo -e "  $current_task"
        else
            # Get first incomplete task
            local next_task=$(grep '^\- \[ \]' "$progress_file" 2>/dev/null | head -1 | sed 's/^- \[ \] //')
            if [ -n "$next_task" ]; then
                echo -e "  Next: $next_task"
            else
                echo -e "  ${GREEN}All tasks complete!${NC}"
            fi
        fi
    else
        echo -e "${BOLD}Current Task:${NC}"
        echo -e "  ${YELLOW}No PROGRESS.md - create one to track tasks${NC}"
    fi

    echo ""
    echo -e "${CYAN}════════════════════════════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "${BOLD}${GREEN}Session initialized. Ready to work!${NC}"
    echo ""
    echo -e "${DIM}Remember to:${NC}"
    echo -e "${DIM}  1. Update PROGRESS.md as you work${NC}"
    echo -e "${DIM}  2. Create BLOCKERS.md if stuck${NC}"
    echo -e "${DIM}  3. Run ./scripts/session-save.sh before ending${NC}"
    echo ""
}

# Main
main() {
    if [ ! -d "$PHASE_DIR" ]; then
        echo -e "${RED}Error: Directory not found: $PHASE_DIR${NC}"
        exit 1
    fi

    find_main_repo
    display_report
}

main
