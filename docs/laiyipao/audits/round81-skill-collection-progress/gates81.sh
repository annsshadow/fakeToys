#!/usr/bin/env bash
set -uo pipefail
ROOT="$(pwd)"
OUT="$ROOT/docs/laiyipao/audits/round81-skill-collection-progress/data/gates-out.txt"
: > "$OUT"
run(){ echo "=== $1 ===" >> "$OUT"; shift; "$@" >> "$OUT" 2>&1; echo "RC=$?" >> "$OUT"; }
cd "$ROOT/laiyipao/miniapp"
run G1 pnpm exec vitest run src/styles/r81SkillCollectionProgress.contract.test.ts --reporter=verbose
run G2 pnpm exec vitest run --reporter=verbose
run G3 pnpm exec vue-tsc --noEmit
run G4 pnpm run build:h5
cd "$ROOT"
run G5 git diff --check
echo GATES_DONE >> "$OUT"
cat "$OUT"