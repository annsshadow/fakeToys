#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round11-mobile-audit'
"$AB" --session lyp-m11-test set viewport 390 844 2>&1 | head -1
"$AB" --session lyp-m11-test --init-script "$D/auth-init.js" open 'http://127.0.0.1:5174/#/pages/battle/battle?level=1' >/dev/null 2>&1
sleep 4
echo '=== PROBE ==='
"$AB" --session lyp-m11-test eval --stdin < "$D/mobile-probe.js" 2>&1 | head -1
"$AB" --session lyp-m11-test screenshot "$D/screenshots/04-battle-TEST.png" 2>&1 | head -1