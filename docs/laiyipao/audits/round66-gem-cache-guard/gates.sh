#!/usr/bin/env bash
set -uo pipefail
ROOT="$(pwd)"
M="$ROOT/laiyipao/miniapp"
echo "=== R66 CONTRACT ==="
cd "$M" && pnpm exec vitest run src/styles/r66GemCacheGuard.contract.test.ts --reporter=verbose 2>&1; echo "CONTRACT_RC=$?"
echo "=== R10 CONTRACT ==="
cd "$M" && pnpm exec vitest run src/styles/r10GrowthGaps.contract.test.ts --reporter=verbose 2>&1 | tail -12; echo "R10_RC=${PIPESTATUS[0]}"
echo "=== FULL VITEST ==="
cd "$M" && pnpm exec vitest run --reporter=verbose 2>&1 | tail -20; echo "FULL_RC=${PIPESTATUS[0]}"
echo "=== VUE-TSC ==="
cd "$M" && pnpm exec vue-tsc --noEmit 2>&1; echo "TSC_RC=$?"
echo "=== DIFF CHECK ==="
cd "$ROOT" && git diff --check 2>&1; echo "DIFF_RC=$?"
echo ALL_GATES_DONE
