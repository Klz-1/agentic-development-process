# Templates Guide

This directory contains templates for the Agentic Development Process. Templates fall into two categories:

## Templates to COPY (Static Reference Docs)

Copy these to your project during setup - they serve as permanent reference documentation:

| Template | Copy To | Purpose |
|----------|---------|---------|
| `SUBAGENT-GUIDELINES.md` | `.coordination/` | Rules and constraints for subagents |
| `COMMIT-WORKFLOW.md` | `.coordination/` | Testing workflow before commits |
| `PROGRESS-TEMPLATE.md` | `docs/PROGRESS.md` | Initialize master dashboard |

## Templates for REFERENCE (Create Files From)

Use these as guides when creating specific files - don't copy them directly:

| Template | Creates | Who Creates | Where |
|----------|---------|-------------|-------|
| `MASTER-NOTES-TEMPLATE.md` | `MASTER-NOTES.md` | Master | `.worktrees/phase-X/.phase-status/` |
| `BLOCKERS-TEMPLATE.md` | `BLOCKERS.md` | Subagent | `.worktrees/phase-X/.phase-status/` |
| `QUESTIONS-TEMPLATE.md` | `QUESTIONS.md` | Subagent | `.worktrees/phase-X/.phase-status/` |
| `COMPLETED-TEMPLATE.md` | `COMPLETED.md` | Subagent | `.worktrees/phase-X/.phase-status/` |
| `SESSION-STATE-TEMPLATE.md` | `SESSION-STATE.md` | Subagent (auto) | `.worktrees/phase-X/.phase-status/` |
| `SESSION-SUMMARY-TEMPLATE.md` | `SESSION-SUMMARY.md` | Master | `.coordination/` |
| `SENIOR-ENGINEER-REVIEW-TEMPLATE.md` | `PHASE-X-SENIOR-ENGINEER-REVIEW.md` | Master | `.coordination/` |

## Session Templates Clarification

**SESSION-STATE** (Per-Phase):
- Created by subagents (or auto-saved by Claude hooks)
- Lives in `.worktrees/phase-X/.phase-status/SESSION-STATE.md`
- Tracks a single subagent's work state
- Used for handoff between subagent sessions

**SESSION-SUMMARY** (Project-Wide):
- Created by Master Orchestrator
- Lives in `.coordination/SESSION-SUMMARY.md`
- Tracks ALL phases and overall project state
- Used when pausing the entire project

## Configuration Files

| File | Purpose |
|------|---------|
| `claude-settings.json` | Claude Code hooks config (auto-detecting) |
| `claude-settings-master.json` | Claude hooks for Master only |
| `claude-settings-subagent.json` | Claude hooks for Subagent only |
| `tmux-orchestrator.conf` | Tmux configuration for monitoring |

## Review Template

**SENIOR-ENGINEER-REVIEW-TEMPLATE.md** is used when:
- Master reviews completed phase
- Creating PR review documentation
- Output: `.coordination/PHASE-X-SENIOR-ENGINEER-REVIEW.md`
