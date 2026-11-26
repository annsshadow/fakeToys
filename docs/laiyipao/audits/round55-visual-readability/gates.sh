#!/usr/bin/env bash
set -uo pipefail
cd D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248
echo GATE_START
cd laiyipao/miniapp && pnpm exec vitest run src/styles/r55VisualReadability.contract.test.ts 2>&1 | tail -n 12; echo CONTRACT_RC=${PIPESTATUS[0]}
cd D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp && pnpm test 2>&1 | tail -n 12; echo FULL_RC=${PIPESTATUS[0]}
cd D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp && pnpm run type-check 2>&1 | tail -n 8; echo TYPECHECK_RC=${PIPESTATUS[0]}
cd D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/miniapp && pnpm run build:h5 2>&1 | tail -n 8; echo BUILD_RC=${PIPESTATUS[0]}
cd D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248 && git diff --check 2>&1 | tail -n 8; echo DIFFCHECK_RC=${PIPESTATUS[0]}
cd D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/server && export DATABASE_URL="postgres://postgres@127.0.0.1:15440/laiyipao?sslmode=disable" && go test ./internal/domain -run 'TestNoInfraIdentifiersInRepo|TestNoRealSecretsInRepo' -v 2>&1 | tail -n 30; echo GO_RC=${PIPESTATUS[0]}
echo GATE_FINISH
