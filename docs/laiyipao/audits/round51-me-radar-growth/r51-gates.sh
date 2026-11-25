#!/usr/bin/env bash
set -uo pipefail
cd laiyipao/miniapp
echo ===FULL===
pnpm test -- --reporter=dot > /tmp/r51-full.log 2>&1
echo FULL_RC=$?
tail -8 /tmp/r51-full.log
echo ===TYPECHECK===
pnpm run type-check > /tmp/r51-tc.log 2>&1
echo TYPECHECK_RC=$?
tail -5 /tmp/r51-tc.log
echo ===BUILD===
pnpm run build:h5 > /tmp/r51-build.log 2>&1
echo BUILD_H5_RC=$?
tail -6 /tmp/r51-build.log
echo ===GITDIFF===
git diff --check > /tmp/r51-diffcheck.log 2>&1
echo GIT_DIFF_CHECK_RC=$?
tail -5 /tmp/r51-diffcheck.log
echo ===GO===
cd ../server
go test ./internal/domain -run "TestNoRealSecretsInRepo|TestNoInfraIdentifiersInRepo" -count=1 -v 2>&1 | tail -30
echo GO_RC=${PIPESTATUS[0]}
echo R51_GATES_DONE=YES
