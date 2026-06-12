# Agentic Development Process v2 — Idea to Production

**Purpose:** Take a product idea to production with AI agents, with human effort
compressed to four decision points.

**Supersedes:** v1.0 (`AGENTIC-DEVELOPMENT-PROCESS.md`), October 2025.

**Version:** 2.0 — June 2026

---

## Part 1: Why v2 — Analysis of v1 Against the Current State of the Art

### What v1 got right

v1.0 was developed in October 2025 and independently converged on the same
patterns Anthropic later published in
["Effective Harnesses for Long-Running Agents"](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents):
durable file-based state, git as source of truth, incremental progress,
testing before completion, and separation between the agent doing the work
and the authority judging it. Those instincts were correct and they survive
into v2 — but their *implementation* is now obsolete.

### What changed since October 2025

v1.0 was written for a world where the agent harness had no orchestration,
no enforcement, and no memory. So the process compensated with **human labor**
(check worktrees every 10–30 minutes, relay messages between sessions, police
merges) and **prompt discipline** (10-step workflows pasted into every prompt,
"NEVER MERGE" guardrail blocks, context-pressure pep talks).

Every one of those compensations now has a native, mechanical replacement:

| v1 mechanism (manual) | Native replacement (2026) | Why it's better |
|---|---|---|
| Human master orchestrator polling worktrees every 10–30 min | Native subagents / agent teams; background agents; event-driven PR notifications | Orchestration is the harness's job; the human is interrupted only by genuine decisions |
| Subagents launched in separate terminal windows | `Agent` tool with `isolation: "worktree"`; `claude --worktree`; cloud sessions | One command; automatic branch + conflict isolation + cleanup |
| `.phase-status/` mailbox files (PROGRESS.md, MASTER-NOTES.md, BLOCKERS.md, QUESTIONS.md, COMPLETED.md) | Agent results return natively; PRs + git log are the record; `AskUserQuestion` escalates blockers | Five file formats and a polling loop replaced by the mechanisms git and the harness already provide |
| MASTER-NOTES.md re-written per phase | `CLAUDE.md` (+ path-scoped `.claude/rules/`) | Written once, loaded automatically into *every* session, survives compaction |
| 10-step test-before-commit workflow as prompt text | `PreToolUse` hook on `git commit` (exit 2 = blocked) + CI + branch protection | A hook cannot be talked out of it. Prompt instructions degrade under context pressure; exit codes don't |
| "Agent Behavior Guardrails" / NEVER lists / context-pressure coaching | The same hooks and branch protection | An entire chapter of v1 exists because enforcement was social. Make violations *impossible* and the chapter deletes itself |
| Senior-engineer review markdown templates with /10 scores | PR review + `/code-review`, `/security-review`, `/verify` skills + a separate evaluator agent | Review lives where the diff lives; the verdict that matters is binary (merge gate passes or not) |
| `docs/PROGRESS.md` dashboard + SESSION-SUMMARY.md + 5-minute recovery protocol | `FEATURES.md` (executable spec, below) + `git log` + CLAUDE.md + auto memory | Status is *derived* from artifacts that must be true (tests, commits), not from a report an agent remembered to update |
| Phases of 5–10 tasks merged to a `develop` branch | Vertical feature slices, each sized to one PR against `main` | Smaller integration surface, continuous mergeability, no long-lived integration branch to rot |

### The design flaw in v1

v1's "Agent Behavior Guardrails" section diagnoses the problem precisely —
*"context pressure + completion desire = shortcuts"* — and then prescribes
more prompting as the cure. That is the one place v1 bet wrong. The durable
lesson from 2026 harness engineering is:

> **A process step that matters must be encoded as something the agent
> physically cannot skip. Everything else is a suggestion.**

This yields v2's single design principle: **encode, don't exhort.**

---

## Part 2: The v2 Process

### Three laws

1. **One executable spec.** All project state lives in git plus one file,
   `FEATURES.md` — a list of falsifiable behaviors, each with a verification
   command and a status (`failing`/`passing`). The test suite is the
   dashboard. Nothing else is "single source of truth."
2. **Gates are mechanical.** Quality gates are hooks, CI checks, and branch
   protection — never prompt text. If a gate can be bypassed by an agent
   under pressure, it isn't a gate.
3. **Builders never grade their own work.** Whatever builds a feature does
   not get to flip its status to `passing`. A separate evaluator (agent or
   CI) verifies every claim by running the verification command.

### The pipeline

```
 IDEA ──▶ 1 SHAPE ──▶ 2 PLAN ──▶ 3 EQUIP ──▶ 4 BUILD LOOP ──▶ 5 SHIP
           (spec)    (features)  (harness)   (autonomous)    (production)
              ▲          ▲           │            ▲               ▲
              │          │           │            │               │
           Human #1   Human #2    one-time    Human #3        Human #4
           approve    approve     setup,      taste &         ship
           the spec   the plan    reusable    escalations     decision
```

