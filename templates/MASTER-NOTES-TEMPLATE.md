# Notes from Master Coordinator - Phase X

**Last Updated:** [DATE]

---

## Welcome to Phase X: [PHASE NAME]!

[Brief introduction - what this phase is about and why it matters]

---

## What You're Inheriting

**From Previous Phases:**

| Phase | What's Available |
|-------|-----------------|
| Phase 1 | [List services, components, utilities] |
| Phase 2 | [List what Phase 2 built] |
| ... | ... |

**Key Integration Points:**

```typescript
// Example: How to use services from previous phases
import { SomeService } from '../services/some-service';

const service = new SomeService();
await service.doSomething();
```

---

## MANDATORY: Test Before Commit

**READ:** `.coordination/COMMIT-WORKFLOW.md`

**10-Step Process - NO EXCEPTIONS:**

```
1. Implement → 2. Build → 3. Create tests → 4. Run tests (100%) →
5. Manual test → 6. Verify DB → 7. Security check → 8. Commit →
9. Push → 10. COMPLETED.md
```

---

## 🚨 CRITICAL: What You Must NEVER Do

### NEVER SKIP TESTING
Even if context is high, feeling rushed, or "almost done" - testing is MANDATORY.

### NEVER MERGE TO DEVELOP
**YOU DO NOT HAVE PERMISSION TO MERGE.**
- Do NOT run any merge commands
- Do NOT suggest "let's merge"

Your job: Create COMPLETED.md and WAIT for Master Orchestrator review.

### NEVER COMMIT WITHOUT 100% TESTS PASSING
- No "will test later"
- No "mostly passing"
- No shortcuts

---

## Phase X Mission

[Describe the goal of this phase - what problem it solves, what value it adds]

---

## Your Tasks (X Total)

### Task X.1: [Task Name]
**Description:** [What needs to be built]
**Acceptance Criteria:**
- [ ] Criterion 1
- [ ] Criterion 2

### Task X.2: [Task Name]
**Description:** [What needs to be built]
**Acceptance Criteria:**
- [ ] Criterion 1
- [ ] Criterion 2

[Continue for all tasks...]

---

## Key Integration Points

### Using [Service from Previous Phase]

```typescript
// Example code showing how to integrate
```

### Database Schema

```prisma
// Relevant schema for this phase
```

### API Endpoints Available

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/example` | GET | Does something |

---

## Key References

- `.coordination/SUBAGENT-GUIDELINES.md` - Development rules
- `.coordination/COMMIT-WORKFLOW.md` - Testing workflow
- `docs/PROGRESS.md` - Master dashboard
- [Link to relevant external docs]

---

## Communication

**Update these files in `.phase-status/`:**

| File | Purpose | When to Update |
|------|---------|----------------|
| `PROGRESS.md` | Track your work | Every significant milestone |
| `BLOCKERS.md` | Report obstacles | When stuck for >30 min |
| `QUESTIONS.md` | Ask for clarification | When requirements unclear |
| `COMPLETED.md` | Signal completion | When ALL tests pass |

**I'll check every 10-30 minutes.**

---

## Testing Specific to This Phase

**Build Command:** `npm run build`

**Test Command:** `npm test` or `npx ts-node test-phase-X.ts`

**Manual Testing:**
1. [Specific scenario to test]
2. [Another scenario]
3. [Edge case to verify]

**Database Verification:**
- [ ] Check [specific table/data]
- [ ] Verify [relationship/constraint]

---

## Notes & Tips

[Any specific guidance, warnings, or tips for this phase]

---

**Good luck! Remember: Quality > Speed. Test everything.**
