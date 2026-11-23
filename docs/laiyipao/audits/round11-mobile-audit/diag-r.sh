#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round11-mobile-audit'
"$AB" --session lyp-m11diag-r open 'about:blank' >/dev/null 2>&1
"$AB" --session lyp-m11diag-r set viewport 390 844 >/dev/null 2>&1
"$AB" --session lyp-m11diag-r --init-script "$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/reactions/reactions" >/dev/null 2>&1
sleep 3
echo '=== 07-reactions DIAG ==='
"$AB" --session lyp-m11diag-r eval --stdin < "$D/diag-vertical.js" 2>&1 | head -1
"$AB" --session lyp-m11diag-r screenshot "$D/screenshots/07-reactions-vertical.png" 2>&1 | head -1