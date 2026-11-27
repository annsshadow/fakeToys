#!/usr/bin/env bash
set -uo pipefail
ROOT="$(pwd)"
OUT="$ROOT/docs/laiyipao/audits/round88-trend-legend/data/gates-out.txt"
: > "$OUT"
run(){ echo "=== $1 ===" >> "$OUT"; shift; "$@" >> "$OUT" 2>&1; echo "RC=$?" >> "$OUT"; }
cd "$ROOT/laiyipao/miniapp"
run G1_R88_CONTRACT pnpm exec vitest run src/styles/r88TrendLegend.contract.test.ts --reporter=verbose
run G2_FULL_VITEST pnpm exec vitest run --reporter=verbose
run G3_VUE_TSC pnpm exec vue-tsc --noEmit
run G4_BUILD_H5 pnpm run build:h5
cd "$ROOT"
run G5_DIFF_CHECK git diff --check
echo GATES88_DONE >> "$OUT"
grep -E "===|RC=|Test Files|Tests |DONE|error|Error" "$OUT"
