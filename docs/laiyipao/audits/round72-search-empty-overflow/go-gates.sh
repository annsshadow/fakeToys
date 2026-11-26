#!/usr/bin/env bash
cd laiyipao/server
echo "=== GO TEST NO INFRA ==="
go test ./internal/domain -run TestNoInfraIdentifiersInRepo -count=1 -v
echo "=== GO TEST NO REAL SECRETS ==="
go test ./internal/domain -run TestNoRealSecretsInRepo -count=1 -v
echo GO_DONE