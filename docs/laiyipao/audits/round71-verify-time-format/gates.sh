#!/usr/bin/env bash
set -uo pipefail
ROOT="$(cd /d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248 && pwd)"
cd "$ROOT/laiyipao/miniapp"
echo G1_CONTRACT
pnpm exec vitest run src/styles/r71VerifyTimeFormat.contract.test.ts --reporter=verbose 2>&1 | tail -12
echo G1_RC=${PIPESTATUS[0]}
echo G2_FULL
pnpm exec vitest run --reporter=verbose 2>&1 | tail -8
echo G2_RC=${PIPESTATUS[0]}
echo G3_TSC
pnpm exec vue-tsc --noEmit 2>&1 | tail -6
echo G3_RC=${PIPESTATUS[0]}
echo G4_BUILD
pnpm run build:h5 2>&1 | tail -8
echo G4_RC=${PIPESTATUS[0]}
cd "$ROOT"
echo G5_DIFF
git diff --check 2>&1 | tail -6
echo G5_RC=${PIPESTATUS[0]}
echo GATES_DONE