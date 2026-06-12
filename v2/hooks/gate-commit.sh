#!/usr/bin/env bash
# PreToolUse gate for Bash commands.
# Blocks `git commit` unless the test suite passes and the staged diff is
# free of obvious secrets. Exit 2 = block the tool call; stderr is fed back
# to the agent so it can fix the failure and retry.
#
# Adapt TEST_CMD to your project. Keep it the same command CI runs.
set -u
TEST_CMD="${GATE_TEST_CMD:-npm test}"

input=$(cat)
cmd=$(printf '%s' "$input" | jq -r '.tool_input.command // empty')

# Only gate commit commands; let everything else through untouched.
case "$cmd" in
  *"git commit"*) ;;
  *) exit 0 ;;
esac

# Gate 1: test suite must pass.
log=$(mktemp)
if ! $TEST_CMD >"$log" 2>&1; then
  {
    echo "COMMIT BLOCKED: test suite failed. Fix the failures, then commit again."
    echo "--- last 30 lines of test output ---"
    tail -30 "$log"
  } >&2
  rm -f "$log"
  exit 2
fi
rm -f "$log"

# Gate 2: no plausible secrets in the staged diff (added lines only).
if git diff --cached --unified=0 | grep -E '^\+' \
   | grep -Ei '(api[_-]?key|secret|passwd|password|private[_-]?key|token)["'"'"']?\s*[:=]\s*["'"'"'][A-Za-z0-9+/_-]{16,}'; then
  echo "COMMIT BLOCKED: the staged diff appears to contain a secret (line(s) above). Move it to an environment variable and recommit." >&2
  exit 2
fi

exit 0
