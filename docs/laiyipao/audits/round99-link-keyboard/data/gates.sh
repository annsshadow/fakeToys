#!/bin/bash
ROOT="$(pwd)"
OUT="$ROOT/docs/laiyipao/audits/round99-link-keyboard/data/gates-out.txt"
: > "$OUT"
cd "$ROOT/laiyipao/miniapp" || exit 1
run(){ echo "=== $1 ===" >> "$OUT"; shift; "$@" >> "$OUT" 2>&1; echo "RC=$?" >> "$OUT"; }
run G1 pnpm exec vitest run src/styles/r99LinkKeyboard.contract.test.ts
run G2 pnpm exec vitest run src/styles/r60A11yKeyboard.contract.test.ts
run G3 pnpm exec vitest run src/pages/me/me.test.ts src/pages/announcements/announcements.test.ts
run G4 pnpm exec vitest run
run G5 pnpm run type-check
run G6 pnpm run build:h5
cd "$ROOT" || exit 1
run G7 git diff --check
echo GATES_DONE >> "$OUT"
