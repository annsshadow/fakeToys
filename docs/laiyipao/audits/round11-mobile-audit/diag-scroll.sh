#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round11-mobile-audit'
echo '=== REACTIONS VERT ==='
"$AB" --session lyp-m11v2-07 eval --stdin < "$D/diag-vertical.js" 2>&1 | head -1
sleep 1
"$AB" --session lyp-m11v2-07 screenshot "$D/screenshots/07-reactions-vertical.png" 2>&1 | head -1
echo '=== ENEMIES VERT ==='
"$AB" --session lyp-m11v2-08 eval --stdin < "$D/diag-vertical.js" 2>&1 | head -1
sleep 1
"$AB" --session lyp-m11v2-08 screenshot "$D/screenshots/08-enemies-vertical.png" 2>&1 | head -1