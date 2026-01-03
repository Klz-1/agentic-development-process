#!/bin/bash
#
# monitor-phase.sh
# Detailed monitoring for a single phase with live updates
#
# Usage: ./monitor-phase.sh <phase-directory>
#

set -e

# Configuration
PHASE_DIR="${1:-.}"
REFRESH_INTERVAL=3

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

# Validate phase directory
if [ ! -d "$PHASE_DIR" ]; then
    echo -e "${RED}Error: Directory not found: $PHASE_DIR${NC}"
    exit 1
fi

# Check for .phase-status
STATUS_DIR="$PHASE_DIR/.phase-status"
if [ ! -d "$STATUS_DIR" ]; then
    echo -e "${YELLOW}Warning: .phase-status directory not found${NC}"
    echo "Creating .phase-status directory..."
    mkdir -p "$STATUS_DIR"
fi

PHASE_NAME=$(basename "$PHASE_DIR")

# Get terminal dimensions
get_term_size() {
    TERM_COLS=$(tput cols 2>/dev/null || echo 80)
    TERM_ROWS=$(tput lines 2>/dev/null || echo 24)
}

# Draw a horizontal line
draw_line() {
    local char="${1:-─}"
    local width="${2:-$TERM_COLS}"
    printf '%*s' "$width" '' | tr ' ' "$char"
    echo ""
}

# Truncate text to fit width
truncate_text() {
    local text="$1"
    local max_width="${2:-$TERM_COLS}"

    if [ ${#text} -gt $max_width ]; then
        echo "${text:0:$((max_width-3))}..."
    else
        echo "$text"
    fi
}

# Get file age
get_file_age() {
    local file="$1"

    if [ ! -f "$file" ]; then
        echo "N/A"
        return
    fi

    local mod_time=$(stat -c %Y "$file" 2>/dev/null || stat -f %m "$file" 2>/dev/null)
    local now=$(date +%s)
    local diff=$((now - mod_time))

    if [ $diff -lt 60 ]; then
        echo "${diff}s"
    elif [ $diff -lt 3600 ]; then
        echo "$((diff / 60))m"
    elif [ $diff -lt 86400 ]; then
        echo "$((diff / 3600))h"
    else
        echo "$((diff / 86400))d"
    fi
}

# Display header
display_header() {
    echo -e "${BOLD}${MAGENTA}"
    echo "╔══════════════════════════════════════════════════════════════════════╗"
    printf "║  %-68s  ║\n" "PHASE MONITOR: $PHASE_NAME"
    echo "╚══════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# Display status files section
display_status_files() {
    echo -e "${BOLD}${CYAN}Status Files:${NC}"
    echo ""

    # Check each status file
    local files=("PROGRESS.md" "BLOCKERS.md" "COMPLETED.md" "QUESTIONS.md" "MASTER-NOTES.md")

    for file in "${files[@]}"; do
        local filepath="$STATUS_DIR/$file"
        local age=$(get_file_age "$filepath")
        local status=""
        local color=""

        if [ -f "$filepath" ]; then
            status="EXISTS"
            case "$file" in
                COMPLETED.md)
                    color="$GREEN"
                    status="COMPLETE"
                    ;;
                BLOCKERS.md)
                    color="$RED"
                    status="BLOCKED"
                    ;;
                QUESTIONS.md)
                    color="$YELLOW"
                    status="QUESTION"
                    ;;
                *)
                    color="$CYAN"
                    ;;
            esac
            echo -e "  ${color}$file${NC} (updated: $age ago)"
        else
            echo -e "  ${DIM}$file${NC} ${DIM}(not created)${NC}"
        fi
    done
    echo ""
}

# Display PROGRESS.md content
display_progress() {
    local progress_file="$STATUS_DIR/PROGRESS.md"

    echo -e "${BOLD}${CYAN}Progress Report:${NC}"
    draw_line "─" 72

    if [ -f "$progress_file" ]; then
        # Display with some formatting
        while IFS= read -r line; do
            case "$line" in
                "## "*)
                    echo -e "${BOLD}${BLUE}${line}${NC}"
                    ;;
                "- [x]"*)
                    echo -e "  ${GREEN}${line}${NC}"
                    ;;
                "- [ ]"*)
                    echo -e "  ${YELLOW}${line}${NC}"
                    ;;
                "# "*)
                    # Skip top-level headers (already shown)
                    ;;
                *)
                    echo "  $line"
                    ;;
            esac
        done < "$progress_file" | head -30
    else
        echo -e "  ${DIM}No progress report yet.${NC}"
        echo ""
        echo "  Create .phase-status/PROGRESS.md to track progress."
    fi
    echo ""
}

