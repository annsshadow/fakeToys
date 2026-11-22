#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round10-after'
echo '=== LEGEND PE ==='
"$AB" --session lyp-r10-legend eval --stdin < "$D/js-pe.js" 2>&1 | head -1
echo '=== SKILLS BTN ==='
"$AB" --session lyp-r10x-skills eval --stdin < "$D/js-btn.js" 2>&1 | head -1
echo '=== CLICK COMPARE ==='
"$AB" --session lyp-r10x-skills click '.r7-impact .r7-btn.primary' 2>&1 | head -1
sleep 1
"$AB" --session lyp-r10x-skills eval --stdin < "$D/js-modal.js" 2>&1 | head -1
"$AB" --session lyp-r10x-skills screenshot "$D/screenshots/r10-skills-compare.png" 2>&1 | head -1