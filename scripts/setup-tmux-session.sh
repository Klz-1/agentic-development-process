#!/bin/bash
#
# setup-tmux-session.sh
# Creates a tmux session for agentic development orchestration
#
# Usage: ./setup-tmux-session.sh [project-dir] [session-name]
#

set -e

# Configuration
PROJECT_DIR="${1:-$(pwd)}"
SESSION_NAME="${2:-agentic-dev}"
PHASE_DIRS=()

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    if ! command -v tmux &> /dev/null; then
        log_error "tmux is not installed. Please install it first."
        echo "  Ubuntu/Debian: sudo apt-get install tmux"
        echo "  macOS: brew install tmux"
        exit 1
    fi
    log_success "tmux found: $(tmux -V)"

    # Check for inotifywait (Linux) or fswatch (macOS)
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if ! command -v inotifywait &> /dev/null; then
            log_warn "inotifywait not found. Alerts may not work."
            echo "  Install: sudo apt-get install inotify-tools"
        else
            log_success "inotifywait found"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        if ! command -v fswatch &> /dev/null; then
            log_warn "fswatch not found. Alerts may not work."
            echo "  Install: brew install fswatch"
        else
            log_success "fswatch found"
        fi
    fi
}

# Find phase worktrees
find_phase_dirs() {
    log_info "Scanning for phase worktrees..."

    # Look for directories in .worktrees (preferred location)
    for dir in "$PROJECT_DIR/.worktrees"/phase-* "$PROJECT_DIR/.worktrees"/*; do
        if [ -d "$dir" ] && [ -d "$dir/.phase-status" ]; then
            PHASE_DIRS+=("$dir")
            log_success "Found phase: $(basename $dir)"
        fi
    done

    # Also check for worktrees in git
    if [ -d "$PROJECT_DIR/.git" ]; then
        while IFS= read -r line; do
            worktree_path=$(echo "$line" | awk '{print $1}')
            if [ -d "$worktree_path/.phase-status" ]; then
                # Avoid duplicates
                if [[ ! " ${PHASE_DIRS[@]} " =~ " ${worktree_path} " ]]; then
                    PHASE_DIRS+=("$worktree_path")
                    log_success "Found worktree: $(basename $worktree_path)"
                fi
            fi
        done < <(git -C "$PROJECT_DIR" worktree list 2>/dev/null)
    fi

    if [ ${#PHASE_DIRS[@]} -eq 0 ]; then
        log_warn "No phase directories found. Only dashboard will be created."
    else
        log_info "Found ${#PHASE_DIRS[@]} phase directories"
    fi
}

# Kill existing session if it exists
cleanup_session() {
    if tmux has-session -t "$SESSION_NAME" 2>/dev/null; then
        log_warn "Session '$SESSION_NAME' already exists."
        read -p "Kill existing session? (y/N) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            tmux kill-session -t "$SESSION_NAME"
            log_success "Killed existing session"
        else
            log_error "Cannot create session with same name. Exiting."
            exit 1
        fi
    fi
}

# Create the dashboard window
create_dashboard() {
    log_info "Creating dashboard window..."

    # Create session with dashboard window
    tmux new-session -d -s "$SESSION_NAME" -n dashboard -c "$PROJECT_DIR"

    # Main pane: Master PROGRESS.md
    if [ -f "$PROJECT_DIR/docs/PROGRESS.md" ]; then
        tmux send-keys -t "$SESSION_NAME:dashboard" "watch -n 10 'cat docs/PROGRESS.md 2>/dev/null || echo \"PROGRESS.md not found\"'" Enter
    else
        tmux send-keys -t "$SESSION_NAME:dashboard" "echo 'Create docs/PROGRESS.md to see master dashboard here'" Enter
    fi

    # Split for phase status overview
    tmux split-window -t "$SESSION_NAME:dashboard" -h -p 50 -c "$PROJECT_DIR"
    tmux send-keys -t "$SESSION_NAME:dashboard.1" "$(dirname "$0")/phase-status-overview.sh '$PROJECT_DIR' 2>/dev/null || echo 'Waiting for phases...'" Enter

    # Split for alert monitor
    tmux split-window -t "$SESSION_NAME:dashboard.1" -v -p 50 -c "$PROJECT_DIR"
    tmux send-keys -t "$SESSION_NAME:dashboard.2" "$(dirname "$0")/alert-on-change.sh '$PROJECT_DIR'" Enter

    log_success "Dashboard window created"
}

# Create a phase window
create_phase_window() {
    local phase_dir="$1"
    local phase_name=$(basename "$phase_dir")
    local window_name="${phase_name//-/_}"  # Replace hyphens with underscores for tmux

    log_info "Creating window for $phase_name..."

    # Create new window
    tmux new-window -t "$SESSION_NAME" -n "$window_name" -c "$phase_dir"

    # Main pane: Working directory
    tmux send-keys -t "$SESSION_NAME:$window_name" "echo '=== $phase_name Subagent Workspace ===' && pwd && ls -la" Enter

    # Split bottom for monitoring
    tmux split-window -t "$SESSION_NAME:$window_name" -v -p 30 -c "$phase_dir"

    # Progress monitor
    if [ -f "$phase_dir/.phase-status/PROGRESS.md" ]; then
        tmux send-keys -t "$SESSION_NAME:$window_name.1" "watch -n 5 'cat .phase-status/PROGRESS.md 2>/dev/null || echo \"No progress yet\"'" Enter
    else
        tmux send-keys -t "$SESSION_NAME:$window_name.1" "echo 'Waiting for .phase-status/PROGRESS.md...'" Enter
    fi

    # Split for git log
    tmux split-window -t "$SESSION_NAME:$window_name.1" -h -p 50 -c "$phase_dir"
    tmux send-keys -t "$SESSION_NAME:$window_name.2" "watch -n 10 'git log --oneline -8 2>/dev/null || echo \"No git history\"'" Enter

    log_success "Window created: $window_name"
}

# Create git operations window
create_git_window() {
    log_info "Creating git operations window..."

    tmux new-window -t "$SESSION_NAME" -n git-ops -c "$PROJECT_DIR"

    # Git status and operations pane
    tmux send-keys -t "$SESSION_NAME:git-ops" "echo '=== Git Operations ===' && git status" Enter

    # Split for git log
    tmux split-window -t "$SESSION_NAME:git-ops" -h -p 50 -c "$PROJECT_DIR"
    tmux send-keys -t "$SESSION_NAME:git-ops.1" "watch -n 15 'git log --all --oneline --graph -15'" Enter

    log_success "Git operations window created"
}

# Generate session info file
save_session_info() {
    local info_file="$PROJECT_DIR/.coordination/tmux-session-info.md"

    mkdir -p "$PROJECT_DIR/.coordination"

    cat > "$info_file" << EOF
# Tmux Session Information

**Session Name:** $SESSION_NAME
**Created:** $(date)
**Project Directory:** $PROJECT_DIR

## Windows

| # | Name | Purpose |
|---|------|---------|
| 0 | dashboard | Master orchestration overview |
EOF

    local i=1
    for phase_dir in "${PHASE_DIRS[@]}"; do
        local phase_name=$(basename "$phase_dir")
        echo "| $i | ${phase_name//-/_} | $phase_name workspace |" >> "$info_file"
        ((i++))
    done

    echo "| $i | git-ops | Git operations & merging |" >> "$info_file"

    cat >> "$info_file" << EOF

## Quick Commands

\`\`\`bash
# Attach to session
tmux attach -t $SESSION_NAME

# Detach (inside tmux)
Ctrl-b d

# Switch windows
Ctrl-b 0  # Dashboard
Ctrl-b 1  # First phase
Ctrl-b n  # Next window
\`\`\`

## Phase Directories

EOF

    for phase_dir in "${PHASE_DIRS[@]}"; do
        echo "- $phase_dir" >> "$info_file"
    done

    log_success "Session info saved to: $info_file"
}

# Main function
main() {
    echo ""
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║       AGENTIC DEVELOPMENT TMUX SESSION SETUP                 ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo ""

    check_prerequisites
    echo ""

    # Validate project directory
    if [ ! -d "$PROJECT_DIR" ]; then
        log_error "Project directory not found: $PROJECT_DIR"
        exit 1
    fi
    log_info "Project directory: $PROJECT_DIR"

    find_phase_dirs
    echo ""

    cleanup_session
    echo ""

    log_info "Creating tmux session: $SESSION_NAME"
    echo ""

    create_dashboard

    for phase_dir in "${PHASE_DIRS[@]}"; do
        create_phase_window "$phase_dir"
    done

    create_git_window

    save_session_info

    # Go back to dashboard
    tmux select-window -t "$SESSION_NAME:dashboard"

    echo ""
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                    SESSION READY!                            ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo ""
    echo "  Session:  $SESSION_NAME"
    echo "  Windows:  $((${#PHASE_DIRS[@]} + 2)) (dashboard + ${#PHASE_DIRS[@]} phases + git-ops)"
    echo ""
    echo "  To attach: tmux attach -t $SESSION_NAME"
    echo ""

    # Optionally attach
    read -p "Attach to session now? (Y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Nn]$ ]]; then
        tmux attach -t "$SESSION_NAME"
    fi
}

# Run main
main
