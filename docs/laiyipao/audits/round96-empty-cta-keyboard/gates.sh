#!/usr/bin/env bash
set -u
ROOT="$(pwd)"
OUT="$ROOT/docs/laiyipao/audits/round96-empty-cta-keyboard/data/gates-out.txt"
cd "$ROOT/laiyipao/miniapp" || exit 1
{
 echo "=== G1 R96契约 ==="
 pnpm exec vitest run src/styles/r96EmptyCtaKeyboard.contract.test.ts --reporter=default 2>&1; echo "G1_RC=$?"
 echo "=== G2 me.test ==="
 pnpm exec vitest run src/pages/me/me.test.ts --reporter=default 2>&1; echo "G2_RC=$?"
 echo "=== G3 R60 a11y范式抽查(announcements/equipment/skills模板契约) ==="
 pnpm exec vitest run src/styles/r60*.contract.test.ts --reporter=default 2>&1; echo "G3_RC=$?"
 echo "=== G4 R95 backKeyboard契约 ==="
 pnpm exec vitest run src/styles/r95BackKeyboard.contract.test.ts --reporter=default 2>&1; echo "G4_RC=$?"
 echo "=== G5 全量vitest ==="
 pnpm exec vitest run --reporter=default 2>&1 | tail -15; echo "G5_RC=${PIPESTATUS[0]}"
 echo "=== G6 vue-tsc ==="
 pnpm run type-check 2>&1 | tail -8; echo "G6_RC=${PIPESTATUS[0]}"
 echo "=== G7 build:h5 ==="
 pnpm run build:h5 2>&1 | tail -8; echo "G7_RC=${PIPESTATUS[0]}"
 cd "$ROOT" || exit 1
 echo "=== G8 git diff --check ==="
 git diff --check 2>&1; echo "G8_RC=$?"
 echo GATES_ALL_DONE
} > "$OUT" 2>&1
echo GATES_DONE