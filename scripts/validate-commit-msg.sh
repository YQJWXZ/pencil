#!/usr/bin/env bash
set -euo pipefail

# Validate Conventional Commits format
# Usage: validate-commit-msg.sh <commit-msg-file>

if [[ $# -eq 0 ]]; then
    echo "Usage: $0 <commit-msg-file>"
    exit 1
fi

msg=$(cat "$1")

# Check format: <type>(<scope>)?: <subject>
if ! echo "$msg" | grep -qE "^(feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert)(\(.+\))?!?: .+"; then
    echo "❌ Invalid commit message format!"
    echo ""
    echo "Expected: <type>(<scope>): <subject>"
    echo "Example: feat(api): add user authentication"
    echo ""
    echo "Valid types: feat, fix, docs, style, refactor, test, chore, perf, ci, build, revert"
    exit 1
fi
