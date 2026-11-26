#!/usr/bin/env bash
set -uo pipefail
M="laiyipao/miniapp"
cd "$M"
echo ===CONTRACT===
pnpm exec vitest run src/styles/r12GrowthPriority.contract.test.ts --reporter=dot 2>&1 | tail -12
echo CONTRACT_RC=${PIPESTATUS[0]}
echo ===FULL===
pnpm test -- --reporter=dot > /tmp/r48-full.log 2>&1
echo FULL_RC=$?
tail -8 /tmp/r48-full.log
echo ===TYPECHECK===
pnpm run type-check > /tmp/r48-tc.log 2>&1
echo TYPECHECK_RC=$?
tail -5 /tmp/r48-tc.log
echo ===BUILD===
pnpm run build:h5 > /tmp/r48-build.log 2>&1
echo BUILD_H5_RC=$?
tail -6 /tmp/r48-build.log
echo ===GO===
cd ../server
go test ./internal/domain -run "TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo" -count=1 -v 2>&1 | tail -30
echo GO_RC=${PIPESTATUS[0]}
echo R48_GATES_DONE=YES
