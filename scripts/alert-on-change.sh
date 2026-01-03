#!/bin/bash
#
# alert-on-change.sh
# Monitors phase worktrees for important file changes and alerts
#
# Usage: ./alert-on-change.sh [project-dir]
#

set -e

# Configuration
PROJECT_DIR="${1:-$(pwd)}"
WATCH_PATTERNS="BLOCKERS.md|COMPLETED.md|QUESTIONS.md|PROGRESS.md"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'
BOLD='\033[1m'

# Notification function (cross-platform)
send_notification() {
    local title="$1"
    local message="$2"
    local urgency="${3:-normal}"  # low, normal, critical

    # Linux with libnotify
    if command -v notify-send &> /dev/null; then
        notify-send -u "$urgency" "$title" "$message" 2>/dev/null || true
    fi

    # macOS
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if command -v terminal-notifier &> /dev/null; then
            terminal-notifier -title "$title" -message "$message" 2>/dev/null || true
        else
            osascript -e "display notification \"$message\" with title \"$title\"" 2>/dev/null || true
        fi
    fi
}

# Play sound alert (if available)
play_sound() {
    local type="$1"  # alert, complete, info

    case "$type" in
        alert)
            # Linux
            if command -v paplay &> /dev/null; then
                paplay /usr/share/sounds/freedesktop/stereo/alarm-clock-elapsed.oga 2>/dev/null &
            fi
            # macOS
            if [[ "$OSTYPE" == "darwin"* ]]; then
                afplay /System/Library/Sounds/Sosumi.aiff 2>/dev/null &
            fi
            ;;
        complete)
            if [[ "$OSTYPE" == "darwin"* ]]; then
                afplay /System/Library/Sounds/Glass.aiff 2>/dev/null &
            fi
            ;;
    esac
}

