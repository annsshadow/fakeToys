#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round11-after'
S=lyp-r11-admdiag
echo -n 'VP: '; "$AB" --session $S eval 'window.innerWidth+"x"+window.innerHeight' 2>&1 | tail -1
"$AB" --session $S --init-script "$D/probe-init.js" open 'http://127.0.0.1:5175/laiyipao-admin/#/admin/dashboard' >/dev/null 2>&1
sleep 4
echo '=== ADMIN DASHBOARD ==='
"$AB" --session $S eval --stdin < "$D/admin-probe.js" 2>&1 | head -1
"$AB" --session $S screenshot "$D/screenshots/r11-admin-dashboard.png" 2>&1 | head -1
echo ADMIN_DONE