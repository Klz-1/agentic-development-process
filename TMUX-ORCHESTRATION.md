# Tmux Orchestration for Autonomous Subagent Monitoring

**Purpose:** Add a layer of autonomy to the Master Orchestrator by using tmux for real-time, persistent monitoring of subagent work across multiple phases.

**Version:** 1.0

---

## Table of Contents

1. [Why Tmux for Agentic Development](#why-tmux-for-agentic-development)
2. [Architecture Overview](#architecture-overview)
3. [Session Structure](#session-structure)
4. [Setup Guide](#setup-guide)
5. [Monitoring Strategies](#monitoring-strategies)
6. [Automated Alerts](#automated-alerts)
7. [Commands Reference](#commands-reference)
8. [Integration with File-Based Communication](#integration-with-file-based-communication)
9. [Advanced Patterns](#advanced-patterns)
10. [Troubleshooting](#troubleshooting)

---

## Why Tmux for Agentic Development

### **The Problem with Manual Polling**

The standard agentic development process requires the Master Orchestrator to:
- Check phase worktrees every 10-30 minutes
- Manually read PROGRESS.md, BLOCKERS.md files
- Switch between terminals for different phases
- Lose visibility when disconnected

**This is reactive, not proactive.**

### **What Tmux Enables**

| Manual Polling | Tmux Orchestration |
|----------------|---------------------|
| Check every 10-30 min | Real-time streaming |
| Active file reading | Passive monitoring |
| One terminal at a time | All phases visible |
| Lost on disconnect | Persistent sessions |
| No alerts | Automated notifications |
| Context switching overhead | Unified dashboard |

### **Core Benefits**

1. **Persistent Sessions**
   - Work continues even if SSH/terminal disconnects
   - Reattach and continue exactly where you left off
   - No lost context or interrupted monitoring

2. **Real-Time Visibility**
   - See subagent output as it happens
   - Watch test runs, build outputs, git activity
   - Immediate awareness of blockers

3. **Parallel Monitoring**
   - View multiple phases simultaneously
   - Split panes for dashboard + details
   - No context switching between terminals

4. **Automated Alerts**
   - Get notified when BLOCKERS.md created
   - Alert on COMPLETED.md (phase done)
   - Watch for test failures in real-time

5. **Programmatic Control**
   - Send commands to subagent panes
   - Capture output for analysis
   - Script complex workflows

---

## Architecture Overview

### **Master Orchestrator Session Layout**

```
┌─────────────────────────────────────────────────────────────────────┐
│                     TMUX SESSION: agentic-dev                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────── Window 1: Dashboard ────────────────┐            │
│  │ ┌──────────────────┐ ┌────────────────────────────┐ │            │
│  │ │  Master Status   │ │     Phase Overview         │ │            │
│  │ │  PROGRESS.md     │ │  watch phase-status.sh     │ │            │
│  │ │  (auto-refresh)  │ │  Phase 1: ✅ Complete      │ │            │
│  │ │                  │ │  Phase 2: 🔄 In Progress   │ │            │
│  │ │                  │ │  Phase 3: ⏳ Pending       │ │            │
│  │ └──────────────────┘ └────────────────────────────┘ │            │
│  │ ┌─────────────────────────────────────────────────┐ │            │
│  │ │  Alert Monitor - inotifywait watching for:     │ │            │
│  │ │  BLOCKERS.md, COMPLETED.md, QUESTIONS.md       │ │            │
│  │ └─────────────────────────────────────────────────┘ │            │
│  └────────────────────────────────────────────────────┘            │
│                                                                     │
│  ┌─────────────── Window 2: Phase 1 ──────────────────┐            │
│  │ ┌────────────────────────────────────────────────┐ │            │
│  │ │  Subagent Terminal (Claude instance)           │ │            │
│  │ │  Working on Phase 1 tasks...                   │ │            │
│  │ └────────────────────────────────────────────────┘ │            │
│  │ ┌──────────────────┐ ┌────────────────────────────┐ │            │
│  │ │  Progress Watch  │ │  Git Log Monitor          │ │            │
│  │ │  PROGRESS.md     │ │  git log --oneline -5     │ │            │
│  │ └──────────────────┘ └────────────────────────────┘ │            │
│  └────────────────────────────────────────────────────┘            │
│                                                                     │
│  ┌─────────────── Window 3: Phase 2 ──────────────────┐            │
│  │  [Same structure as Phase 1]                       │            │
│  └────────────────────────────────────────────────────┘            │
│                                                                     │
│  ┌─────────────── Window N: Phase N ──────────────────┐            │
│  │  [Same structure for each active phase]            │            │
│  └────────────────────────────────────────────────────┘            │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### **Data Flow**

```
Subagent works in tmux pane
         │
         ├──→ Output visible in real-time
         │
         ├──→ Updates .phase-status/PROGRESS.md
         │         │
         │         └──→ inotifywait detects change
         │                    │
         │                    └──→ Alert in Dashboard
         │
         ├──→ Creates BLOCKERS.md (if stuck)
         │         │
         │         └──→ 🚨 ALERT: Master notified immediately
         │
         └──→ Creates COMPLETED.md (when done)
                   │
                   └──→ 🎉 ALERT: Phase complete notification
```

---

## Session Structure

### **Recommended Window Layout**

| Window | Name | Purpose | Panes |
|--------|------|---------|-------|
| 0 | `dashboard` | Master orchestrator overview | 3 |
| 1 | `phase-1` | Phase 1 subagent | 3 |
| 2 | `phase-2` | Phase 2 subagent | 3 |
| N | `phase-N` | Phase N subagent | 3 |
| N+1 | `git-ops` | Git operations & merging | 2 |

### **Dashboard Window (Window 0)**

```
┌─────────────────────────────────────────────────────┐
│ Pane 0: Master PROGRESS.md                          │
│ watch -n 10 cat docs/PROGRESS.md                    │
├────────────────────────┬────────────────────────────┤
│ Pane 1: Phase Status   │ Pane 2: Alert Monitor      │
│ ./scripts/phase-       │ ./scripts/alert-on-        │
│   status-overview.sh   │   change.sh                │
└────────────────────────┴────────────────────────────┘
```

### **Phase Window (Window 1-N)**

```
┌─────────────────────────────────────────────────────┐
│ Pane 0: Subagent Terminal (Main Work Area)          │
│ Claude instance working on phase tasks              │
├────────────────────────┬────────────────────────────┤
│ Pane 1: Progress       │ Pane 2: Git Activity       │
│ watch -n 5 cat         │ watch -n 10 git log        │
│ .phase-status/         │ --oneline -8               │
│ PROGRESS.md            │                            │
└────────────────────────┴────────────────────────────┘
```

---

## Setup Guide

### **Prerequisites**

```bash
# Install tmux (if not present)
# Ubuntu/Debian
sudo apt-get install tmux inotify-tools

# macOS
brew install tmux fswatch

# Verify installation
tmux -V
```

### **Quick Start**

```bash
# Navigate to your project using this framework
cd /path/to/your-project

# Run the setup script
./scripts/setup-tmux-session.sh

# Attach to the session
tmux attach -t agentic-dev
```

### **Manual Setup**

```bash
# 1. Create the session
tmux new-session -d -s agentic-dev -n dashboard

# 2. Configure dashboard window
tmux send-keys -t agentic-dev:dashboard 'watch -n 10 cat docs/PROGRESS.md' Enter

# Split for phase overview
tmux split-window -t agentic-dev:dashboard -h
tmux send-keys -t agentic-dev:dashboard.1 './scripts/phase-status-overview.sh' Enter

# Split for alerts
tmux split-window -t agentic-dev:dashboard.1 -v
tmux send-keys -t agentic-dev:dashboard.2 './scripts/alert-on-change.sh' Enter

# 3. Create phase windows (repeat for each phase)
tmux new-window -t agentic-dev -n phase-1
tmux send-keys -t agentic-dev:phase-1 'cd /path/to/project-phase-1' Enter

# Split for progress monitoring
tmux split-window -t agentic-dev:phase-1 -v -p 30
tmux send-keys -t agentic-dev:phase-1.1 'watch -n 5 cat .phase-status/PROGRESS.md' Enter

# Split for git monitoring
tmux split-window -t agentic-dev:phase-1.1 -h
tmux send-keys -t agentic-dev:phase-1.2 'watch -n 10 git log --oneline -8' Enter

# 4. Attach to session
tmux attach -t agentic-dev
```

---

## Monitoring Strategies

### **Strategy 1: Passive Dashboard Monitoring**

The dashboard window provides constant visibility without active checking.

**What to watch:**
- `docs/PROGRESS.md` - Overall project status
- Phase status pane - Quick health check of all phases
- Alert pane - Immediate notification of important events

**When to check other windows:**
- Alert appears in dashboard
- Need to provide guidance to subagent
- Ready to review completed phase

### **Strategy 2: Active Phase Observation**

Switch to a phase window to observe subagent work in real-time.

```bash
# Switch to Phase 2 window
tmux select-window -t agentic-dev:phase-2

# Watch the subagent work in Pane 0
# Monitor progress in Pane 1
# Check git activity in Pane 2
```

**When to use:**
- Critical phase in progress
- Subagent reported blocker
- Approaching phase completion

### **Strategy 3: File Change Watching**

Use inotifywait (Linux) or fswatch (macOS) for instant alerts.

```bash
# Linux - Watch for key file changes
inotifywait -m -r -e create,modify,delete \
  --include '(BLOCKERS|COMPLETED|QUESTIONS)\.md$' \
  /path/to/project-phase-*/.phase-status/

# macOS - Same with fswatch
fswatch -r --include='(BLOCKERS|COMPLETED|QUESTIONS)\.md$' \
  /path/to/project-phase-*/.phase-status/
```

### **Strategy 4: Test Output Streaming**

Watch test runs in real-time:

```bash
# In phase window, run tests with output
npm test 2>&1 | tee test-output.log

# Or watch the log file
tail -f test-output.log
```

---

## Automated Alerts

### **Alert Types**

| Event | File | Urgency | Action |
|-------|------|---------|--------|
| Blocker | `BLOCKERS.md` | 🔴 High | Check immediately |
| Question | `QUESTIONS.md` | 🟡 Medium | Respond when able |
| Complete | `COMPLETED.md` | 🟢 Info | Queue for review |
| Progress | `PROGRESS.md` | ⚪ Low | Passive awareness |

### **Alert Script Example**

```bash
#!/bin/bash
# alert-on-change.sh

WATCH_DIRS="/path/to/project-phase-*/.phase-status"

echo "🔔 Alert Monitor Started"
echo "Watching: $WATCH_DIRS"
echo "──────────────────────────────"

inotifywait -m -r -e create,modify $WATCH_DIRS 2>/dev/null | while read path action file; do
  timestamp=$(date '+%H:%M:%S')

  case "$file" in
    BLOCKERS.md)
      echo "🚨 [$timestamp] BLOCKER DETECTED!"
      echo "   Path: $path$file"
      echo "   Action required: Check subagent immediately"
      # Optional: Send desktop notification
      notify-send "🚨 Subagent Blocker" "Check $path$file"
      ;;
    COMPLETED.md)
      echo "🎉 [$timestamp] PHASE COMPLETE!"
      echo "   Path: $path$file"
      echo "   Action: Queue for code review"
      notify-send "🎉 Phase Complete" "Ready for review: $path"
      ;;
    QUESTIONS.md)
      echo "❓ [$timestamp] Question from subagent"
      echo "   Path: $path$file"
      echo "   Action: Respond in MASTER-NOTES.md"
      ;;
    PROGRESS.md)
      echo "📝 [$timestamp] Progress update: $path"
      ;;
  esac
done
```

### **Desktop Notifications**

```bash
# Linux (requires libnotify)
notify-send "Title" "Message"

# macOS
osascript -e 'display notification "Message" with title "Title"'

# Cross-platform with terminal-notifier (macOS) or notify-send (Linux)
```

---

## Commands Reference

### **Session Management**

```bash
# Create new session
tmux new-session -d -s agentic-dev

# Attach to session
tmux attach -t agentic-dev

# Detach from session (inside tmux)
Ctrl-b d

# List sessions
tmux list-sessions

# Kill session
tmux kill-session -t agentic-dev
```

### **Window Management**

```bash
# Create window
tmux new-window -t agentic-dev -n phase-3

# Switch windows
Ctrl-b 0  # Dashboard
Ctrl-b 1  # Phase 1
Ctrl-b n  # Next window
Ctrl-b p  # Previous window

# Rename window
Ctrl-b ,

# Close window
Ctrl-b &
```

### **Pane Management**

```bash
# Split horizontally
Ctrl-b "

# Split vertically
Ctrl-b %

# Navigate panes
Ctrl-b [arrow key]

# Resize pane
Ctrl-b Ctrl-[arrow key]

# Close pane
Ctrl-b x

# Zoom pane (toggle fullscreen)
Ctrl-b z
```

### **Useful Shortcuts**

```bash
# Copy mode (scroll through output)
Ctrl-b [
# Use arrow keys to scroll, q to exit

# Search in copy mode
Ctrl-b [ then Ctrl-s (forward) or Ctrl-r (reverse)

# Send command to pane programmatically
tmux send-keys -t agentic-dev:phase-1.0 'npm test' Enter

# Capture pane output
tmux capture-pane -t agentic-dev:phase-1.0 -p > output.txt
```

---

## Integration with File-Based Communication

### **Enhanced Communication Flow with Tmux**

```
                    ┌─────────────────────────┐
                    │    TMUX DASHBOARD       │
                    │  (Real-time visibility) │
                    └────────────┬────────────┘
                                 │
                                 │ Watches
                                 ▼
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   ┌──────────────┐     ┌──────────────┐     ┌────────────┐ │
│   │   Phase 1    │     │   Phase 2    │     │  Phase N   │ │
│   │   Worktree   │     │   Worktree   │     │  Worktree  │ │
│   │              │     │              │     │            │ │
│   │ .phase-status│     │ .phase-status│     │.phase-stat │ │
│   │ ├─PROGRESS   │     │ ├─PROGRESS   │     │├─PROGRESS  │ │
│   │ ├─BLOCKERS   │◄────┤ ├─BLOCKERS   │◄────┤├─BLOCKERS  │ │
│   │ └─COMPLETED  │     │ └─COMPLETED  │     │└─COMPLETED │ │
│   └──────────────┘     └──────────────┘     └────────────┘ │
│                                                             │
└─────────────────────────────────────────────────────────────┘
                                 │
                                 │ inotifywait/fswatch
                                 ▼
                    ┌─────────────────────────┐
                    │    ALERT SYSTEM         │
                    │  • Desktop notification │
                    │  • Sound alert          │
                    │  • Log entry            │
                    └─────────────────────────┘
```

### **Master Orchestrator Workflow with Tmux**

**Before (Manual Polling):**
```
Every 10-30 min:
  1. cd to phase worktree
  2. cat .phase-status/PROGRESS.md
  3. Check for BLOCKERS.md
  4. Update MASTER-NOTES.md if needed
  5. Repeat for each phase
  6. Update docs/PROGRESS.md
```

**After (Tmux Orchestration):**
```
Continuous:
  1. Dashboard shows all phases at a glance
  2. Alerts notify when attention needed
  3. Switch to phase window only when necessary
  4. Real-time visibility into subagent work

On alert:
  1. Receive notification (visual/audio)
  2. Switch to relevant phase window (Ctrl-b N)
  3. Read context in progress pane
  4. Interact with subagent or update MASTER-NOTES.md
  5. Return to dashboard
```

---

## Advanced Patterns

### **Pattern 1: Subagent Spawning from Tmux**

Spawn new Claude instances directly in phase windows:

```bash
# Create new phase worktree
git worktree add ../project-phase-3 -b feature/phase-3-name
mkdir -p ../project-phase-3/.phase-status

# Create new tmux window
tmux new-window -t agentic-dev -n phase-3
tmux send-keys -t agentic-dev:phase-3 'cd /path/to/project-phase-3' Enter

# Split and set up monitoring
tmux split-window -t agentic-dev:phase-3 -v -p 30
tmux send-keys -t agentic-dev:phase-3.1 'watch -n 5 cat .phase-status/PROGRESS.md' Enter

tmux split-window -t agentic-dev:phase-3.1 -h
tmux send-keys -t agentic-dev:phase-3.2 'watch -n 10 git log --oneline -8' Enter

# Return to main pane and start Claude
tmux select-pane -t agentic-dev:phase-3.0
tmux send-keys -t agentic-dev:phase-3.0 'claude' Enter
```

### **Pattern 2: Synchronized Builds**

Run builds across all phases and monitor:

```bash
#!/bin/bash
# sync-build-all.sh

for phase in /path/to/project-phase-*; do
  phase_name=$(basename $phase)
  tmux send-keys -t agentic-dev:$phase_name.0 'npm run build 2>&1 | tee build.log' Enter
done

echo "Builds started in all phase windows"
echo "Watch for results in each window"
```

### **Pattern 3: Test Watch Mode**

Enable test watch mode with output streaming:

```bash
# In phase window
npm test -- --watch 2>&1 | tee -a test-watch.log

# Monitor from another pane or dashboard
tail -f /path/to/project-phase-X/test-watch.log
```

### **Pattern 4: Git Activity Aggregation**

Watch git activity across all worktrees:

```bash
#!/bin/bash
# git-activity-monitor.sh

while true; do
  clear
  echo "Git Activity Across All Phases"
  echo "=============================="
  echo ""

  for phase in /path/to/project-phase-*; do
    phase_name=$(basename $phase)
    echo "📁 $phase_name:"
    cd $phase
    git log --oneline -3 --format="   %h %s" 2>/dev/null
    echo ""
  done

  sleep 30
done
```

### **Pattern 5: Metric Collection**

Collect progress metrics automatically:

```bash
#!/bin/bash
# collect-metrics.sh

METRICS_FILE="/path/to/project/docs/METRICS.log"

while true; do
  timestamp=$(date '+%Y-%m-%d %H:%M:%S')

  for phase in /path/to/project-phase-*; do
    phase_name=$(basename $phase)
    progress_file="$phase/.phase-status/PROGRESS.md"

    if [ -f "$progress_file" ]; then
      # Count completed tasks (lines with [x])
      completed=$(grep -c '\[x\]' "$progress_file" 2>/dev/null || echo 0)
      # Count total tasks (lines with [ ] or [x])
      total=$(grep -c '\[.\]' "$progress_file" 2>/dev/null || echo 0)

      echo "$timestamp,$phase_name,$completed,$total" >> $METRICS_FILE
    fi
  done

  sleep 300  # Every 5 minutes
done
```

### **Pattern 6: Session Persistence Script**

Recreate session from saved state:

```bash
#!/bin/bash
# restore-session.sh

# Read saved state
source /path/to/project/.coordination/tmux-state.sh

# Create session with saved windows
tmux new-session -d -s agentic-dev -n dashboard

for phase in "${ACTIVE_PHASES[@]}"; do
  tmux new-window -t agentic-dev -n $phase
  tmux send-keys -t agentic-dev:$phase "cd $PROJECT_ROOT-$phase" Enter
  # Set up monitoring panes...
done

echo "Session restored with ${#ACTIVE_PHASES[@]} phase windows"
```

---

## Troubleshooting

### **Common Issues**

**Issue: Tmux session not persisting after system restart**
- **Cause:** Tmux sessions don't survive reboots
- **Solution:** Use tmux-resurrect plugin or recreate with setup script

```bash
# Install tmux-resurrect
git clone https://github.com/tmux-plugins/tmux-resurrect ~/.tmux/plugins/tmux-resurrect

# Add to ~/.tmux.conf
run-shell ~/.tmux/plugins/tmux-resurrect/resurrect.tmux

# Save session: Ctrl-b Ctrl-s
# Restore session: Ctrl-b Ctrl-r
```

**Issue: Watch command not updating**
- **Cause:** File permissions or path issues
- **Solution:** Check file exists and is readable

```bash
# Verify file exists
ls -la /path/to/.phase-status/PROGRESS.md

# Try manual cat
cat /path/to/.phase-status/PROGRESS.md
```

**Issue: inotifywait not available**
- **Cause:** inotify-tools not installed
- **Solution:** Install the package

```bash
# Ubuntu/Debian
sudo apt-get install inotify-tools

# macOS (use fswatch instead)
brew install fswatch
```

**Issue: Notifications not appearing**
- **Cause:** Desktop notification system not configured
- **Solution:** Install and configure notification tools

```bash
# Linux
sudo apt-get install libnotify-bin

# Test notification
notify-send "Test" "This is a test notification"
```

**Issue: Panes too small to see content**
- **Cause:** Terminal window size or pane layout
- **Solution:** Adjust pane sizes or zoom

```bash
# Zoom current pane (toggle)
Ctrl-b z

# Resize pane
Ctrl-b Ctrl-[arrow key]

# Or specify percentage when splitting
tmux split-window -v -p 70  # 70% for new pane
```

### **Performance Tips**

1. **Reduce watch frequency** for less critical monitors
   ```bash
   watch -n 30 ...  # Every 30 seconds instead of 5
   ```

2. **Use inotify instead of polling** when possible
   ```bash
   # Polling (uses CPU)
   watch -n 5 cat file.md

   # Event-based (efficient)
   while inotifywait -e modify file.md; do cat file.md; done
   ```

3. **Limit git log depth**
   ```bash
   git log --oneline -5  # Not -50
   ```

4. **Close unused windows** to reduce memory usage

---

## Best Practices

### **Session Hygiene**

1. **Name sessions descriptively**
   ```bash
   tmux new-session -s project-name-agentic
   ```

2. **Use consistent window naming**
   ```
   dashboard, phase-1, phase-2, git-ops
   ```

3. **Document your layout**
   - Save tmux config
   - Create setup script
   - Include in project repo

### **Monitoring Discipline**

1. **Keep dashboard visible** - Don't get lost in phase details
2. **Respond to alerts promptly** - Especially BLOCKERS.md
3. **Update MASTER-NOTES.md** directly from phase window
4. **Commit regularly** - Don't let work pile up

### **Integration with Agentic Process**

1. **Dashboard = docs/PROGRESS.md** - Keep these in sync
2. **Use tmux for visibility**, files for communication
3. **Don't skip file updates** - Tmux doesn't replace file-based protocol
4. **Detach when done** - Session persists for next session

---

## Quick Reference Card

```
┌─────────────────────────────────────────────────────────────┐
│                TMUX ORCHESTRATION CHEAT SHEET               │
├─────────────────────────────────────────────────────────────┤
│ SESSION                                                     │
│   tmux new -s agentic-dev     Create session                │
│   tmux attach -t agentic-dev  Attach to session             │
│   Ctrl-b d                    Detach from session           │
├─────────────────────────────────────────────────────────────┤
│ WINDOWS                                                     │
│   Ctrl-b c                    Create window                 │
│   Ctrl-b 0-9                  Switch to window N            │
│   Ctrl-b n/p                  Next/previous window          │
│   Ctrl-b ,                    Rename window                 │
├─────────────────────────────────────────────────────────────┤
│ PANES                                                       │
│   Ctrl-b "                    Split horizontal              │
│   Ctrl-b %                    Split vertical                │
│   Ctrl-b [arrow]              Navigate panes                │
│   Ctrl-b z                    Zoom pane (toggle)            │
│   Ctrl-b x                    Close pane                    │
├─────────────────────────────────────────────────────────────┤
│ MONITORING                                                  │
│   watch -n 5 cat file         Auto-refresh file view        │
│   inotifywait -m dir          Watch for file changes        │
│   tail -f logfile             Stream log output             │
├─────────────────────────────────────────────────────────────┤
│ ALERTS                                                      │
│   🚨 BLOCKERS.md              Check immediately             │
│   ❓ QUESTIONS.md             Respond when able             │
│   🎉 COMPLETED.md             Queue for review              │
└─────────────────────────────────────────────────────────────┘
```

---

**Version:** 1.0
**Created:** January 2025
**License:** MIT

**This enhancement brings autonomous monitoring to the Agentic Development Process, transforming passive polling into active, real-time orchestration.**
