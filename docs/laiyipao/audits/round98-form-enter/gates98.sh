#!/usr/bin/env bash
set -u
ROOT="D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248"
MINI="$ROOT/laiyipao/miniapp"
OUT="$ROOT/docs/laiyipao/audits/round98-form-enter/data/gates-out.txt"
> "$OUT"
cd "$MINI" || exit 1
echo '=== G1 CONTRACT r98 ===' | tee -a "$OUT"
pnpm exec vitest run src/styles/r98FormEnter.contract.test.ts --reporter=default 2>&1 | tee -a "$OUT"; echo "G1_RC=${PIPESTATUS[0]}" | tee -a "$OUT"
echo '=== G2 DIRECTED me/login/r69/r61/r55 ===' | tee -a "$OUT"
pnpm exec vitest run src/pages/me/me.test.ts src/pages/login/login.test.ts src/styles/r69*.contract.test.ts src/styles/r61*.contract.test.ts src/styles/r55*.contract.test.ts --reporter=default 2>&1 | tail -30 | tee -a "$OUT"; echo "G2_RC=${PIPESTATUS[0]}" | tee -a "$OUT"
echo '=== G3 FULL MINIAPP ===' | tee -a "$OUT"
pnpm exec vitest run --reporter=default 2>&1 | tail -25 | tee -a "$OUT"; echo "G3_RC=${PIPESTATUS[0]}" | tee -a "$OUT"
echo '=== G4 VUE-TSC ===' | tee -a "$OUT"
pnpm run type-check 2>&1 | tee -a "$OUT"; echo "G4_RC=${PIPESTATUS[0]}" | tee -a "$OUT"
echo '=== G5 BUILD H5 ===' | tee -a "$OUT"
pnpm run build:h5 2>&1 | tail -18 | tee -a "$OUT"; echo "G5_RC=${PIPESTATUS[0]}" | tee -a "$OUT"
cd "$ROOT" || exit 1
echo '=== G6 DIFF CHECK ===' | tee -a "$OUT"
git diff --check 2>&1 | tee -a "$OUT"; echo "G6_RC=${PIPESTATUS[0]}" | tee -a "$OUT"
echo '=== GATES_DONE ===' | tee -a "$OUT"
