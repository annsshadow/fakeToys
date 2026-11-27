#!/usr/bin/env bash
set -u
ROOT="$(pwd)"
OUT="$ROOT/docs/laiyipao/audits/round97-copy-failure-feedback/data/gates-out.txt"
cd "$ROOT/laiyipao/miniapp" || exit 1
{
 echo "=== G1 R97契约 ==="
 pnpm exec vitest run src/styles/r97CopyFailure.contract.test.ts --reporter=default 2>&1; echo "G1_RC=$?"
 echo "=== G2 定向 me.test + battles.test ==="
 pnpm exec vitest run src/pages/me/me.test.ts src/pages/battles/battles.test.ts --reporter=default 2>&1; echo "G2_RC=$?"
 echo "=== G3 全量vitest ==="
 pnpm exec vitest run --reporter=default 2>&1 | tail -12; echo "G3_RC=${PIPESTATUS[0]}"
 echo "=== G4 vue-tsc ==="
 pnpm run type-check 2>&1 | tail -8; echo "G4_RC=${PIPESTATUS[0]}"
 echo "=== G5 build:h5 ==="
 pnpm run build:h5 2>&1 | tail -10; echo "G5_RC=${PIPESTATUS[0]}"
 cd "$ROOT" || exit 1
 echo "=== G6 git diff --check ==="
 git diff --check 2>&1; echo "G6_RC=$?"
 echo GATES_ALL_DONE
} > "$OUT" 2>&1
echo GATES_DONE
