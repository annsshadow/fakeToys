#!/usr/bin/env bash
set -u
ROOT=$(pwd)
echo '=== G1 CONTRACT ==='
cd "$ROOT/laiyipao/miniapp" && pnpm exec vitest run src/styles/r72SearchEmpty.contract.test.ts --reporter=verbose
echo '=== G2 FULL VITEST ==='
cd "$ROOT/laiyipao/miniapp" && pnpm exec vitest run --reporter=verbose
echo '=== G3 VUE-TSC ==='
cd "$ROOT/laiyipao/miniapp" && pnpm exec vue-tsc --noEmit
echo '=== G4 BUILD:H5 ==='
cd "$ROOT/laiyipao/miniapp" && pnpm run build:h5
echo '=== G5 DIFF CHECK ==='
cd "$ROOT" && git diff --check
echo GATES_DONE