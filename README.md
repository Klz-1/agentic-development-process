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

## Quick Start

### 1. Set Up Your Project

```bash
# Create coordination structure
mkdir -p .coordination/status-reports .coordination/archives docs

# Create core files
touch docs/PROGRESS.md
touch .coordination/SUBAGENT-GUIDELINES.md
touch .coordination/COMMIT-WORKFLOW.md

# Copy templates from templates/ directory
```

### 2. Create Phase Worktrees

```bash
# Create a worktree for Phase 1
git worktree add ../project-phase-1 -b feature/phase-1-foundation

# Set up communication
mkdir -p ../project-phase-1/.phase-status
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
- Review and merge when quality gates pass

## Documentation

- **[Full Guide](./AGENTIC-DEVELOPMENT-PROCESS.md)** - Complete operational playbook
- **[Templates](./templates/)** - Ready-to-use templates for all coordination files
- **[Examples](./examples/)** - Sample phase breakdowns for different project types

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