# Display git status
display_git_status() {
    echo -e "${BOLD}${CYAN}Git Status:${NC}"
    draw_line "─" 72

    if [ -d "$PHASE_DIR/.git" ] || [ -f "$PHASE_DIR/.git" ]; then
        # Branch and commit
        local branch=$(git -C "$PHASE_DIR" branch --show-current 2>/dev/null || echo "N/A")
        local commit=$(git -C "$PHASE_DIR" log --oneline -1 2>/dev/null || echo "No commits")

        echo -e "  ${BOLD}Branch:${NC} $branch"
        echo -e "  ${BOLD}Latest:${NC} $commit"
        echo ""

        # Recent commits
        echo -e "  ${BOLD}Recent commits:${NC}"
        git -C "$PHASE_DIR" log --oneline -5 2>/dev/null | while read line; do
            echo "    $line"
        done

        echo ""

        # Modified files count
        local modified=$(git -C "$PHASE_DIR" status --porcelain 2>/dev/null | wc -l)
        if [ "$modified" -gt 0 ]; then
            echo -e "  ${YELLOW}Modified files: $modified${NC}"
            git -C "$PHASE_DIR" status --porcelain 2>/dev/null | head -5 | while read line; do
                echo "    $line"
            done
            if [ "$modified" -gt 5 ]; then
                echo "    ... and $((modified - 5)) more"
            fi
        else
            echo -e "  ${GREEN}Working directory clean${NC}"
        fi
    else
        echo -e "  ${DIM}Not a git repository${NC}"
    fi
    echo ""
}

# Display blockers if present
display_blockers() {
    local blockers_file="$STATUS_DIR/BLOCKERS.md"

    if [ -f "$blockers_file" ]; then
        echo -e "${BOLD}${RED}ACTIVE BLOCKER:${NC}"
        draw_line "─" 72
        echo -e "${RED}"
        cat "$blockers_file" | head -15 | sed 's/^/  /'
        echo -e "${NC}"
        echo ""
    fi
}

# Display questions if present
display_questions() {
    local questions_file="$STATUS_DIR/QUESTIONS.md"

    if [ -f "$questions_file" ]; then
        echo -e "${BOLD}${YELLOW}PENDING QUESTION:${NC}"
        draw_line "─" 72
        echo -e "${YELLOW}"
        cat "$questions_file" | head -10 | sed 's/^/  /'
        echo -e "${NC}"
        echo ""
    fi
}

# Display master notes preview
display_master_notes() {
    local notes_file="$STATUS_DIR/MASTER-NOTES.md"

    if [ -f "$notes_file" ]; then
        local age=$(get_file_age "$notes_file")
        echo -e "${BOLD}${CYAN}Master Notes:${NC} (updated: $age ago)"
        draw_line "─" 72

        # Show just the tasks section
        if grep -q "## .* Tasks" "$notes_file" 2>/dev/null; then
            sed -n '/## .* Tasks/,/^## /p' "$notes_file" | head -15 | sed 's/^/  /'
        else
            head -10 "$notes_file" | sed 's/^/  /'
        fi
        echo ""
    fi
}

# Main display function
display_monitor() {
    get_term_size
    clear

    display_header

    echo -e "  ${DIM}Directory: $PHASE_DIR${NC}"
    echo -e "  ${DIM}Updated: $(date '+%Y-%m-%d %H:%M:%S')${NC}"
    echo ""

    display_status_files
    display_blockers
    display_questions
    display_progress
    display_git_status
    display_master_notes

    draw_line "─" 72
    echo -e "  ${DIM}Refreshing every ${REFRESH_INTERVAL}s... Press Ctrl+C to stop${NC}"
}

# Main loop
main() {
    while true; do
        display_monitor
        sleep $REFRESH_INTERVAL
    done
}

# Handle Ctrl+C
trap 'echo ""; echo "Phase monitor stopped."; exit 0' INT TERM

# Check for single run mode
if [ "$2" = "--once" ]; then
    display_monitor
else
    main
fi
