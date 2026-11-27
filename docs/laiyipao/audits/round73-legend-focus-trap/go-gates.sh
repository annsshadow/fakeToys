#!/bin/bash
set -uo pipefail
ROOT="$(pwd)"
OUT="$ROOT/docs/laiyipao/audits/round73-legend-focus-trap/data/go-gates-out.txt"
: > "$OUT"
cd "$ROOT/laiyipao/server"
echo "===== GO TestNoInfra =====" >> "$OUT"
go test ./internal/... -run TestNoInfraIdentifiersInRepo -count=1 -v >> "$OUT" 2>&1; echo "GO1_RC=$?" >> "$OUT"
echo "===== GO TestNoRealSecrets =====" >> "$OUT"
go test ./internal/... -run TestNoRealSecretsInRepo -count=1 -v >> "$OUT" 2>&1; echo "GO2_RC=$?" >> "$OUT"
echo GO_GATES_DONE >> "$OUT"