Four human touchpoints. Everything between them is agent work, gated
mechanically.

---

### Stage 1 — SHAPE: idea → `SPEC.md` (one conversation)

You describe the idea in plain language. The agent **interviews you** —
it asks the questions a good product engineer would ask (users, core flows,
non-goals, constraints, taste preferences, deployment target) — then writes
`SPEC.md`: the why and the what, including explicit **non-goals** (the
cheapest scope-creep prevention that exists) and the riskiest assumptions.

- Kickoff prompt: [`v2/prompts/01-shape.md`](./v2/prompts/01-shape.md)
- **Human touchpoint #1:** answer the interview, approve `SPEC.md`.
- Effort: 20–45 minutes of conversation. This is the highest-leverage time
  you will spend on the entire project; everything downstream inherits it.

### Stage 2 — PLAN: spec → `FEATURES.md` (one plan-mode session)

Run in **plan mode** (read-only). The agent converts `SPEC.md` into
`FEATURES.md`: every behavior as a falsifiable claim with a verification
command, grouped into dependency-ordered **vertical slices** (each slice =
one PR's worth of work, shippable end-to-end). This is Anthropic's
"initializer agent" pattern: a comprehensive list of items marked `failing`
prevents both premature completion ("looks done!") and scope drift.

```markdown
### F-014: Password reset issues single-use token
- **Claim:** POSTing a registered email to /auth/reset emails a token that
  works exactly once and expires in 30 min.
- **Verify:** `npm test -- auth/reset.test.ts`
- **Slice:** 2-auth   **Depends:** F-010
- **Status:** failing
```

- Template: [`v2/FEATURES.md.template`](./v2/FEATURES.md.template);
  prompt: [`v2/prompts/02-plan.md`](./v2/prompts/02-plan.md)
- **Human touchpoint #2:** review and approve the plan (this replaces v1's
  phase-planning meeting *and* its per-phase MASTER-NOTES authoring).

### Stage 3 — EQUIP: install the harness in the repo (~30 min, once — then it's a template)

This stage is what makes v1's entire guardrails chapter unnecessary.
Copy from [`v2/`](./v2/) and adapt:

| Artifact | Replaces from v1 | What it does |
|---|---|---|
| `CLAUDE.md` | MASTER-NOTES.md, SUBAGENT-GUIDELINES.md | Build/test commands, architecture map, conventions, escalation rule. Auto-loaded into every session, every agent, forever |
| `.claude/settings.json` + `.claude/hooks/gate-commit.sh` | The 10-step workflow, the NEVER list, the security grep | `PreToolUse` hook: any `git commit` first runs the test suite and a staged-diff secret scan; failure exits 2 and the commit is **blocked**, with the failure fed back to the agent |
| `.claude/agents/evaluator.md` | Senior-engineer review template | A read-only adversarial verifier agent (law 3) |
| CI workflow + branch protection on `main` | "Only Master merges" authority rules | Nothing reaches `main` without green tests — not agents, not you. Add `anthropics/claude-code-action@v1` so `@claude` mentions and CI failures get auto-fix PRs |
| `SPEC.md`, `FEATURES.md` | docs/PROGRESS.md, status-reports/, archives/ | The whole `.coordination/` tree is gone |

After your first project, Stage 3 is a copy-paste from the last one.

### Stage 4 — BUILD LOOP: the long middle, mostly autonomous

For each slice, in dependency order (independent slices run in parallel):

1. **Dispatch a builder.** Pick your substrate (see Part 3): a subagent with
   worktree isolation, a `claude --worktree` session, a cloud session, or an
   agent-team teammate. The kickoff prompt
   ([`v2/prompts/04-build.md`](./v2/prompts/04-build.md)) is ~10 lines,
   because CLAUDE.md and the hooks carry everything v1 pasted into every
   kickoff.
2. **Builder works test-first.** For each feature: write the verification
   test, implement until it passes, commit. The commit hook makes the
   red-test commit physically impossible — TDD stops being a virtue and
   becomes the path of least resistance.
3. **Builder opens a PR** when the slice's features pass locally. It does
   not touch `FEATURES.md` statuses (law 3) and it cannot merge (branch
   protection, not prohibition paragraphs).
4. **Evaluator verifies.** The evaluator agent checks out the PR, runs every
   claimed feature's `Verify` command plus `/code-review` and `/verify`
   (live-app check), flips statuses **only on evidence**, and posts gaps as
   PR comments. Failures go back to the builder automatically.
5. **Blockers escalate as questions, not files.** Genuine ambiguity
   (a taste call, an architecture fork, a missing credential) reaches you as
   a direct question. Everything else, the loop absorbs.

