# Stage 4 kickoff — BUILD (one slice)

Per slice, dispatch a builder (subagent with worktree isolation, a
`claude --worktree` session, or a cloud session). The prompt is short on
purpose — CLAUDE.md, FEATURES.md, and the hooks carry the process:

---

Build slice [N-name] from FEATURES.md on branch feat/[name].

Work feature by feature, test-first: write the Verify test, implement until
green, commit. The commit gate runs the suite — if it blocks you, fix the
failure, never bypass it. Do not edit FEATURES.md statuses. When every
feature in the slice passes locally, open a PR to main titled
"slice [N]: [name]" listing the feature IDs it claims; the evaluator
verifies from there.

If you hit genuine ambiguity (taste, architecture fork, missing credential),
ask me; otherwise decide, note it in the PR description, and keep moving.

---

Then dispatch the evaluator agent on the PR. For long unattended runs,
prefer a fresh builder session per slice over one marathon session — a new
session recovers full state from CLAUDE.md + FEATURES.md + git log.
