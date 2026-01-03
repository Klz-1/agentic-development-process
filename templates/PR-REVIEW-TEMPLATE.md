# PR Review Template

Use this template when presenting PR review recommendations to the user.

---

## PR Review: Phase [X] - [Title]

**PR:** #[NUMBER]
**Branch:** `feature/phase-X-name` → `develop`
**Author:** [Subagent/Phase Name]
**Created:** [Date]

---

### Recommendation: [✅ APPROVE / ❌ REJECT]

---

### Summary

[1-2 sentence summary of what this phase implements and its significance to the project]

---

### Quality Assessment

| Quality Gate | Status | Evidence |
|--------------|--------|----------|
| CI/CD Pipeline | ✅/❌ | [All checks passing / X checks failing] |
| Build | ✅/❌ | [0 errors / X errors] |
| Tests | ✅/❌ | [X/Y passing (100%) / X failing] |
| Code Review Score | ✅/❌ | [X/10 - meets/below threshold] |
| Security Scan | ✅/❌ | [No issues / X issues found] |
| Integration | ✅/❌ | [Verified with Phase X / Issues found] |
| Documentation | ✅/❌ | [Complete / Missing items] |

---

### Code Review Highlights

**Strengths:**
- [Positive aspect 1]
- [Positive aspect 2]

**Concerns:**
- [Issue 1 - severity]
- [Issue 2 - severity]

**Improvements (non-blocking):**
- [Suggestion 1]
- [Suggestion 2]

---

### Risk Assessment

**Risk Level:** 🟢 Low / 🟡 Medium / 🔴 High

**Rationale:**
[Explain why this risk level was assigned]

**Potential Impact:**
- [Impact area 1]
- [Impact area 2]

**Mitigation:**
- [How risks are addressed or can be monitored]

---

### Files Changed

| Category | Files | Lines |
|----------|-------|-------|
| Source | X files | +Y / -Z |
| Tests | X files | +Y / -Z |
| Config | X files | +Y / -Z |
| Docs | X files | +Y / -Z |
| **Total** | **X files** | **+Y / -Z** |

**Key files:**
- `path/to/important/file.ts` - [brief description]
- `path/to/another/file.ts` - [brief description]

---

### Test Evidence

**Automated Tests:**
```
[Test output summary or link to CI logs]
```

**Manual Testing:**
- [Scenario 1]: ✅ Verified
- [Scenario 2]: ✅ Verified

**Database Verification:**
- [What was checked]: ✅ Correct

---

### Integration Verification

| Dependency | Status | Notes |
|------------|--------|-------|
| Phase 1 | ✅ Compatible | [Details] |
| Phase 2 | ✅ Compatible | [Details] |
| External API | ✅ Working | [Details] |

---

### Checklist

- [ ] All quality gates pass
- [ ] No blocking issues identified
- [ ] Security review complete
- [ ] Integration verified
- [ ] Documentation complete
- [ ] Ready for production

---

## Action Required

**User: Please confirm merge approval**

| Action | Command |
|--------|---------|
| ✅ Approve | Reply "approve" or "merge" |
| ❌ Reject | Reply "reject" with feedback |
| ❓ Questions | Ask for clarification |

---

### Additional Context

[Any additional information the user should know before making a decision]

---

*Review conducted by Master Orchestrator on [Date/Time]*
