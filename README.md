# Agentic Development Process

A battle-tested framework for managing multi-phase software development with AI coding agents.

## Why This Exists

When building complex software with AI agents, the biggest challenge isn't the AI's capability—it's **context continuity**. Agents lose memory between sessions, leading to:

- Repeated work and wasted context
- Inconsistent quality across phases
- Broken integrations between components
- Lost knowledge when resuming projects

This guide solves that with a comprehensive orchestration pattern.

## Industry Validation

This process was developed independently in **October 2025** through real-world production development. In **November 2025**, Anthropic published ["Effective Harnesses for Long-Running Agents"](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents) describing remarkably similar patterns:

| Pattern | This Guide | Anthropic Blog |
|---------|-----------|----------------|
| File-based state persistence | `docs/PROGRESS.md` + `.phase-status/` | `claude-progress.txt` |
| Git as source of truth | Detailed merge workflow | Git logs + commits |
| Incremental progress | 5-10 tasks per phase | One feature per session |
| Session initialization | 5-minute context recovery | Structured startup protocol |
| Testing before completion | 10-step mandatory workflow | End-to-end verification |

**This convergence validates these patterns as industry best practices.** This guide provides the comprehensive operational playbook that complements Anthropic's conceptual framework.

## What's Different Here

While Anthropic's blog covers the "what," this guide covers the "how" in exhaustive detail:

- **Master Orchestrator Pattern** - Coordinate multiple agents working in parallel (not just sequential handoffs)
- **Git Worktrees** - True parallel development across phases
- **Quality Gates** - 10 mandatory checkpoints before any merge
- **Code Review Process** - Scoring system and review templates
- **Agent Guardrails** - Explicit prohibitions to prevent context-pressure shortcuts
- **File-Based Communication** - Bidirectional protocol between master and subagents
- **Tmux Orchestration** - Real-time, autonomous monitoring of subagent work with automated alerts

## How to Use This Document

The main document (`AGENTIC-DEVELOPMENT-PROCESS.md`) is designed to be used as a **system prompt** for an AI coding agent (like Claude Code) to initialize it as the Master Orchestrator.

**Typical Workflow:**

1. **Initialize Master Agent:** Feed `AGENTIC-DEVELOPMENT-PROCESS.md` + your `prd.md` (Product Requirements Document) to Claude Code
2. **Master Plans Phases:** The AI agent reads your PRD and breaks it into phases using this process
3. **Master Creates Subagent Prompts:** For each phase, the Master prepares prompts for subagents
4. **Subagents Execute:** Each subagent works in its own git worktree following the guidelines
5. **Master Reviews & Merges:** Master coordinates, reviews, and merges completed phases

> **Note:** When you see "(You)" in `AGENTIC-DEVELOPMENT-PROCESS.md`, it addresses the AI agent that will become the Master Orchestrator after reading the prompt.

## Quick Start

### 1. Set Up Your Project

```bash
# Clone or copy the agentic-development-process repo
# Replace <org> with actual GitHub org/user
git clone https://github.com/<org>/agentic-development-process.git /tmp/adp

# Create coordination structure
mkdir -p .coordination/status-reports .coordination/archives docs

# Copy templates TO your project (these are static reference docs)
cp /tmp/adp/templates/SUBAGENT-GUIDELINES.md .coordination/
cp /tmp/adp/templates/COMMIT-WORKFLOW.md .coordination/

# Initialize master dashboard
cp /tmp/adp/templates/PROGRESS-TEMPLATE.md docs/PROGRESS.md

# Create worktrees directory (add to .gitignore)
mkdir -p .worktrees
echo ".worktrees/" >> .gitignore

# Install pre-commit hooks (quality gates)
mkdir -p .git/hooks
cp /tmp/adp/githooks/pre-commit .git/hooks/
cp /tmp/adp/githooks/commit-msg .git/hooks/
chmod +x .git/hooks/*

# Install Claude Code hooks (session management)
mkdir -p .claude/hooks
cp /tmp/adp/hooks/session-start.sh .claude/hooks/
cp /tmp/adp/hooks/session-end.sh .claude/hooks/
chmod +x .claude/hooks/*
cp /tmp/adp/templates/claude-settings.json .claude/settings.json

# Clean up
rm -rf /tmp/adp
```

**Note:** Templates like `BLOCKERS-TEMPLATE.md`, `COMPLETED-TEMPLATE.md`, `QUESTIONS-TEMPLATE.md` are **reference templates** - subagents use them as guides when creating those files, they're not copied to your project.

