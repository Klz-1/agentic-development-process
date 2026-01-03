#!/bin/bash
#
# phase-status-overview.sh
# Displays a real-time overview of all phase statuses
#
# Usage: ./phase-status-overview.sh [project-dir]
#

# Configuration
PROJECT_DIR="${1:-$(pwd)}"
REFRESH_INTERVAL=10

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m'
BOLD='\033[1m'
DIM='\033[2m'

# Status icons
ICON_COMPLETE=""
ICON_PROGRESS=""
ICON_BLOCKED=""
ICON_PENDING=""
ICON_QUESTION=""

# Fallback to ASCII if Unicode not supported
if [[ "$TERM" != *"256color"* ]]; then
    ICON_COMPLETE="[OK]"
    ICON_PROGRESS="[..]"
    ICON_BLOCKED="[!!]"
    ICON_PENDING="[  ]"
    ICON_QUESTION="[? ]"
fi

# Find all phase directories
find_phase_dirs() {
    local parent_dir=$(dirname "$PROJECT_DIR")
    local dirs=()

    # Check .worktrees directory (preferred location)
    for dir in "$PROJECT_DIR/.worktrees"/phase-* "$PROJECT_DIR/.worktrees"/*; do
        if [ -d "$dir/.phase-status" ]; then
            dirs+=("$dir")
        fi
    done

    # Check git worktrees
    if [ -d "$PROJECT_DIR/.git" ]; then
        while IFS= read -r line; do
            local worktree_path=$(echo "$line" | awk '{print $1}')
            if [ -d "$worktree_path/.phase-status" ]; then
                if [[ ! " ${dirs[@]} " =~ " ${worktree_path} " ]]; then
                    dirs+=("$worktree_path")
                fi
            fi
        done < <(git -C "$PROJECT_DIR" worktree list 2>/dev/null)
    fi

    # Sort by name
    printf '%s\n' "${dirs[@]}" | sort
}

# Get phase status
get_phase_status() {
    local phase_dir="$1"
    local status_dir="$phase_dir/.phase-status"

    # Check for COMPLETED.md first
    if [ -f "$status_dir/COMPLETED.md" ]; then
        echo "complete"
        return
    fi

    # Check for BLOCKERS.md
    if [ -f "$status_dir/BLOCKERS.md" ]; then
        echo "blocked"
        return
    fi

    # Check for QUESTIONS.md
    if [ -f "$status_dir/QUESTIONS.md" ]; then
        echo "question"
        return
    fi

    # Check for PROGRESS.md (in progress)
    if [ -f "$status_dir/PROGRESS.md" ]; then
        echo "progress"
        return
    fi

    # No status files yet
    echo "pending"
}

# Get task progress from PROGRESS.md
get_task_progress() {
    local progress_file="$1"

    if [ ! -f "$progress_file" ]; then
        echo "0/0"
        return
    fi

    local completed=$(grep -c '\[x\]' "$progress_file" 2>/dev/null || echo 0)
    local total=$(grep -c '\[.\]' "$progress_file" 2>/dev/null || echo 0)

    echo "$completed/$total"
}

# Get last update time
get_last_update() {
    local file="$1"

    if [ ! -f "$file" ]; then
        echo "N/A"
        return
    fi

    local mod_time=$(stat -c %Y "$file" 2>/dev/null || stat -f %m "$file" 2>/dev/null)
    local now=$(date +%s)
    local diff=$((now - mod_time))

    if [ $diff -lt 60 ]; then
        echo "${diff}s ago"
    elif [ $diff -lt 3600 ]; then
        echo "$((diff / 60))m ago"
    elif [ $diff -lt 86400 ]; then
        echo "$((diff / 3600))h ago"
    else
        echo "$((diff / 86400))d ago"
    fi
}

# Get current task from PROGRESS.md
get_current_task() {
    local progress_file="$1"

    if [ ! -f "$progress_file" ]; then
        echo "Not started"
        return
    fi

    # Look for "Current Task" section or first uncompleted task
    local current=$(grep -A1 "## Current Task" "$progress_file" 2>/dev/null | tail -1 | sed 's/^[[:space:]]*//')

    if [ -z "$current" ] || [ "$current" = "## Current Task" ]; then
        # Get first uncompleted task
        current=$(grep '^\- \[ \]' "$progress_file" 2>/dev/null | head -1 | sed 's/^- \[ \] //')
    fi

    if [ -z "$current" ]; then
        current="Working..."
    fi

    # Truncate if too long
    if [ ${#current} -gt 40 ]; then
        current="${current:0:37}..."
    fi

    echo "$current"
}

# Get git branch and latest commit
get_git_info() {
    local phase_dir="$1"

    if [ ! -d "$phase_dir/.git" ] && [ ! -f "$phase_dir/.git" ]; then
        echo "N/A"
        return
    fi

    local branch=$(git -C "$phase_dir" branch --show-current 2>/dev/null)
    local commit=$(git -C "$phase_dir" log --oneline -1 2>/dev/null | cut -c1-7)

    if [ -n "$branch" ] && [ -n "$commit" ]; then
        echo "$branch ($commit)"
    elif [ -n "$branch" ]; then
        echo "$branch"
    else
        echo "N/A"
    fi
}

# Display the overview
display_overview() {
    clear

    echo -e "${BOLD}${MAGENTA}"
    echo "╔══════════════════════════════════════════════════════════════════════╗"
    echo "║               PHASE STATUS OVERVIEW                                  ║"
    echo "╚══════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo ""
    echo -e "  ${DIM}Project: $PROJECT_DIR${NC}"
    echo -e "  ${DIM}Updated: $(date '+%Y-%m-%d %H:%M:%S')${NC}"
    echo ""

    local phase_dirs=($(find_phase_dirs))

    if [ ${#phase_dirs[@]} -eq 0 ]; then
        echo -e "  ${YELLOW}No phase worktrees found.${NC}"
        echo ""
        echo "  Create phase worktrees with .phase-status directories:"
        echo "    git worktree add .worktrees/phase-1 -b feature/phase-1"
        echo "    mkdir -p .worktrees/phase-1/.phase-status"
        echo ""
        return
    fi

    # Summary statistics
    local total=${#phase_dirs[@]}
    local complete=0
    local progress=0
    local blocked=0
    local pending=0

    for phase_dir in "${phase_dirs[@]}"; do
        local status=$(get_phase_status "$phase_dir")
        case "$status" in
            complete) ((complete++)) ;;
            progress) ((progress++)) ;;
            blocked) ((blocked++)) ;;
            pending) ((pending++)) ;;
        esac
    done

    echo -e "  ${BOLD}Summary:${NC} ${GREEN}$complete complete${NC} | ${CYAN}$progress in progress${NC} | ${RED}$blocked blocked${NC} | ${DIM}$pending pending${NC}"
    echo ""
    echo "────────────────────────────────────────────────────────────────────────"
    echo ""

    # Display each phase
    for phase_dir in "${phase_dirs[@]}"; do
        local phase_name=$(basename "$phase_dir")
        local status=$(get_phase_status "$phase_dir")
        local status_dir="$phase_dir/.phase-status"
        local progress_file="$status_dir/PROGRESS.md"

        # Status indicator and color
        local status_icon=""
        local status_color=""
        local status_text=""

        case "$status" in
            complete)
                status_icon="$ICON_COMPLETE"
                status_color="$GREEN"
                status_text="Complete"
                ;;
            progress)
                status_icon="$ICON_PROGRESS"
                status_color="$CYAN"
                status_text="In Progress"
                ;;
            blocked)
                status_icon="$ICON_BLOCKED"
                status_color="$RED"
                status_text="BLOCKED"
                ;;
            question)
                status_icon="$ICON_QUESTION"
                status_color="$YELLOW"
                status_text="Has Question"
                ;;
            pending)
                status_icon="$ICON_PENDING"
                status_color="$DIM"
                status_text="Pending"
                ;;
        esac

        # Get details
        local task_progress=$(get_task_progress "$progress_file")
        local last_update=$(get_last_update "$progress_file")
        local current_task=$(get_current_task "$progress_file")
        local git_info=$(get_git_info "$phase_dir")

        # Display
        echo -e "  ${status_color}${status_icon}${NC} ${BOLD}$phase_name${NC}"
        echo -e "     Status: ${status_color}$status_text${NC}  |  Tasks: $task_progress  |  Updated: $last_update"
        echo -e "     ${DIM}Current: $current_task${NC}"
        echo -e "     ${DIM}Branch: $git_info${NC}"

        # Show blocker or question preview if applicable
        if [ "$status" = "blocked" ] && [ -f "$status_dir/BLOCKERS.md" ]; then
            local blocker_preview=$(head -5 "$status_dir/BLOCKERS.md" | grep -v '^#' | head -1 | sed 's/^[[:space:]]*//')
            if [ -n "$blocker_preview" ]; then
                echo -e "     ${RED}Blocker: $blocker_preview${NC}"
            fi
        fi

        if [ "$status" = "question" ] && [ -f "$status_dir/QUESTIONS.md" ]; then
            local question_preview=$(head -5 "$status_dir/QUESTIONS.md" | grep -v '^#' | head -1 | sed 's/^[[:space:]]*//')
            if [ -n "$question_preview" ]; then
                echo -e "     ${YELLOW}Question: $question_preview${NC}"
            fi
        fi

        echo ""
    done

    echo "────────────────────────────────────────────────────────────────────────"
    echo ""
    echo -e "  ${DIM}Refreshing every ${REFRESH_INTERVAL}s... Press Ctrl+C to stop${NC}"
}

# Main loop
main() {
    while true; do
        display_overview
        sleep $REFRESH_INTERVAL
    done
}

# Handle Ctrl+C
trap 'echo ""; echo "Overview stopped."; exit 0' INT TERM

# Check if running in watch mode or single shot
if [ "$2" = "--once" ]; then
    display_overview
else
    main
fi
