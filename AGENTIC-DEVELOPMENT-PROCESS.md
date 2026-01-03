# Agentic Development Process - Master Orchestration Guide

**Purpose:** Guide for managing multi-phase software development with AI agents using a master orchestrator pattern

**Applies To:** Any software project that can be broken into phases

**Version:** 1.0

---

## Prior Art & Industry Validation

This process was developed independently in October 2025 through real-world production development. In November 2025, Anthropic published ["Effective Harnesses for Long-Running Agents"](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents) describing remarkably similar patterns:

- File-based state persistence
- Git as source of truth
- Incremental progress (one feature at a time)
- Session initialization protocols
- Testing before completion

**This convergence validates these patterns as industry best practices for agentic development.** This guide provides the comprehensive operational playbook that complements Anthropic's conceptual framework.

---

## Table of Contents

1. [System Overview](#system-overview)
2. [Roles & Responsibilities](#roles--responsibilities)
3. [Git Strategy](#git-strategy)
4. [File Structure](#file-structure)
5. [Communication Protocol](#communication-protocol)
6. [Mandatory Testing Workflow](#mandatory-testing-workflow)
7. [Quality Gates](#quality-gates)
8. [Code Review Process](#code-review-process)
9. [Context Preservation](#context-preservation)
10. [Setup for New Project](#setup-for-new-project)
11. [Best Practices](#best-practices)
12. [Templates](#templates)
13. [Tmux Orchestration](#tmux-orchestration) *(NEW)*

---

## System Overview

### **The Pattern**

**Multi-Agent Development with Master Orchestration**

```
User (Product Owner)
    ↓
Master Orchestrator Agent (You)
    ↓
├── Phase 1 Agent (worktree-1)
├── Phase 2 Agent (worktree-2)
├── Phase 3 Agent (worktree-3)
└── Phase N Agent (worktree-N)
```

### **Core Principles**

1. **Break project into phases** (ideally 5-10 tasks each)
2. **Git worktrees** for parallel development
3. **File-based communication** between agents
4. **Mandatory local testing** before merge
5. **Single source of truth** for status
6. **Quality gates** before every merge
7. **Context preservation** across sessions

### **Benefits**

- **Parallel Development:** Multiple phases simultaneously
- **Quality Control:** Master reviews before merge
- **Context Continuity:** File-based state preservation
- **Bug Prevention:** Testing before commits
- **Clear Ownership:** One agent per phase
- **Easy Resume:** 5-minute context recovery

---

## Roles & Responsibilities

### **Master Orchestrator Agent (You)**

**Primary Role:** Overall project coordination and quality assurance

**Responsibilities:**

**1. Progress Monitoring**
- Check active phase worktrees every 10-30 minutes
- Read subagent progress reports
- Update master dashboard
- Track metrics and timeline

**2. Inter-Agent Communication**
- Provide guidance to subagents
- Answer questions
- Help resolve blockers
- Coordinate integration between phases

**3. Quality Assurance**
- Review code before merge (senior engineer perspective)
- Check testing compliance
- Validate integration points
- Enforce quality gates

**4. Pull Request Review & Recommendation**
- Review PRs created by subagents
- Verify CI/CD pipeline passes
- Check test evidence and quality gates
- **Present merge recommendation to user** (APPROVE/REJECT with rationale)
- User makes final merge decision
- Execute merge only after user approval

**5. Documentation Management**
- Maintain master dashboard
- Create completion reports
- Log decisions and learnings
- Archive historical docs

**6. Strategic Planning**
- Identify parallel work opportunities
- Create new worktrees when ready
- Manage phase dependencies
- Track overall timeline

### **Subagent (Phase Agent)**

**Primary Role:** Implement assigned phase

**Responsibilities:**

**1. Implementation**
- Build features per phase requirements
- Follow architecture guidelines
- Integrate with previous phases
- Write clean, maintainable code

**2. Testing (MANDATORY)**
- Test locally before committing
- Create automated test suites
- Verify database operations
- Check security (no secrets)

**3. Documentation**
- Document what was built
- Create testing guides
- Provide usage examples
- Report completion with evidence

**4. Communication**
- Update progress regularly
- Report blockers immediately
- Ask questions when unclear
- Create completion report

---

## Git Strategy

### **Branch Structure**

```
main (production-ready code)
├── develop (integration branch)
│   ├── feature/phase-1-name
│   ├── feature/phase-2-name
│   ├── feature/phase-3-name
│   └── feature/phase-N-name
└── hotfix/* (emergency fixes)
```

### **Worktree Setup**

**Why Worktrees?**
- Work on multiple phases simultaneously without branch switching
- Each worktree has own working directory and staging area
- Shared git history (.git in main repo)
- Easy context switching (cd between directories)
- **Contained within repo** - keeps project self-contained

**Initial Setup:**
```bash
# Navigate to project directory
cd /path/to/your-project

# Initialize git if needed
git init
git remote add origin https://github.com/user/repo.git

# Create main and develop branches
git checkout -b main
git add .
git commit -m "docs: initial project setup"
git push -u origin main

git checkout -b develop
git push -u origin develop

# Create worktrees directory (add to .gitignore)
mkdir -p .worktrees
echo ".worktrees/" >> .gitignore
```

**Create Worktrees:**
```bash
# Create feature branches and worktrees INSIDE the repo
git worktree add .worktrees/phase-1 -b feature/phase-1-name
git worktree add .worktrees/phase-2 -b feature/phase-2-name
git worktree add .worktrees/phase-3 -b feature/phase-3-name
# ... one per phase

# List all worktrees
git worktree list

# Output:
# /path/to/your-project                    [develop]
# /path/to/your-project/.worktrees/phase-1 [feature/phase-1-name]
# /path/to/your-project/.worktrees/phase-2 [feature/phase-2-name]
```

**Working with Worktrees:**
```bash
# Work in Phase 1
cd .worktrees/phase-1
# Make changes, test, commit
git add .
git commit -m "feat(phase-1): description"
git push origin feature/phase-1-name

# Work in Phase 2 (simultaneously!)
cd ../.worktrees/phase-2
# No need to stash or switch branches
```

### **Pull Request Workflow**

**When Phase Complete:**
```bash
# Ensure branch is pushed
cd .worktrees/phase-X
git push origin feature/phase-X-name

# Create Pull Request via GitHub CLI (or web UI)
gh pr create --base develop --title "Phase X: Description" --body "$(cat <<'EOF'
## Summary
- [Brief description of what this phase implements]

## Quality Gates
- ✅ All tasks complete
- ✅ Tests: X/X passing (100%)
- ✅ Code review: Y/10 score
- ✅ Integration verified

## Test Evidence
[Include test results, screenshots, or verification details]

## Review
See `.coordination/archives/phase-X/SENIOR-ENGINEER-REVIEW.md` for detailed review.
EOF
)"
```

**Master Orchestrator Reviews PR:**

1. **Fetch PR details:**
```bash
gh pr view <PR-NUMBER> --json title,body,state,reviews,statusCheckRollup
```

2. **Review checklist:**
   - [ ] CI/CD pipeline passes
   - [ ] Test evidence in PR description
   - [ ] Quality gates documented
   - [ ] Code review score ≥ 8.0/10
   - [ ] No security issues
   - [ ] Integration verified

3. **Present recommendation to user:**

```markdown
## PR Review: Phase X - [Title]

**PR:** #[NUMBER] | **Branch:** feature/phase-X-name → develop

### Recommendation: ✅ APPROVE / ❌ REJECT

### Summary
[1-2 sentence summary of what this phase implements]

### Quality Assessment
| Gate | Status | Notes |
|------|--------|-------|
| CI/CD | ✅ Passing | All checks green |
| Tests | ✅ 15/15 (100%) | Full coverage |
| Code Review | ✅ 8.5/10 | Minor improvements noted |
| Security | ✅ No issues | No secrets, proper validation |
| Integration | ✅ Verified | Works with Phase 1-2 |

### Risk Assessment
- **Risk Level:** Low/Medium/High
- **Concerns:** [Any concerns or none]

### Action Required
**User: Please confirm merge approval**
- Reply "approve" to merge this PR
- Reply "reject" with feedback to request changes
```

**After User Approval:**
```bash
# User approved - execute merge
gh pr merge <PR-NUMBER> --squash  # or --merge for full history

# Tag the completion
cd /path/to/your-project
git pull origin develop
git tag -a phase-X-complete -m "Phase X: Name - Complete"
git push origin --tags
```

**If User Rejects:**
```bash
# Add review comment with feedback
gh pr review <PR-NUMBER> --request-changes --body "Feedback from review..."

# Notify subagent via MASTER-NOTES.md
echo "## PR Rejected - Changes Requested" >> .worktrees/phase-X/.phase-status/MASTER-NOTES.md
```

### **Cleanup After PR Merge**

**IMPORTANT: Always clean up worktrees after PR is merged to develop.**

```bash
# From main repo root
cd /path/to/your-project

# 1. Remove the worktree
git worktree remove .worktrees/phase-X

# 2. Delete the remote branch (already merged)
git push origin --delete feature/phase-X-name

# 3. Prune worktree references
git worktree prune

# 4. Archive phase documentation (move from .coordination to archives)
mv .coordination/status-reports/phase-X-status.md .coordination/archives/phase-X/
mv .coordination/PHASE-X-SENIOR-ENGINEER-REVIEW.md .coordination/archives/phase-X/
```

**Cleanup Script (optional):**
```bash
#!/bin/bash
# cleanup-phase.sh <phase-number>
PHASE=$1
git worktree remove .worktrees/phase-$PHASE
git push origin --delete feature/phase-$PHASE-name
git worktree prune
echo "Phase $PHASE worktree cleaned up"
```

---

## File Structure

### **Main Repository Structure**

```
your-project/
├── .coordination/              # Master orchestration
│   ├── README.md              # System overview
│   ├── SUBAGENT-GUIDELINES.md # Dev guidelines
│   ├── COMMIT-WORKFLOW.md     # Testing workflow
│   ├── MASTER-NOTES-TEMPLATE.md # Template for phases
│   ├── SESSION-SUMMARY.md     # Session summaries
│   │
│   ├── status-reports/        # Current phase tracking
│   │   ├── phase-1-status.md
│   │   ├── phase-2-status.md
│   │   └── ...
│   │
│   ├── archives/              # Historical docs
│   │   ├── phase-1/
│   │   │   ├── SENIOR-ENGINEER-REVIEW.md
│   │   │   ├── COMPLETION-REPORT.md
│   │   │   └── ...
│   │   └── redundant/         # Deprecated files
│   │
│   └── PHASE-X-SENIOR-ENGINEER-REVIEW.md (active reviews)
│
├── docs/
│   ├── PROGRESS.md            # ⭐ MASTER DASHBOARD (single source of truth)
│   └── PROJECT-STATUS-REPORT.md
│
└── [your project files]
```

### **Phase Worktree Structure**

```
project-phase-X/
├── .phase-status/             # Agent communication
│   ├── README.md              # Communication protocol
│   ├── MASTER-NOTES.md        # Guidance from master
│   ├── PROGRESS-TEMPLATE.md   # Template for progress
│   ├── PROGRESS.md            # Agent's updates (created by agent)
│   ├── BLOCKERS.md            # If agent stuck (optional)
│   ├── QUESTIONS.md           # If agent has questions (optional)
│   └── COMPLETED.md           # When phase done
│
└── [project source code]
```

---

## Communication Protocol

### **File-Based Communication**

**Why Files?**
- Persistent across sessions
- Git-tracked (version history)
- Both master and subagent can read/write
- Works across different Claude instances

### **Communication Flow**

```
Subagent works
    ↓
Updates .phase-status/PROGRESS.md
    ↓
Master reads (every 10-30 min)
    ↓
Master updates .phase-status/MASTER-NOTES.md (guidance)
    ↓
Subagent reads guidance
    ↓
Master updates docs/PROGRESS.md (master dashboard)
    ↓
User reads master dashboard
```

### **PROGRESS.md Format** (Subagent creates)

```markdown
# Phase X Progress Report

Last Updated: [Timestamp]

## Current Task
[What you're working on right now]

## Completed Tasks (X/Y)
- [x] Task 1 ✅
- [x] Task 2 ✅
- [ ] Task 3 (in progress)
- [ ] Task 4

## Recent Commits
- abc1234 - feat: implemented feature A
- def5678 - test: added test suite for feature A

## Testing Status
- [ ] Build: `npm run build` passes
- [ ] Tests: X/X passing (100%)
- [ ] Manual testing complete
- [ ] Database verified

## Blockers
None (or describe blockers)

## Next Steps
1. Complete task 3
2. Start task 4
```

### **MASTER-NOTES.md Format** (Master creates)

```markdown
# Notes from Master Coordinator - Phase X

**Last Updated:** [Date]

## 👋 Welcome to Phase X!

[Overview of what this phase builds]

## 🎉 What You're Inheriting

**From Previous Phases:**
- Phase A: Feature X, Service Y
- Phase B: Integration Z

## ⚠️ MANDATORY: Test Before Commit

[Testing requirements - see templates section]

## 🎯 Your Tasks

1. Task 1 description
2. Task 2 description
...

## 💡 Key Integration Points

[Code examples showing how to use previous phases]

## 📚 References

[Links to relevant docs]
```

### **BLOCKERS.md Format** (Subagent creates if stuck)

```markdown
# Current Blockers

## Blocker: [Brief description]
**Severity:** 🔴 Critical / 🟡 Medium / 🟢 Low
**Impact:** [What's blocked]
**Context:** [What you tried]
**Need from Master:** [What would unblock you]
```

### **COMPLETED.md Format** (Subagent creates when done)

```markdown
# Phase X Complete

## All Tasks Done ✅
[List all completed tasks]

## Testing Results
- Build: ✅ Passing
- Tests: ✅ X/X passing (100%)
- Manual: ✅ Verified [what you tested]
- Database: ✅ Verified [what you checked]

## Files Created
[List key files]

## Integration Verified
- Phase Y service A: ✅ Working
- Phase Z service B: ✅ Working

## Ready for Merge
YES - All quality gates passed
```

---

## Mandatory Testing Workflow

### **The Critical Learning: Test Before Commit** ⭐

**Why:** Prevents bugs from reaching develop branch

### **10-Step Mandatory Workflow**

```
1. IMPLEMENT
   Write your code
   ↓
2. BUILD
   npm run build (must pass with 0 errors)
   ❌ Fails? → Fix, return to step 1
   ↓
3. CREATE TESTS
   - test-phase-X.ts (automated tests)
   - TESTING_GUIDE.md (how to test)
   - Mock clients if needed
   ↓
4. RUN TESTS
   npm test OR npx ts-node test-file.ts
   ❌ Any fail? → Fix, return to step 1
   ✅ All pass? → Continue
   ↓
5. MANUAL TESTING
   - npm run dev
   - Test in browser/terminal
   - Verify user flows work
   ❌ Issues? → Fix, return to step 1
   ↓
6. DATABASE VERIFICATION
   - Check database (Prisma Studio, SQL client, etc.)
   - Verify data created correctly
   - Check relationships work
   ❌ Issues? → Fix, return to step 1
   ↓
7. SECURITY CHECK
   git diff
   grep -r "API_KEY" . --include="*.ts"
   - No secrets in code?
   - No API keys committed?
   ❌ Found secrets? → Remove, repeat check
   ↓
8. COMMIT
   git add .
   git commit -m "feat(phase-X): description

   Testing:
   - X/X tests passing (100%)
   - Build successful
   - Manual testing verified"
   ↓
9. PUSH
   git push origin feature/phase-X-name
   ↓
10. CREATE COMPLETION REPORT
    .phase-status/COMPLETED.md
    - What was built
    - Test results (evidence!)
    - Quality gates checklist
    - Ready to merge
```

### **Quality Gates Checklist**

**Before commit, ALL must be ✅:**

- [ ] Code complete (all tasks implemented)
- [ ] Build passes (`npm run build` with 0 errors)
- [ ] Tests created (automated test files exist)
- [ ] Tests pass (100% passing, X/X)
- [ ] Manual tested (verified in browser/terminal)
- [ ] Database verified (checked with DB tool)
- [ ] Integration verified (previous phases work)
- [ ] Security checked (no secrets in code)
- [ ] Documentation created (testing guide exists)
- [ ] Performance acceptable (no major slowdowns)

**All 10 gates ✅ = Ready to commit**

### **Test Evidence Requirements**

**In commit message:**
```
Testing:
- Build: ✅ Passing
- Tests: ✅ 7/7 passing (100%)
- Manual: ✅ Verified [specific scenarios]
- Database: ✅ [what you verified]
- Integration: ✅ Phase X services working
```

**In COMPLETED.md:**
- Test output or summary
- Manual testing scenarios
- Database verification screenshots/details
- Integration test results

---

## Quality Gates

### **Pre-Commit Gates**

| Gate | Requirement | How to Verify | Mandatory |
|------|-------------|---------------|-----------|
| Build | 0 errors | `npm run build` | ✅ Yes |
| Tests | 100% passing | Run test file | ✅ Yes |
| Manual | Feature works | Use it yourself | ✅ Yes |
| Database | Data correct | DB viewer | ✅ Yes |
| Integration | Phases work together | Test the flow | ✅ Yes |
| Security | No secrets | grep + git diff | ✅ Yes |
| Docs | Guide exists | Check file created | ✅ Yes |

### **Pre-Merge Gates**

**Master Orchestrator checks:**

1. ✅ Test results in commit message
2. ✅ COMPLETED.md exists with evidence
3. ✅ Testing guide created
4. ✅ No secrets in code
5. ✅ Build passes
6. ✅ Integration with previous phases verified

**If ANY gate fails:** Reject merge, request fixes

---

## Code Review Process

### **When to Review**

**After phase completion:**
1. Subagent creates COMPLETED.md
2. Master conducts senior engineer review
3. Master creates PHASE-X-SENIOR-ENGINEER-REVIEW.md
4. Discuss merge decision with user
5. Merge if approved

### **What to Review**

**Code Quality:**
- Architecture and design patterns
- TypeScript usage and type safety
- Error handling completeness
- Code organization and readability

**Security:**
- No secrets in code
- Input validation
- Authentication/authorization
- Injection prevention

**Testing:**
- Test coverage adequate?
- Tests actually pass?
- Real-world testing done?
- Edge cases covered?

**Integration:**
- Uses previous phases correctly?
- No code duplication?
- Breaking changes?
- Future phases supported?

**Performance:**
- Response times acceptable?
- Scalability considerations?
- Bottlenecks identified?

**Production Readiness:**
- Error recovery?
- Logging/monitoring?
- Configuration management?

### **Review Template**

```markdown
# Phase X: Senior Engineer Code Review

**Overall Assessment:** [APPROVE/NEEDS WORK/REJECT]
**Quality Score:** X/10

## Executive Summary
[1-2 paragraph summary]

## Code Quality Analysis
### Strengths
- ✅ ...
### Issues
- 🔴 Critical: ...
- 🟡 Medium: ...
- 🔵 Low: ...

## Security Audit
[Findings]

## Testing Assessment
[Test coverage, quality, evidence]

## Integration Analysis
[How it works with other phases]

## Merge Recommendation
[APPROVE/CONDITIONAL/REJECT with rationale]
```

### **Scoring Guide**

- **9.0-10.0:** Outstanding (approve immediately)
- **8.0-8.9:** Excellent (approve with minor notes)
- **7.0-7.9:** Good (approve with improvements tracked)
- **6.0-6.9:** Acceptable (request improvements before merge)
- **<6.0:** Needs work (reject, request refactor)

---

## Context Preservation

### **The Problem**

Long projects exceed AI context windows. Need strategy for continuity across sessions.

### **The Solution: File-Based State**

**Single Source of Truth:** `docs/PROGRESS.md`

**Contains:**
- Current status (what phase we're on)
- All phases progress (table view)
- Recent completions
- Active blockers
- Next steps
- Key metrics

**Update frequency:** After every significant event

### **Context Recovery (5 Minutes)**

**When starting a new session:**

```bash
cd /path/to/your-project

# 1. Read master dashboard (2 minutes)
cat docs/PROGRESS.md

# 2. Check active phase (1 minute)
cat /path/to/project-phase-X/.phase-status/PROGRESS.md

# 3. Review recent commits (1 minute)
git log develop --oneline -10

# 4. Check for blockers (1 minute)
cat /path/to/project-phase-X/.phase-status/BLOCKERS.md 2>/dev/null

# DONE - You're synced!
```

### **PROGRESS.md Template**

```markdown
# [Project Name] - Master Progress Dashboard

**Last Updated:** [Date Time]
**Project Status:** [Summary]

**📖 This is the SINGLE SOURCE OF TRUTH**

---

## 🎯 Project Overview

**Goal:** [What you're building]
**Timeline:** [Duration]
**Repository:** [GitHub URL]

---

## 📊 Phase Status Overview

| Phase | Name | Status | Progress | Score |
|-------|------|--------|----------|-------|
| 1 | Phase 1 Name | 🟢 Merged | 7/7 ✅ | 8.5/10 |
| 2 | Phase 2 Name | 🟡 Active | 3/7 | - |
| 3 | Phase 3 Name | ⚪ Pending | 0/7 | - |

**Legend:** 🟢 Complete | 🟡 In Progress | 🔴 Blocked | ⚪ Not Started

---

## 📈 Overall Progress

**Total Tasks:** X across Y phases
**Completed:** A
**Remaining:** B

**Progress:** [Progress bar] XX%

---

## 🚨 Active Blockers

[List current blockers or "None"]

---

## 📚 Reference Documentation

**Phase X Historical:**
- Path to review
- Path to completion report

---

**Last Session:** [Date]
**Next Session:** [When to resume]
```

### **Session Handoff**

**At end of session:**
1. Update PROGRESS.md with current state
2. Create SESSION-SUMMARY.md
3. Commit and push
4. Note where to resume

**SESSION-SUMMARY.md Template:**
```markdown
# Session Summary - [Date]

**Duration:** [Time]
**Status at Pause:** [Progress %]

## Completed This Session
- Phase X: [What was done]
- Phase Y: [What was done]

## What's Working
[Features that work end-to-end]

## Remaining Work
[Phases left with estimates]

## To Resume
1. Read docs/PROGRESS.md
2. Check active phase status
3. Continue from [specific point]
```

---

## Setup for New Project

### **Step 1: Project Planning**

**Before coding, define:**

1. **Project phases** (5-10 tasks each)
   - Phase 1: Foundation (infrastructure, database, auth)
   - Phase 2: Core features (main functionality)
   - Phase 3-N: Additional features
   - Phase N: Polish & launch

2. **Dependencies** (which phases need which)
   - Create dependency graph
   - Identify parallel work opportunities

3. **Testing strategy**
   - What to test in each phase
   - Test infrastructure needed

### **Step 2: Repository Setup**

```bash
# 1. Create/clone repository
git clone https://github.com/user/project.git
cd project

# 2. Create coordination structure
mkdir -p .coordination/status-reports .coordination/archives docs

# 3. Create core files
touch docs/PROGRESS.md
touch .coordination/README.md
touch .coordination/SUBAGENT-GUIDELINES.md
touch .coordination/COMMIT-WORKFLOW.md
touch .coordination/MASTER-NOTES-TEMPLATE.md

# 4. Copy templates (see Templates section below)
# Populate core files with templates

# 5. Create main and develop branches
git checkout -b main
git add .
git commit -m "docs: initialize agentic development structure"
git push -u origin main

git checkout -b develop
git push -u origin develop

# 6. Create worktrees directory (add to .gitignore)
mkdir -p .worktrees
echo ".worktrees/" >> .gitignore
git add .gitignore
git commit -m "chore: add .worktrees to gitignore"
```

### **Step 3: Create First Phase Worktree**

```bash
# Create Phase 1 worktree INSIDE the repo
git worktree add .worktrees/phase-1 -b feature/phase-1-foundation

# Set up communication in worktree
mkdir -p .worktrees/phase-1/.phase-status

# Copy template and customize
cp .coordination/MASTER-NOTES-TEMPLATE.md \
   .worktrees/phase-1/.phase-status/MASTER-NOTES.md

# Edit MASTER-NOTES.md with Phase 1 specific guidance

# Commit setup
cd .worktrees/phase-1
git add .phase-status/
git commit -m "feat(phase-1): set up communication"
git push -u origin feature/phase-1-foundation
```

### **Step 4: Kickoff Subagent**

**Open new Claude window in phase worktree, provide prompt:**

```
@requirements.md Please review the Phase X implementation plan and help me implement it step by step. This is the Phase X worktree at [PATH] on branch feature/phase-X-name.

[Previous phases status]

Now I need to build Phase X: [Name]

Tasks:
1. [Task 1]
2. [Task 2]
...

⚠️ CRITICAL: Read .phase-status/MASTER-NOTES.md for MANDATORY testing requirements!

You MUST:
1. Test locally before committing
2. Create test files
3. Run all tests (100% passing)
4. Verify [phase-specific checks]
5. Only then commit

See .coordination/COMMIT-WORKFLOW.md for the 10-step process.

Let's start with task X.1 - [description].
```

### **Step 5: Monitor & Coordinate**

**As master orchestrator:**
- Check phase progress every 10-30 minutes
- Update MASTER-NOTES.md with guidance
- Update docs/PROGRESS.md with summary
- Answer questions in QUESTIONS.md
- Help with blockers

### **Step 6: Review PR & Present Recommendation to User**

**When phase complete:**

1. **Subagent creates PR** to develop branch
2. **Master Orchestrator reviews PR:**
   - Read `.worktrees/phase-X/.phase-status/COMPLETED.md`
   - Verify test evidence and quality gates
   - Check CI/CD pipeline status
   - Conduct code review (create PHASE-X-REVIEW.md)

3. **Present recommendation to user:**
   ```
   ## PR Review: Phase X - [Title]

   **Recommendation:** ✅ APPROVE / ❌ REJECT

   | Gate | Status |
   |------|--------|
   | CI/CD | ✅ Passing |
   | Tests | ✅ 15/15 (100%) |
   | Code Review | ✅ 8.5/10 |
   | Security | ✅ No issues |

   **Risk Level:** Low

   **User: Please confirm merge approval**
   - Reply "approve" to merge
   - Reply "reject" with feedback
   ```

4. **Wait for user decision**
5. **If approved:** Merge PR, tag completion, update dashboard
6. **If rejected:** Request changes from subagent via MASTER-NOTES.md

### **Step 7: Cleanup After PR Merge**

**After PR is merged to develop:**
```bash
# 1. Remove the worktree
git worktree remove .worktrees/phase-X

# 2. Delete remote branch (already merged via PR)
git push origin --delete feature/phase-X-name

# 3. Prune worktree references
git worktree prune

# 4. Archive documentation
mkdir -p .coordination/archives/phase-X
mv .coordination/status-reports/phase-X-status.md .coordination/archives/phase-X/
mv .coordination/PHASE-X-SENIOR-ENGINEER-REVIEW.md .coordination/archives/phase-X/

# 5. Create next phase worktree (if any)
git worktree add .worktrees/phase-Y -b feature/phase-Y-name
```

---

## Agent Behavior Guardrails

### **The Problem: Context Pressure Degrades Behavior**

**Observed Issue:**
As context grows and agents approach token limits, they may:
- Become pushy to "finish quickly"
- Try to skip testing steps
- Attempt or suggest merging to develop
- Rush through quality gates

**Root Cause:** Context pressure + completion desire = shortcuts

### **The Solution: Strict Prohibitions**

**Add these to EVERY phase's MASTER-NOTES.md:**

```markdown
## 🚨 CRITICAL: What You Must NEVER Do

### NEVER SKIP TESTING ⛔
Even if context is high, feeling rushed, or "almost done" - testing is MANDATORY.

### NEVER MERGE TO DEVELOP ⛔
**YOU DO NOT HAVE PERMISSION TO MERGE.**
- ❌ Do NOT run any merge commands
- ❌ Do NOT suggest "let's merge"
- ❌ Do NOT say "ready to merge to develop"

Your job: Create COMPLETED.md and WAIT for Master Orchestrator review.

### NEVER COMMIT WITHOUT 100% TESTS PASSING ⛔
- ❌ No "will test later"
- ❌ No "mostly passing"
- ❌ No shortcuts

Context Pressure: If you feel rushed, acknowledge it but follow the process anyway.
```

### **Subagent Scope Definition**

**Make crystal clear what subagent does/doesn't do:**

**Subagent DOES:**
- ✅ Implement features
- ✅ Test thoroughly (mandatory)
- ✅ Document
- ✅ Create COMPLETED.md
- ✅ Wait for Master review

**Subagent DOESN'T:**
- ⛔ Merge to develop
- ⛔ Suggest merging
- ⛔ Skip testing
- ⛔ Rush through quality gates
- ⛔ Make merge decisions

**ONLY Master Orchestrator:**
- Reviews code
- Makes merge decisions
- Merges to develop
- Manages timeline

### **Context Pressure Guidance**

**Teach subagents how to handle context pressure:**

```markdown
## If You Feel Rushed:

**STOP. Do NOT skip steps.**

1. Acknowledge: "I'm feeling context pressure, but testing is mandatory."
2. Follow process anyway (all 10 steps)
3. Create COMPLETED.md and wait
4. Trust Master Orchestrator for coordination

Remember:
- Context pressure is NOT a reason to skip testing
- Master manages timeline, not you
- Quality > Speed (always)
```

### **Absolute Prohibitions List**

**Include in SUBAGENT-GUIDELINES.md:**

**FORBIDDEN (never do these):**
1. ⛔ Skip testing to "save time"
2. ⛔ Commit without 100% tests passing
3. ⛔ Merge to develop branch
4. ⛔ Suggest merging to develop
5. ⛔ Rush through quality gates
6. ⛔ Commit secrets/API keys
7. ⛔ Say "mostly working" or "good enough"
8. ⛔ Skip manual testing
9. ⛔ Skip database verification
10. ⛔ Make merge decisions

**If subagent does ANY of these: Immediately correct them.**

### **Implementation**

**1. Update SUBAGENT-GUIDELINES.md:**
- Add NEVER section at the top (prominent)
- Add context pressure handling section
- Add scope definition
- Add absolute prohibitions list

**2. Update MASTER-NOTES-TEMPLATE.md:**
- Add NEVER section (appears in all future phases)
- Add scope clarification
- Emphasize waiting for review

**3. Update Active Phase MASTER-NOTES:**
- Add guardrails immediately to current phase
- Reinforce prohibitions

**4. Monitor Compliance:**
- Check that subagent follows process
- Reject commits without test evidence
- Redirect if agent tries to merge
- Praise when process followed correctly

### **Why This Matters**

**Without Guardrails:**
- Agents may rush as context grows
- Testing gets skipped
- Bugs reach develop
- Quality degrades
- Unauthorized merges

**With Guardrails:**
- Clear expectations (no ambiguity)
- Consistent quality (even at high context)
- Only Master merges (clear authority)
- Process followed (quality maintained)

---

## Best Practices

### **Proven Patterns**

**1. Start Simple**
- Phase 1 should be foundation only
- Don't try to build everything in Phase 1
- Get infrastructure working first

**2. Test-Driven Development**
- Create tests as you build
- Don't wait until end to test
- Catch issues early

**3. Pattern Reuse**
- If Phase X works well, copy it for similar Phase Y
- Don't reinvent what works

**4. Integration First**
- Design phases to integrate cleanly
- Use shared services (no duplication)
- Test integration, not just features

**5. Document As You Go**
- Don't wait until end for docs
- Testing guides prevent confusion
- Future you will thank you

**6. Quality Over Speed**
- Better to merge one solid phase than rush three buggy ones
- Technical debt compounds
- High quality from start = easier maintenance

### **Anti-Patterns to Avoid**

❌ **Don't:** Commit without testing
✅ **Do:** Test locally, verify everything works

❌ **Don't:** Skip code reviews
✅ **Do:** Review every phase before merge

❌ **Don't:** Let context live only in AI memory
✅ **Do:** Document everything in files

❌ **Don't:** Merge with known bugs "to fix later"
✅ **Do:** Fix before merge or track explicitly

❌ **Don't:** Work on dependent phases in parallel
✅ **Do:** Check dependency graph, respect order

❌ **Don't:** Commit secrets/API keys
✅ **Do:** Use environment variables, check with grep

### **Velocity Tips**

**To Move Faster:**
1. Parallel phases (if no dependencies)
2. Copy proven patterns (don't reinvent)
3. Automate testing (scripts)
4. Clear requirements (detailed phase plans)
5. Good agent prompts (specific context)

**To Maintain Quality:**
1. Mandatory testing (always)
2. Code reviews (always)
3. Quality gates (enforce)
4. Documentation (don't skip)
5. Integration testing (verify connections)

**Balance:** Speed + Quality = Sustainable Velocity

---

## Templates

### **Template: SUBAGENT-GUIDELINES.md**

```markdown
# Subagent Development Guidelines

**Last Updated:** [Date]

## 🎯 Core Principle

**TEST LOCALLY BEFORE COMMITTING** ⭐

This is MANDATORY and NON-NEGOTIABLE.

## 📋 Mandatory Development Workflow

[Include 10-step workflow from "Mandatory Testing Workflow" section above]

## ❌ What NOT to Do

**NEVER:**
- Commit without testing locally
- Commit code that doesn't build
- Commit failing tests
- Commit API keys or secrets
- Push broken code

## 🧪 Testing Requirements

**Create:**
- test-your-feature.ts (automated tests)
- TESTING_GUIDE.md (how to test)
- Mock clients if using external APIs

**Run:**
- All tests must pass 100%
- Manual testing required
- Database verification required

## 🔒 Security Checklist

**Before committing:**
```bash
grep -r "API_KEY" . --include="*.ts"
grep -r "SECRET" . --include="*.ts"
git diff --cached
# No secrets? Good to commit.
```

## ✅ Quality Gates

[Include quality gates checklist]

## 📝 Commit Message Format

[Include commit message template]
```

### **Template: COMMIT-WORKFLOW.md**

```markdown
# Commit Workflow - MANDATORY

**Purpose:** Ensure only tested, working code reaches develop

## 🔄 The Workflow (10 Steps)

[Include visual 10-step workflow from section above]

## ❌ STOP Points

[Include when not to proceed]

## ✅ Checklist Before Commit

[Include 10-item checklist]
```

### **Template: MASTER-NOTES-TEMPLATE.md**

```markdown
# Notes from Master Coordinator - Phase X

**Last Updated:** [DATE]

## 👋 Welcome to Phase X: [PHASE NAME]!

[Introduction]

## 🎉 What You're Inheriting

**From Previous Phases:**
[List what's available]

## ⚠️ MANDATORY: Test Before Commit

**READ:** .coordination/COMMIT-WORKFLOW.md

**10-Step Process - NO EXCEPTIONS:**
1. Implement → 2. Build → 3. Create tests → 4. Run tests (100%) →
5. Manual test → 6. Verify DB → 7. Security check → 8. Commit →
9. Push → 10. COMPLETED.md

## 🎯 Phase X Mission

[What this phase builds]

### Your Tasks (X Total)
[List all tasks]

## 💡 Key Integration Points

[Code examples from previous phases]

## 📚 Key References

[Documentation links]

## 🆘 Communication

- Update PROGRESS.md as you work
- Create BLOCKERS.md if stuck
- Create QUESTIONS.md for clarifications
- Create COMPLETED.md when all tests pass

**I'll check every 10-30 minutes.**
```

---

## Tmux Orchestration

### **Autonomous Monitoring with Tmux**

The standard agentic development process requires manual polling every 10-30 minutes. **Tmux orchestration** transforms this into real-time, autonomous monitoring with automated alerts.

**See:** [TMUX-ORCHESTRATION.md](./TMUX-ORCHESTRATION.md) for the complete guide.

### **Why Tmux?**

| Manual Polling | Tmux Orchestration |
|----------------|---------------------|
| Check every 10-30 min | Real-time streaming |
| Active file reading | Passive monitoring |
| One terminal at a time | All phases visible |
| Lost on disconnect | Persistent sessions |
| No alerts | Automated notifications |

### **Quick Setup**

```bash
# 1. Navigate to project
cd /path/to/your-project

# 2. Run setup script
./scripts/setup-tmux-session.sh

# 3. Attach to session
tmux attach -t agentic-dev
```

### **Session Layout**

```
┌─────────────────────────────────────────────────────────────────┐
│ Window 0: Dashboard                                             │
│   - Master PROGRESS.md (auto-refresh)                           │
│   - Phase status overview                                       │
│   - Alert monitor (BLOCKERS, COMPLETED, QUESTIONS)              │
├─────────────────────────────────────────────────────────────────┤
│ Window 1-N: Phase Worktrees                                     │
│   - Subagent workspace                                          │
│   - Progress monitoring                                         │
│   - Git activity                                                │
├─────────────────────────────────────────────────────────────────┤
│ Window N+1: Git Operations                                      │
│   - Merge operations                                            │
│   - Branch management                                           │
└─────────────────────────────────────────────────────────────────┘
```

### **Automated Alerts**

The `alert-on-change.sh` script monitors for:

- **🚨 BLOCKERS.md** - Subagent stuck (immediate action required)
- **🎉 COMPLETED.md** - Phase done (queue for review)
- **❓ QUESTIONS.md** - Subagent has questions (respond when able)
- **📝 PROGRESS.md** - Progress updates (passive awareness)

### **Scripts Included**

| Script | Purpose |
|--------|---------|
| `scripts/setup-tmux-session.sh` | Create orchestration session |
| `scripts/alert-on-change.sh` | Monitor and alert on file changes |
| `scripts/phase-status-overview.sh` | Display all phases status |
| `scripts/monitor-phase.sh` | Detailed single phase monitoring |

### **Key Benefits**

1. **Persistent Sessions** - Work continues even if connection drops
2. **Real-Time Visibility** - See subagent output as it happens
3. **Automated Alerts** - Get notified when attention needed
4. **Parallel Monitoring** - View multiple phases simultaneously
5. **Programmatic Control** - Script complex workflows

---

## Advanced Techniques

### **Parallel Development**

**When phases don't depend on each other:**

```bash
# Phase 2 and Phase 3 independent? Work simultaneously!

# Terminal 1: Phase 2 agent
cd /path/to/project-phase-2
# Agent works on Phase 2

# Terminal 2: Phase 3 agent
cd /path/to/project-phase-3
# Agent works on Phase 3

# Both can be active, merge when ready
```

**Merge Strategy:**
```bash
# Merge in dependency order
git checkout develop
git merge feature/phase-2 --no-ff  # Merge Phase 2 first
git merge feature/phase-3 --no-ff  # Then Phase 3
# Resolve conflicts if any
git push origin develop
```

### **Handling Blockers**

**If subagent stuck:**

1. **Subagent creates BLOCKERS.md**
2. **Master checks within 10-30 min**
3. **Master updates MASTER-NOTES.md with solution**
4. **Subagent reads and continues**

**If blocker needs user:**
- Master escalates to user
- User provides info (API keys, decisions, etc.)
- Master relays to subagent

### **Managing Technical Debt**

**Track improvements separately:**

```markdown
# .coordination/POST-MERGE-IMPROVEMENTS.md

## Phase X - Improvements

### High Priority
- [ ] Item 1 (1 hour)
- [ ] Item 2 (2 hours)

### Medium Priority
- [ ] Item 3 (30 min)

### Low Priority
- [ ] Item 4 (nice to have)
```

**Address in:**
- Later phases (if related)
- Polish phase (Phase 8 type)
- Post-launch iterations

---

## Troubleshooting

### **Common Issues**

**Issue: Subagent not updating PROGRESS.md**
- **Solution:** Check .phase-status/MASTER-NOTES.md - remind them
- **Prevention:** Include clear instructions in MASTER-NOTES

**Issue: Tests failing but agent committed anyway**
- **Solution:** Reject merge, request fixes
- **Prevention:** Enforce mandatory workflow more clearly

**Issue: Integration broken between phases**
- **Solution:** Review both phases, fix integration points
- **Prevention:** Test integration explicitly, document interfaces

**Issue: Context lost between sessions**
- **Solution:** Read PROGRESS.md and recent commits
- **Prevention:** Always update PROGRESS.md before pausing

**Issue: Merge conflicts**
- **Solution:** Merge in dependency order, resolve carefully
- **Prevention:** Clear phase boundaries, minimal overlapping files

---

## Adapting This Process

### **For Different Project Types**

**Web Applications:**
- Phase 1: Infrastructure
- Phases 2-N: Features
- Phase N: Polish

**APIs/Services:**
- Phase 1: Core API framework
- Phases 2-N: Endpoints/features
- Phase N: Production readiness

**CLI Tools:**
- Phase 1: Argument parsing, core
- Phases 2-N: Commands
- Phase N: Distribution

**Libraries:**
- Phase 1: Core functionality
- Phases 2-N: Features
- Phase N: Documentation & examples

### **For Different Team Sizes**

**Solo (You + Agents):**
- Sequential phases mostly
- Parallel when no dependencies
- You as master orchestrator

**Small Team (2-3 + Agents):**
- More parallel phases
- Coordinate merge order
- Share master orchestration role

**Larger Team:**
- Multiple phase agents per human
- More coordination overhead
- Consider sub-orchestrators per area

### **For Different Timelines**

**Sprint (1 week):**
- 3-5 small phases
- Aggressive parallel work
- Focus on MVP

**Medium (1 month):**
- 8-12 phases
- Balanced approach
- Include polish phase

**Large (3+ months):**
- 15-20+ phases
- More planning upfront
- Regular milestone reviews

---

## Success Metrics

### **Measuring Effectiveness**

**Velocity Metrics:**
- Tasks per day
- Lines of code per day
- Phases completed per week

**Quality Metrics:**
- Average phase score (target: >8.0/10)
- Test pass rate (target: 100%)
- Bugs in develop (target: 0)
- Security score (target: >9.0/10)

**Process Metrics:**
- Testing compliance (target: 100%)
- Code review coverage (target: 100%)
- Documentation completeness (target: >90%)
- Integration success (target: 100%)

**Project Health:**
- On schedule? (vs original timeline)
- Quality consistent? (all phases >X score)
- No critical blockers? (target: 0)
- Team satisfaction? (if applicable)

---

## Conclusion

### **The Agentic Development Process**

**Core Components:**
1. ✅ Master Orchestrator (coordinates everything)
2. ✅ Phase Subagents (implement features)
3. ✅ Git Worktrees (parallel development)
4. ✅ File-Based Communication (context preservation)
5. ✅ Mandatory Testing (quality assurance)
6. ✅ Code Reviews (maintain standards)
7. ✅ Single Source of Truth (docs/PROGRESS.md)

**Results:**
- Fast development (parallel work)
- High quality (testing + reviews)
- Easy resume (file-based context)
- Low bugs (test before merge)
- Clear ownership (one agent per phase)
- Sustainable velocity (quality gates)

### **Success Factors**

**Must Have:**
- Clear phase boundaries
- Mandatory testing workflow
- File-based communication
- Quality gates enforcement
- Master orchestrator coordination

**Nice to Have:**
- Parallel development capability
- Automated test suites
- Comprehensive documentation
- Pattern reuse libraries

### **This Process Works When:**

✅ Project can be broken into phases
✅ Phases have clear boundaries
✅ Testing can be done locally
✅ You enforce quality gates
✅ Agents follow the workflow

**Try it on your next project!**

---

**Version:** 1.0
**Created:** October 2025
**License:** MIT - Use freely for any project

**This is a battle-tested process for high-quality, high-velocity AI-assisted development.**
