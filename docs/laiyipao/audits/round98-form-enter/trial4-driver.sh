#!/usr/bin/env bash
set -u
AB(){ agent-browser --session "$1" "${@:2}"; }
S=r98-t4
BASE=http://127.0.0.1:5174/#
SEL='input[aria-label="输入兑换码"]'
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round98-form-enter" || exit 1
AB $S close --all >/dev/null 2>&1
AB $S open >/dev/null 2>&1
AB $S set viewport 768 1024 >/dev/null 2>&1
AB $S navigate $BASE/pages/signin/signin >/dev/null 2>&1
sleep 3
echo -n '--- HOOK: '; AB $S eval --stdin < data/r98-hook.js
echo '--- BEFORE ---'; AB $S eval --stdin < data/r98-probe2.js
AB $S fill "$SEL" 'QA-CODE-001' >/dev/null 2>&1
echo -n '--- INPUT: '; AB $S eval --stdin < data/r98-inputval.js
AB $S press Enter >/dev/null 2>&1
sleep 2
echo '--- AFTER ENTER ---'; AB $S eval --stdin < data/r98-probe2.js
echo '=== DONE ==='
