#!/usr/bin/env bash
REPO=$(cd "$(dirname "$0")/../../../../" && pwd)
cd "$REPO/laiyipao/admin" || exit 1
echo "===GATES_START CWD=$(pwd)==="
echo "===G1_R94==="
pnpm exec vitest run tests/R94DefenseEmptyCta.contract.test.ts --reporter=dot; echo "G1_RC=$?"
echo "===G2_r5Mapping==="
pnpm exec vitest run tests/r5Mapping.contract.test.ts --reporter=dot; echo "G2_RC=$?"
echo "===G3_PageSkeleton==="
pnpm exec vitest run tests/PageSkeleton.contract.test.ts --reporter=dot; echo "G3_RC=$?"
echo "===G4_FULL==="
pnpm exec vitest run --reporter=dot; echo "G4_RC=$?"
echo "===G5_vue-tsc==="
pnpm run type-check; echo "G5_RC=$?"
echo "===G6_build==="
pnpm run build:only; echo "G6_RC=$?"
cd "$REPO" || exit 1
echo "===G7_diff-check==="
git diff --check; echo "G7_RC=$?"
echo "===GATES_DONE==="