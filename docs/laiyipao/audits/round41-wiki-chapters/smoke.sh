#!/bin/bash
set -u
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-wiki-chapters
DATA=$D/data
AB=agent-browser
S=lyp-r41-smoke
timeout 20s $AB close --all >/dev/null 2>&1 || true
timeout 30s $AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
timeout 60s $AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/smoke-open.txt" 2>&1
timeout 30s $AB --session "$S" wait 1200 >/dev/null 2>&1
timeout 60s $AB --session "$S" eval "$(tr '\n' ' ' < "$D/chapter-probe.js")" > "$DATA/smoke-probe.txt" 2>&1
echo SMOKE_RC=$?
timeout 20s $AB --session "$S" close >/dev/null 2>&1 || true
