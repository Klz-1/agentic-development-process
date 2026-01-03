#!/bin/bash
#
# session-save.sh
# Save current session state for seamless handoff to next session
#
# Usage: ./scripts/session-save.sh [phase-dir]
#
# Run this before ending a session to preserve context for the next session.
# The script creates/updates SESSION-STATE.md with current work state.
#

set -e

# Configuration
PHASE_DIR="${1:-$(pwd)}"
STATE_FILE="$PHASE_DIR/.phase-status/SESSION-STATE.md"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'
BOLD='\033[1m'

# Ensure .phase-status directory exists
mkdir -p "$PHASE_DIR/.phase-status"

echo ""
echo -e "${BOLD}${CYAN}══════════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}${CYAN}                    SESSION STATE SAVE                         ${NC}"
echo -e "${BOLD}${CYAN}══════════════════════════════════════════════════════════════${NC}"
echo ""

# Gather information
PHASE_NAME=$(basename "$PHASE_DIR")
BRANCH=$(git -C "$PHASE_DIR" branch --show-current 2>/dev/null || echo "unknown")
LAST_COMMIT=$(git -C "$PHASE_DIR" log --oneline -1 2>/dev/null || echo "No commits")
UNCOMMITTED=$(git -C "$PHASE_DIR" status --porcelain 2>/dev/null | wc -l | tr -d ' ')
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

# Get current task from PROGRESS.md
CURRENT_TASK=""
PROGRESS_FILE="$PHASE_DIR/.phase-status/PROGRESS.md"
if [ -f "$PROGRESS_FILE" ]; then
    CURRENT_TASK=$(grep -A1 "## Current Task" "$PROGRESS_FILE" 2>/dev/null | tail -1 | sed 's/^[[:space:]]*//')
    if [ -z "$CURRENT_TASK" ] || [ "$CURRENT_TASK" = "## Current Task" ]; then
        CURRENT_TASK=$(grep '^\- \[ \]' "$PROGRESS_FILE" 2>/dev/null | head -1 | sed 's/^- \[ \] //')
    fi
fi

# Get progress stats
if [ -f "$PROGRESS_FILE" ]; then
    COMPLETED=$(grep -c '\[x\]' "$PROGRESS_FILE" 2>/dev/null || echo 0)
    TOTAL=$(grep -c '\[.\]' "$PROGRESS_FILE" 2>/dev/null || echo 0)
    PROGRESS="$COMPLETED/$TOTAL"
else
    PROGRESS="unknown"
fi

# Interactive prompts for context
echo -e "${YELLOW}Please provide session context (press Enter to skip):${NC}"
echo ""

read -p "What were you working on? " WORKING_ON
read -p "What's the current status? " CURRENT_STATUS
read -p "What should the next session do first? " NEXT_STEPS
read -p "Any blockers or concerns? " BLOCKERS
read -p "Any decisions made this session? " DECISIONS

echo ""

# Get modified files summary
MODIFIED_FILES=$(git -C "$PHASE_DIR" status --porcelain 2>/dev/null | head -10)

# Get recent commits this session (last 5)
RECENT_COMMITS=$(git -C "$PHASE_DIR" log --oneline -5 2>/dev/null)

# Generate the state file
cat > "$STATE_FILE" << EOF
# Session State - $PHASE_NAME

**Saved:** $TIMESTAMP
**Branch:** $BRANCH
**Progress:** $PROGRESS tasks

---

## Last Session Summary

**Working On:** ${WORKING_ON:-"Not specified"}

**Status:** ${CURRENT_STATUS:-"Not specified"}

**Current Task:** ${CURRENT_TASK:-"Not specified"}

---

## For Next Session

### Priority Actions
${NEXT_STEPS:-"1. Review this state file
2. Check MASTER-NOTES.md for updates
3. Continue with current task"}

### Blockers/Concerns
${BLOCKERS:-"None reported"}

### Decisions Made
${DECISIONS:-"None reported"}

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

### Uncommitted Changes
**Count:** $UNCOMMITTED files

\`\`\`
$MODIFIED_FILES
\`\`\`

---

## Quick Resume Commands

\`\`\`bash
# Navigate to phase
cd $PHASE_DIR

# Check status
git status

# Run session init
./scripts/session-init.sh

# Continue working...
\`\`\`

---

## Files to Review

- \`.phase-status/PROGRESS.md\` - Task tracking
- \`.phase-status/MASTER-NOTES.md\` - Master guidance
- \`.phase-status/BLOCKERS.md\` - If exists, blockers to resolve

---

*Session state saved automatically. Valid for 24 hours.*
EOF

echo -e "${GREEN}════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${GREEN}${BOLD}Session state saved successfully!${NC}"
echo ""
echo -e "  ${CYAN}File:${NC} $STATE_FILE"
echo -e "  ${CYAN}Time:${NC} $TIMESTAMP"
echo ""
echo -e "${DIM}Next session can run:${NC}"
echo -e "${DIM}  ./scripts/session-init.sh${NC}"
echo -e "${DIM}to recover this context.${NC}"
echo ""

# Also update PROGRESS.md with session end marker if it exists
if [ -f "$PROGRESS_FILE" ]; then
    # Check if there's already a session log section
    if ! grep -q "## Session Log" "$PROGRESS_FILE"; then
        echo "" >> "$PROGRESS_FILE"
        echo "---" >> "$PROGRESS_FILE"
        echo "" >> "$PROGRESS_FILE"
        echo "## Session Log" >> "$PROGRESS_FILE"
    fi

    echo "" >> "$PROGRESS_FILE"
    echo "### Session End: $TIMESTAMP" >> "$PROGRESS_FILE"
    echo "- Status: ${CURRENT_STATUS:-"Session ended"}" >> "$PROGRESS_FILE"
    echo "- Next: ${NEXT_STEPS:-"Continue with current task"}" >> "$PROGRESS_FILE"

    echo -e "${GREEN}✓${NC} PROGRESS.md updated with session log"
fi

echo ""
