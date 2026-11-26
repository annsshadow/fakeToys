#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r8-l1; D='docs/laiyipao/audits/round8-after/'; SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round8-after/screenshots'
run(){ "$AB" --session "$S" "$@" 2>&1; }
j(){ "$AB" --session "$S" eval --stdin < "$D/$1" 2>&1 | head -1; }
echo '=== ENTER BATTLE ==='
run click '.start-btn' >/dev/null; sleep 6
echo "URL: $(run get url | head -2 | tail -1)"
echo "BATTLE_T0: $(j battle-probe.js)"
run screenshot "$SHOT/r8-battle-clamp-hud.png" >/dev/null; echo SHOT1_DONE
sleep 8
echo "BATTLE_T8: $(j battle-probe.js)"
echo BATTLE_DONE