### 2. Create Phase Worktrees

```bash
# Create a worktree for Phase 1 INSIDE the repo
git worktree add .worktrees/phase-1 -b feature/phase-1-foundation

# Set up communication
mkdir -p .worktrees/phase-1/.phase-status
```

### 3. Launch Subagent

Open a new AI agent session in the phase worktree with clear instructions:

```
This is Phase 1 at [PATH] on branch feature/phase-1-foundation.

Tasks:
1. [Task 1]
2. [Task 2]

⚠️ CRITICAL: Read .phase-status/MASTER-NOTES.md for testing requirements!
```

### 4. Orchestrate

As master orchestrator:
- Check phase progress every 10-30 minutes
- Provide guidance via MASTER-NOTES.md
- **Review PRs and present recommendations to user**
- **User approves/rejects merge** (Master executes after approval)
- Clean up worktrees after PR merge (see Step 6)

### 5. (Optional) Enable Tmux Monitoring

For real-time, autonomous monitoring with automated alerts:

```bash
# Set up tmux session with dashboard
./scripts/setup-tmux-session.sh

# Attach to monitor all phases
tmux attach -t agentic-dev
```

See [TMUX-ORCHESTRATION.md](./TMUX-ORCHESTRATION.md) for the complete guide.

### 6. Cleanup After PR Merge

```bash
# After PR is merged to develop, clean up the worktree
git worktree remove .worktrees/phase-1
git push origin --delete feature/phase-1-foundation
git worktree prune
```

## Documentation

- **[Full Guide](./AGENTIC-DEVELOPMENT-PROCESS.md)** - Complete operational playbook
- **[Tmux Orchestration](./TMUX-ORCHESTRATION.md)** - Real-time monitoring with automated alerts
- **[Templates](./templates/)** - Ready-to-use templates for all coordination files
- **[Scripts](./scripts/)** - Automation scripts for tmux setup and monitoring
- **[Examples](./examples/)** - Sample phase breakdowns for different project types

## Automation Features

### Pre-Commit Hooks

Enforce quality gates automatically before every commit:

```bash
# Install hooks (choose one method)
git config core.hooksPath githooks
# OR
cp githooks/* .git/hooks/ && chmod +x .git/hooks/*
```

The pre-commit hook checks:
- No secrets/credentials in code
- Linting passes
- Tests pass
- Build succeeds

### Session Management

**Automatic with Claude Hooks (recommended):**

The hooks auto-detect your role based on directory structure:
- **Main repo** (has `.worktrees/`) → Master Orchestrator context
- **Phase worktree** (has `.phase-status/`) → Subagent context

**One-time setup** (works for both roles):
```bash
mkdir -p .claude/hooks
cp hooks/session-start.sh .claude/hooks/
cp hooks/session-end.sh .claude/hooks/
chmod +x .claude/hooks/*.sh
cp templates/claude-settings.json .claude/settings.json
```

| Role | Auto-Detected When | SessionStart Shows |
|------|-------------------|-------------------|
| Master | In main repo with `.worktrees/` | All phases overview, blockers |
| Subagent | In worktree with `.phase-status/` | Master notes, current task |

Zero configuration needed - role is detected automatically!

## Core Concepts

### File-Based Communication

```
Subagent works → Updates PROGRESS.md → Master reads
                                              ↓
User reads ← Master updates dashboard ← Master provides guidance
```

### Quality Gates (All Must Pass)

1. Code complete
2. Build passes (0 errors)
3. Tests created
4. Tests pass (100%)
5. Manual testing done
6. Database verified
7. Integration verified
8. Security checked
9. Documentation created
10. Performance acceptable

### Agent Guardrails

Subagents are explicitly prohibited from:
- Skipping testing
- Merging to develop
- Committing without 100% tests passing
- Making merge decisions

Only the Master Orchestrator merges.

## Results You Can Expect

With this process:
- **Fast development** via parallel worktrees
- **High quality** via mandatory testing + reviews
- **Easy resume** via file-based context
- **Low bugs** via test-before-merge
- **Clear ownership** via one agent per phase
- **Sustainable velocity** via enforced quality gates

## Contributing

Issues and PRs welcome. This is a living document based on real-world usage.

## License

MIT License - Use freely for any project.

---

*This is a battle-tested process for high-quality, high-velocity AI-assisted development.*
