#!/bin/bash
cd "$(git rev-parse --show-toplevel)/laiyipao/admin" || exit 1
echo "=== R35定向 ==="
pnpm exec vitest run tests/WikiGemMechanics.test.ts --reporter=dot 2>&1; echo R35_RC=$?
echo "=== 前序R31-R34+R35 ==="
pnpm exec vitest run tests/WikiGemMechanics.test.ts tests/WikiEnemyMechanics.test.ts tests/WikiReactionMechanics.test.ts tests/WikiSkillMechanics.test.ts tests/WikiView.test.ts --reporter=dot 2>&1; echo PREFIX_RC=$?
echo "=== 全量 ==="
pnpm test -- --testTimeout=90000 2>&1 | tail -25; echo FULL_RC=${PIPESTATUS[0]}
echo "=== type-check ==="
pnpm type-check 2>&1; echo TC_RC=$?
echo "=== build ==="
pnpm build 2>&1 | tail -10; echo BUILD_RC=${PIPESTATUS[0]}
echo TESTS_DONE=YES
