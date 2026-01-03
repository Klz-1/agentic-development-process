#!/bin/bash
#
# tmux-hooks.sh
# Configure tmux hooks for automatic session management
#
# Usage:
#   ./scripts/tmux-hooks.sh setup    # Install tmux hooks
#   ./scripts/tmux-hooks.sh remove   # Remove tmux hooks
#
# This script configures tmux to:
#   - Auto-save session state when a pane closes
#   - Auto-save session state when a window closes
#   - Auto-init when a new pane is created in a phase directory
#

SCRIPTS_DIR="$(cd "$(dirname "$0")" && pwd)"

setup_hooks() {
    echo "Setting up tmux session management hooks..."
    echo ""

    # Hook: Save session when pane closes
    tmux set-hook -g pane-exited "run-shell '$SCRIPTS_DIR/session-save.sh --quiet #{pane_current_path}'"

    # Hook: Save session when window closes
    tmux set-hook -g window-unlinked "run-shell '$SCRIPTS_DIR/session-save.sh --quiet #{pane_current_path}'"

    # Hook: Show session init info when pane gains focus (if in phase dir)
    # Note: This can be noisy, so it's commented out by default
    # tmux set-hook -g pane-focus-in "run-shell 'if [ -d #{pane_current_path}/.phase-status ]; then echo \"Phase: \$(basename #{pane_current_path})\"; fi'"

    echo "✓ Tmux hooks installed:"
    echo "  • pane-exited: Auto-saves session state"
    echo "  • window-unlinked: Auto-saves session state"
    echo ""
    echo "Hooks are active for this tmux server session."
    echo ""
    echo "To make permanent, add to your tmux.conf:"
    echo ""
    echo "  # Agentic Development Process - Session Hooks"
    echo "  set-hook -g pane-exited \"run-shell '$SCRIPTS_DIR/session-save.sh --quiet #{pane_current_path}'\""
    echo "  set-hook -g window-unlinked \"run-shell '$SCRIPTS_DIR/session-save.sh --quiet #{pane_current_path}'\""
    echo ""
}

remove_hooks() {
    echo "Removing tmux session management hooks..."

    tmux set-hook -gu pane-exited 2>/dev/null || true
    tmux set-hook -gu window-unlinked 2>/dev/null || true
    tmux set-hook -gu pane-focus-in 2>/dev/null || true

    echo "✓ Tmux hooks removed"
}

show_status() {
    echo "Current tmux hooks:"
    echo ""
    tmux show-hooks -g 2>/dev/null | grep -E "(pane-exited|window-unlinked|pane-focus)" || echo "  No agentic hooks found"
    echo ""
}

case "${1:-status}" in
    setup|install)
        setup_hooks
        ;;
    remove|uninstall)
        remove_hooks
        ;;
    status)
        show_status
        ;;
    *)
        echo "Usage: $0 {setup|remove|status}"
        echo ""
        echo "Commands:"
        echo "  setup   - Install tmux hooks for auto session management"
        echo "  remove  - Remove tmux hooks"
        echo "  status  - Show current hook status"
        exit 1
        ;;
esac
