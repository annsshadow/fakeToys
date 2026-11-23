#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round12-after'
S=lyp-r12-detail100
echo CLICK=$("$AB" --session $S click '.battle-row.tap.lose' 2>&1 | head -1)
sleep 2
echo '=== DETAIL PROBE ==='
"$AB" --session $S eval --stdin < "$D/detail-probe.js" 2>&1 | head -1
"$AB" --session $S screenshot "$D/screenshots/r12-detail100-defeat-review.png" 2>&1 | tail -1
echo EXPAND_DONE