#!/usr/bin/env bash
set -u
AB(){ agent-browser --session "$1" "${@:2}"; }
BASE="http://127.0.0.1:5174/#"
ROOT="D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round98-form-enter"
cd "$ROOT" || exit 1
S=r98-dir
AB $S close --all >/dev/null 2>&1; AB $S open >/dev/null 2>&1
AB $S set viewport 768 1024 >/dev/null 2>&1
echo '===== BAG CLICK POSITIVE ====='
AB $S navigate "$BASE/pages/bag/bag" >/dev/null 2>&1; sleep 3
echo -n 'HOOK:'; AB $S eval --stdin < data/r98-hook2.js
AB $S fill 'input[aria-label="预设名（最多 20 字）"]' 'QA点击预设' >/dev/null 2>&1
echo -n 'CLEAR:'; AB $S eval --stdin < data/r98-clear.js
echo -n 'CLICK:'; AB $S eval --stdin < data/click-bag.js
sleep 2; echo 'AFTER_CLICK:'; AB $S eval --stdin < data/r98-probe2.js
echo '===== LOGIN ENTER PARADIGM ====='
AB $S navigate "$BASE/pages/login/login" >/dev/null 2>&1; sleep 3
echo -n 'HOOK:'; AB $S eval --stdin < data/r98-hook2.js
echo -n 'LOGIN_INPUT_DUMP:'; AB $S eval --stdin < data/r98-dumpinput.js
echo '=== DIR_DONE ==='
