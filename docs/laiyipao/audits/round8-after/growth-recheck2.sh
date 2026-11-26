#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r8-growth; D='docs/laiyipao/audits/round8-after/'; SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round8-after/screenshots'
run(){ "$AB" --session "$S" "$@" 2>&1; }
j(){ "$AB" --session "$S" eval --stdin < "$D/$1" 2>&1|head -1; }
echo '=== LOGIN ==='
run --init-script "$D/probe-init.js" open 'http://127.0.0.1:5174/#/pages/login/login' >/dev/null 2>&1; sleep 4
run set viewport 390 844 >/dev/null 2>&1
j clear-token.js >/dev/null 2>&1
run fill "input[type='text']" 'admin' >/dev/null 2>&1
run fill "input[type='password']" 'admin12345' >/dev/null 2>&1
run click '.btn-primary' >/dev/null 2>&1; sleep 6
echo "$(j token-len.js)"
echo '=== SKILLS LOGGED ==='
j reset-probe.js >/dev/null 2>&1
run open 'http://127.0.0.1:5174/#/pages/skills/skills' >/dev/null 2>&1; sleep 5
echo "SKILLS: $(j growth-probe.js)"
echo "AF: $(j af-dump.js)"
run screenshot "$SHOT/r8-growth-skills-logged.png" >/dev/null 2>&1; echo SHOT_DONE