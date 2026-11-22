#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='docs/laiyipao/audits/round8-after/'; SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round8-after/screenshots'
echo '=== FRESH L1 ==='
"$AB" --session lyp-r8-l1fresh --init-script "$D/probe-init.js" open 'http://127.0.0.1:5174/#/pages/stage/stage?level=1' >/dev/null 2>&1; sleep 4
"$AB" --session lyp-r8-l1fresh set viewport 390 844 >/dev/null 2>&1
echo "L1: $("$AB" --session lyp-r8-l1fresh eval --stdin < "$D/stage-probe.js" 2>&1 | head -1)"
"$AB" --session lyp-r8-l1fresh screenshot "$SHOT/r8-L1-stage-fresh.png" >/dev/null 2>&1; echo L1_SHOT_DONE
echo '=== FRESH L100 ==='
"$AB" --session lyp-r8-l100fresh --init-script "$D/probe-init.js" open 'http://127.0.0.1:5174/#/pages/stage/stage?level=100' >/dev/null 2>&1; sleep 4
"$AB" --session lyp-r8-l100fresh set viewport 390 844 >/dev/null 2>&1
echo "L100: $("$AB" --session lyp-r8-l100fresh eval --stdin < "$D/stage-probe.js" 2>&1 | head -1)"
"$AB" --session lyp-r8-l100fresh screenshot "$SHOT/r8-L100-boss-fresh.png" >/dev/null 2>&1; echo L100_SHOT_DONE
echo '=== HOME HONEST ==='
echo "HOME: $("$AB" --session lyp-r8-home eval --stdin < "$D/honest-probe.js" 2>&1 | head -1)"
echo RECAP_DONE