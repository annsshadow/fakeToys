#!/usr/bin/env bash
set -uo pipefail
ROOT="$(pwd)"
OUT="$ROOT/docs/laiyipao/audits/round88-trend-legend/data/go-out.txt"
: > "$OUT"
cd "$ROOT/laiyipao/server"
echo "=== GO NoInfra ===" >> "$OUT"
go test ./internal/domain/... -run TestNoInfraIdentifiersInRepo -count=1 >> "$OUT" 2>&1; echo "RC=$?" >> "$OUT"
echo "=== GO NoRealSecrets -v ===" >> "$OUT"
go test ./internal/domain/... -run TestNoRealSecretsInRepo -count=1 -v >> "$OUT" 2>&1; echo "RC=$?" >> "$OUT"
echo GO_DONE >> "$OUT"
cat "$OUT"