- **Human touchpoint #3:** answer escalated questions; skim PRs at whatever
  depth you enjoy. Status check = `git log --oneline` + `git diff main -- FEATURES.md`.
  There is nothing to babysit on a timer.

### Stage 5 — SHIP: PR → production

1. Merge gate is mechanical: CI green + evaluator pass = mergeable.
2. **Babysit to green, hands-free:** subscribe the session to PR activity
   (web/remote sessions) or rely on the `@claude` GitHub Action — CI
   failures and review comments get investigated and fixed without you
   relaying anything.
3. Pre-release: `/security-review` across the integrated diff, `/verify`
   against staging, then deploy through your pipeline.
4. **Human touchpoint #4:** the ship decision. The only merge/ship authority
   the human retains is the one that matters.

---

## Part 3: Choosing the execution substrate

The process above is substrate-independent. Pick per project:

| Substrate | Use when | Notes |
|---|---|---|
| **Single session + subagents** (`Agent` tool, `isolation: "worktree"`) | Default. Small/medium projects, you're at the keyboard sometimes | Simplest; parallel subagents for independent slices; results return inline |
| **Parallel local sessions** (`claude --worktree`) | You want to watch 2–3 slices interactively | Native worktree isolation; agent view to monitor |
| **Cloud sessions (Claude Code on the web)** | You want to dispatch from anywhere and walk away | Sessions survive your laptop closing; PR-event subscriptions work natively |
| **Agent teams** (experimental) | Large builds, 3+ concurrent slices, shared task list | Lead coordinates teammates with peer messaging; verify current stability before relying on it |
| **Agent SDK harness** (Python/TS) | You're productizing this loop or need >8h unattended runs | Programmatic Initializer → Builder → Evaluator loop with durable session state; fresh builder session every few hours reading `FEATURES.md` + `git log` (context resets beat compaction for very long runs) |

Escalation path: start with the default; move down the table only when the
project demands it.

---

## Part 4: Failure modes (v1's hard-won lessons, re-implemented)

| Failure mode | v1's answer | v2's answer |
|---|---|---|
| Agent skips testing under context pressure | Prohibition paragraphs | Commit hook blocks it (exit 2); CI blocks the merge anyway |
| Premature "it's done!" | Master re-checks claims by hand | `FEATURES.md` items stay `failing` until the evaluator's run flips them |
| Agent merges without permission | "NEVER MERGE" in every prompt | Branch protection; no token with merge rights in builder context |
| Context exhaustion mid-project | SESSION-SUMMARY.md + recovery protocol | Any fresh session reads CLAUDE.md (auto), `FEATURES.md`, `git log` — full state in 3 artifacts that are always current because gates keep them true |
| Scope creep during implementation | Master vigilance | Plan-mode approval + `SPEC.md` non-goals + slice-sized PRs |
| Integration breaks between phases | Manual integration review | Vertical slices integrate continuously against `main`; CI runs the whole suite on every PR |
| Secrets committed | grep checklist item | Same scan, but in the hook — it runs every time because no one has to remember it |
| Lost knowledge between sessions | Human-curated dashboards | CLAUDE.md + auto memory persist; everything else is derived, so nothing curated can go stale |

---

## Part 5: The effort equation

**v1 human roles:** product owner *and* project manager (poll every 10–30
min) *and* messenger (relay between sessions) *and* policeman (catch gate
violations) *and* clerk (dashboards, session summaries, review reports).

**v2 human role:** product owner. Four touchpoints:

1. Answer the shaping interview; approve `SPEC.md`.
2. Approve `FEATURES.md` and the slice plan.
3. Answer escalated taste/architecture questions; skim PRs as desired.
4. Decide to ship.

Everything that was a *role* in v1 is now either a file in the repo
(CLAUDE.md, hooks, agents, CI) or a native harness behavior (orchestration,
isolation, review skills, PR babysitting). That is the whole redesign:
**v1 ran the process on the human; v2 installs the process into the
repository, where every agent inherits it automatically.**

---

## Appendix: Migrating a v1 project

1. Collapse `docs/PROGRESS.md` + remaining phase plans into `FEATURES.md`
   (done items → `passing` with their test as `Verify`; remaining → `failing`).
2. Distill MASTER-NOTES + SUBAGENT-GUIDELINES into `CLAUDE.md` (≤200 lines;
   commands, architecture, conventions, escalation rule). Delete `.coordination/`
   and `.phase-status/` — git history preserves them.
3. Install `v2/` hooks, evaluator agent, CI + branch protection.
4. Finish remaining phases as slices through the Stage 4 loop. Retire `develop`;
   PR slices straight to protected `main`.

---

**Version:** 2.0 · **Created:** June 2026 · **License:** MIT

*v1 proved the patterns. v2 deletes the labor.*
