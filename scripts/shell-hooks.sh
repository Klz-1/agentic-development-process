#!/bin/bash
#
# shell-hooks.sh
# Source this file to enable automatic session management in your shell
#
# Usage:
#   source /path/to/scripts/shell-hooks.sh
#
# Or add to your ~/.bashrc or ~/.zshrc:
#   source /path/to/agentic-development-process/scripts/shell-hooks.sh
#
# Features:
#   - Auto-saves session state when shell exits (if in a phase directory)
#   - Auto-runs session-init when cd'ing into a phase directory
#   - Periodic auto-save every 10 minutes (optional)
#

# Store the script directory for finding other scripts
AGENTIC_SCRIPTS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ============================================================================
# EXIT TRAP - Auto-save on shell exit
# ============================================================================

_agentic_exit_handler() {
    # Check if we're in a phase directory
    if [ -d ".phase-status" ]; then
        # Find session-save script
        local save_script=""
        for loc in "./scripts/session-save.sh" "../scripts/session-save.sh" "$AGENTIC_SCRIPTS_DIR/session-save.sh"; do
            if [ -x "$loc" ]; then
                save_script="$loc"
                break
            fi
        done

        if [ -n "$save_script" ]; then
            echo ""
            echo "Auto-saving session state on exit..."
            "$save_script" --quiet "$(pwd)"
        fi
    fi
}

# Register the exit handler
trap _agentic_exit_handler EXIT

# ============================================================================
# CD HOOK - Auto-init when entering phase directory
# ============================================================================

# Store original cd function
if [ -n "$ZSH_VERSION" ]; then
    # Zsh
    _agentic_original_cd() {
        builtin cd "$@"
    }
else
    # Bash
    _agentic_original_cd() {
        builtin cd "$@"
    }
fi

# Override cd to detect phase directories
cd() {
    _agentic_original_cd "$@" || return $?

    # Check if we entered a phase directory
    if [ -d ".phase-status" ]; then
        # Find session-init script
        local init_script=""
        for loc in "./scripts/session-init.sh" "../scripts/session-init.sh" "$AGENTIC_SCRIPTS_DIR/session-init.sh"; do
            if [ -x "$loc" ]; then
                init_script="$loc"
                break
            fi
        done

        if [ -n "$init_script" ]; then
            echo ""
            echo -e "\033[0;36mPhase directory detected. Running session-init...\033[0m"
            echo ""
            "$init_script" "$(pwd)"
        else
            echo ""
            echo -e "\033[1;33mPhase directory detected. Consider running session-init.sh\033[0m"
            echo ""
        fi
    fi
}

# ============================================================================
# PERIODIC AUTO-SAVE (Optional)
# ============================================================================

# Uncomment the following to enable periodic auto-save every 10 minutes
# This runs in the background and saves state periodically

# _agentic_periodic_save() {
#     while true; do
#         sleep 600  # 10 minutes
#         if [ -d ".phase-status" ]; then
#             local save_script=""
#             for loc in "./scripts/session-save.sh" "../scripts/session-save.sh" "$AGENTIC_SCRIPTS_DIR/session-save.sh"; do
#                 if [ -x "$loc" ]; then
#                     save_script="$loc"
#                     break
#                 fi
#             done
#             if [ -n "$save_script" ]; then
#                 "$save_script" --quiet "$(pwd)" 2>/dev/null
#             fi
#         fi
#     done
# }
#
# # Start periodic save in background
# _agentic_periodic_save &
# AGENTIC_PERIODIC_PID=$!
#
# # Clean up on exit
# trap "kill $AGENTIC_PERIODIC_PID 2>/dev/null" EXIT

# ============================================================================
# HELPER FUNCTIONS
# ============================================================================

# Quick alias for session management
alias session-init='$AGENTIC_SCRIPTS_DIR/session-init.sh'
alias session-save='$AGENTIC_SCRIPTS_DIR/session-save.sh'
alias session-save-auto='$AGENTIC_SCRIPTS_DIR/session-save.sh --auto'

# Show session status
session-status() {
    if [ -d ".phase-status" ]; then
        echo "Phase: $(basename $(pwd))"
        echo "Branch: $(git branch --show-current 2>/dev/null || echo 'unknown')"

        if [ -f ".phase-status/SESSION-STATE.md" ]; then
            echo ""
            echo "Last session state:"
            head -15 ".phase-status/SESSION-STATE.md" | tail -10
        fi

        if [ -f ".phase-status/BLOCKERS.md" ]; then
            echo ""
            echo -e "\033[0;31mBLOCKERS.md exists - review blockers\033[0m"
        fi
    else
        echo "Not in a phase directory"
    fi
}

# ============================================================================
# COMPLETION MESSAGE
# ============================================================================

echo -e "\033[0;32m✓\033[0m Agentic session hooks loaded"
echo -e "  \033[2m• Exit trap: Auto-save on shell exit\033[0m"
echo -e "  \033[2m• CD hook: Auto-init on entering phase directory\033[0m"
echo -e "  \033[2m• Aliases: session-init, session-save, session-status\033[0m"
echo ""
