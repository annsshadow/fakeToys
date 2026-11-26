#!/usr/bin/env bash
set -uo pipefail
ROOT="$(pwd)"
cd "$ROOT/laiyipao/server"
echo "=== GO TESTNoInfra ==="
go test ./internal/domain -run TestNoInfraIdentifiersInRepo -v 2>&1 | tail -20; echo "INFRA_RC=${PIPESTATUS[0]}"
echo "=== GO TESTNoRealSecrets ==="
go test ./internal/domain -run TestNoRealSecretsInRepo -v 2>&1 | tail -20; echo "SECRET_RC=${PIPESTATUS[0]}"
echo GO_DONE
