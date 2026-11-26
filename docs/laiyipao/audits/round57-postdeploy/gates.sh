#!/usr/bin/env bash
set -uo pipefail
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 9
echo "=== Go门禁 ==="
cd laiyipao/server
go test ./internal/domain -run "TestNoInfraIdentifiersInRepo|TestNoRealSecretsInRepo" -v 2>&1 | tail -40
echo "GATES_EXIT=${PIPESTATUS[0]}"