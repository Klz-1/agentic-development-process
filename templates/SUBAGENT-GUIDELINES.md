# Subagent Development Guidelines

**Last Updated:** [Date]

---

## 🎯 Core Principle

**TEST LOCALLY BEFORE COMMITTING**

This is MANDATORY and NON-NEGOTIABLE.

---

## 🚨 CRITICAL: What You Must NEVER Do

### NEVER SKIP TESTING
Even if context is high, feeling rushed, or "almost done" - testing is MANDATORY.

### NEVER MERGE TO DEVELOP
**YOU DO NOT HAVE PERMISSION TO MERGE.**
- Do NOT run any merge commands
- Do NOT suggest "let's merge"
- Do NOT say "ready to merge to develop"

Your job: Create COMPLETED.md and WAIT for Master Orchestrator review.

### NEVER COMMIT WITHOUT 100% TESTS PASSING
- No "will test later"
- No "mostly passing"
- No shortcuts

**Context Pressure:** If you feel rushed, acknowledge it but follow the process anyway.

---

## 📋 Mandatory Development Workflow (10 Steps)

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

---

## ❌ What NOT to Do

**NEVER:**
- Commit without testing locally
- Commit code that doesn't build
- Commit failing tests
- Commit API keys or secrets
- Push broken code
- Skip manual testing
- Say "mostly working" or "good enough"

---

## 🧪 Testing Requirements

**Create:**
- `test-your-feature.ts` (automated tests)
- `TESTING_GUIDE.md` (how to test)
- Mock clients if using external APIs

**Run:**
- All tests must pass 100%
- Manual testing required
- Database verification required

---

## 🔒 Security Checklist

**Before committing:**
```bash
# Check for secrets
grep -r "API_KEY" . --include="*.ts"
grep -r "SECRET" . --include="*.ts"
grep -r "PASSWORD" . --include="*.ts"

# Review staged changes
git diff --cached

# No secrets? Good to commit.
```

---

## ✅ Quality Gates (All Must Pass)

Before commit, ALL must be checked:

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

**All 10 gates must pass = Ready to commit**

---

## 📝 Commit Message Format

```
feat(phase-X): brief description of what was built

Detailed description of changes if needed.

Testing:
- Build: ✅ Passing
- Tests: ✅ X/X passing (100%)
- Manual: ✅ Verified [specific scenarios]
- Database: ✅ [what you verified]
- Integration: ✅ Phase Y services working
```

---

## 🆘 Communication

**Update these files in `.phase-status/`:**

| File | When |
|------|------|
| `PROGRESS.md` | Regularly as you work |
| `BLOCKERS.md` | When you're stuck |
| `QUESTIONS.md` | When you need clarification |
| `COMPLETED.md` | When all tests pass |

**Master Orchestrator checks every 10-30 minutes.**

---

## If You Feel Rushed

**STOP. Do NOT skip steps.**

1. Acknowledge: "I'm feeling context pressure, but testing is mandatory."
2. Follow process anyway (all 10 steps)
3. Create COMPLETED.md and wait
4. Trust Master Orchestrator for coordination

Remember:
- Context pressure is NOT a reason to skip testing
- Master manages timeline, not you
- Quality > Speed (always)
