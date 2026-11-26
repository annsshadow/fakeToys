#!/usr/bin/env bash
set -uo pipefail
ROOT="$(cd /d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248 && pwd)"
cd "$ROOT/laiyipao/server"
echo GO_INFRA
go test ./internal/domain -run TestNoInfraIdentifiersInRepo -v 2>&1 | tail -12
echo GO_INFRA_RC=${PIPESTATUS[0]}
echo GO_SECRETS
go test ./internal/domain -run TestNoRealSecretsInRepo -v 2>&1 | tail -16
echo GO_SEC_RC=${PIPESTATUS[0]}
echo GO_DONE