# Find all phase directories
find_phase_dirs() {
    local parent_dir=$(dirname "$PROJECT_DIR")
    local dirs=()

    # Look for phase directories
    # Check .worktrees directory first (preferred location)
    for dir in "$PROJECT_DIR/.worktrees"/phase-* "$PROJECT_DIR/.worktrees"/*; do
        if [ -d "$dir/.phase-status" ]; then
            dirs+=("$dir/.phase-status")
        fi
    done

    # Check git worktrees
    if [ -d "$PROJECT_DIR/.git" ]; then
        while IFS= read -r line; do
            local worktree_path=$(echo "$line" | awk '{print $1}')
            if [ -d "$worktree_path/.phase-status" ]; then
                local status_dir="$worktree_path/.phase-status"
                if [[ ! " ${dirs[@]} " =~ " ${status_dir} " ]]; then
                    dirs+=("$status_dir")
                fi
            fi
        done < <(git -C "$PROJECT_DIR" worktree list 2>/dev/null)
    fi

    echo "${dirs[@]}"
}

# Process file change event
process_event() {
    local path="$1"
    local event="$2"
    local file="$3"
    local timestamp=$(date '+%H:%M:%S')
    local phase_name=$(basename $(dirname "$path"))

    case "$file" in
        BLOCKERS.md)
            echo -e "${RED}${BOLD}"
            echo "╔══════════════════════════════════════════════════════════════╗"
            echo "║  BLOCKER DETECTED                                      ║"
            echo "╚══════════════════════════════════════════════════════════════╝"
            echo -e "${NC}"
            echo -e "  ${RED}Time:${NC}  $timestamp"
            echo -e "  ${RED}Phase:${NC} $phase_name"
            echo -e "  ${RED}File:${NC}  $path$file"
            echo ""
            echo -e "  ${YELLOW}ACTION REQUIRED: Check subagent immediately${NC}"
            echo ""

            # Show blocker content if file exists
            if [ -f "$path$file" ]; then
                echo -e "  ${CYAN}--- Blocker Content ---${NC}"
                sed 's/^/  /' "$path$file" | head -20
                echo -e "  ${CYAN}------------------------${NC}"
            fi
            echo ""

            send_notification "BLOCKER: $phase_name" "Subagent needs help! Check immediately." "critical"
            play_sound "alert"
            ;;

        COMPLETED.md)
            echo -e "${GREEN}${BOLD}"
            echo "╔══════════════════════════════════════════════════════════════╗"
            echo "║  PHASE COMPLETE                                         ║"
            echo "╚══════════════════════════════════════════════════════════════╝"
            echo -e "${NC}"
            echo -e "  ${GREEN}Time:${NC}  $timestamp"
            echo -e "  ${GREEN}Phase:${NC} $phase_name"
            echo -e "  ${GREEN}File:${NC}  $path$file"
            echo ""
            echo -e "  ${CYAN}ACTION: Queue for code review${NC}"
            echo ""

            send_notification "COMPLETE: $phase_name" "Phase ready for review!" "normal"
            play_sound "complete"
            ;;

        QUESTIONS.md)
            echo -e "${YELLOW}${BOLD}"
            echo "╔══════════════════════════════════════════════════════════════╗"
            echo "║  QUESTION FROM SUBAGENT                                  ║"
            echo "╚══════════════════════════════════════════════════════════════╝"
            echo -e "${NC}"
            echo -e "  ${YELLOW}Time:${NC}  $timestamp"
            echo -e "  ${YELLOW}Phase:${NC} $phase_name"
            echo -e "  ${YELLOW}File:${NC}  $path$file"
            echo ""
            echo -e "  ${CYAN}ACTION: Respond in MASTER-NOTES.md${NC}"
            echo ""

            # Show question content
            if [ -f "$path$file" ]; then
                echo -e "  ${CYAN}--- Question ---${NC}"
                sed 's/^/  /' "$path$file" | head -15
                echo -e "  ${CYAN}----------------${NC}"
            fi
            echo ""

            send_notification "Question: $phase_name" "Subagent has a question" "normal"
            ;;

        PROGRESS.md)
            echo -e "${BLUE}[$timestamp]${NC} Progress update: ${CYAN}$phase_name${NC}"
            ;;
    esac
}

# Linux monitoring with inotifywait
monitor_linux() {
    local watch_dirs="$1"

    echo -e "${CYAN}Starting inotifywait monitor...${NC}"
    echo ""

    inotifywait -m -r -e create,modify,moved_to \
        --format '%w %e %f' \
        $watch_dirs 2>/dev/null | while read path event file; do

        # Filter for files we care about
        if [[ "$file" =~ ($WATCH_PATTERNS) ]]; then
            process_event "$path" "$event" "$file"
        fi
    done
}

# macOS monitoring with fswatch
monitor_macos() {
    local watch_dirs="$1"

    echo -e "${CYAN}Starting fswatch monitor...${NC}"
    echo ""

    fswatch -r --event Created --event Updated --event Renamed \
        $watch_dirs 2>/dev/null | while read filepath; do

        local file=$(basename "$filepath")
        local path=$(dirname "$filepath")/

        if [[ "$file" =~ ($WATCH_PATTERNS) ]]; then
            process_event "$path" "modified" "$file"
        fi
    done
}

# Fallback polling monitor
monitor_polling() {
    local watch_dirs="$1"
    local interval=5

    echo -e "${YELLOW}Using polling mode (inotifywait/fswatch not available)${NC}"
    echo ""

    # Store initial checksums
    declare -A checksums

    while true; do
        for dir in $watch_dirs; do
            for pattern in BLOCKERS.md COMPLETED.md QUESTIONS.md PROGRESS.md; do
                local file="$dir/$pattern"
                if [ -f "$file" ]; then
                    local current_sum=$(md5sum "$file" 2>/dev/null | awk '{print $1}' || md5 -q "$file" 2>/dev/null)
                    local stored_sum="${checksums[$file]}"

                    if [ -n "$current_sum" ] && [ "$current_sum" != "$stored_sum" ]; then
                        if [ -n "$stored_sum" ]; then
                            # File changed
                            process_event "$dir/" "modified" "$pattern"
                        fi
                        checksums[$file]="$current_sum"
                    fi
                fi
            done
        done
        sleep $interval
    done
}

# Main function
main() {
    echo ""
    echo -e "${BOLD}${MAGENTA}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║         AGENTIC DEVELOPMENT ALERT MONITOR                    ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo ""
    echo -e "  ${CYAN}Project:${NC} $PROJECT_DIR"
    echo -e "  ${CYAN}Started:${NC} $(date)"
    echo ""

    # Find phase directories
    local watch_dirs=$(find_phase_dirs)

    if [ -z "$watch_dirs" ]; then
        echo -e "${YELLOW}No phase directories found.${NC}"
        echo "Create phase worktrees with .phase-status directories to monitor."
        echo ""
        echo "Waiting for phases to be created..."
        echo ""

        # Wait for phases
        while true; do
            watch_dirs=$(find_phase_dirs)
            if [ -n "$watch_dirs" ]; then
                echo -e "${GREEN}Phase directories detected!${NC}"
                break
            fi
            sleep 10
        done
    fi

    echo -e "${CYAN}Watching directories:${NC}"
    for dir in $watch_dirs; do
        echo "  - $dir"
    done
    echo ""
    echo -e "${CYAN}Monitoring for:${NC}"
    echo "  - BLOCKERS.md  (subagent stuck)"
    echo "  - COMPLETED.md (phase done)"
    echo "  - QUESTIONS.md (subagent has questions)"
    echo "  - PROGRESS.md  (progress updates)"
    echo ""
    echo -e "${GREEN}Ready. Waiting for events...${NC}"
    echo "────────────────────────────────────────────────────────────────"
    echo ""

    # Choose monitoring method based on OS
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if command -v inotifywait &> /dev/null; then
            monitor_linux "$watch_dirs"
        else
            monitor_polling "$watch_dirs"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        if command -v fswatch &> /dev/null; then
            monitor_macos "$watch_dirs"
        else
            monitor_polling "$watch_dirs"
        fi
    else
        monitor_polling "$watch_dirs"
    fi
}

# Handle Ctrl+C gracefully
trap 'echo ""; echo "Alert monitor stopped."; exit 0' INT TERM

# Run main
main
