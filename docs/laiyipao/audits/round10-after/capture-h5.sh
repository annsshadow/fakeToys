#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round10-after'
INIT="$D/auth-init.js"
DOM="$D/dom.js"
echo "=== PAGE skills ==="
"$AB" --session lyp-r10x-skills --init-script "$INIT" open "http://127.0.0.1:5174/#/pages/skills/skills" >/dev/null 2>&1
sleep 4
"$AB" --session lyp-r10x-skills get url 2>&1 | head -1
"$AB" --session lyp-r10x-skills eval --stdin < "$DOM" 2>&1 | head -1
"$AB" --session lyp-r10x-skills screenshot "$D/screenshots/r10-skills.png" 2>&1 | head -1
echo "=== PAGE equipment ==="
"$AB" --session lyp-r10x-equipment --init-script "$INIT" open "http://127.0.0.1:5174/#/pages/equipment/equipment" >/dev/null 2>&1
sleep 4
"$AB" --session lyp-r10x-equipment get url 2>&1 | head -1
"$AB" --session lyp-r10x-equipment eval --stdin < "$DOM" 2>&1 | head -1
"$AB" --session lyp-r10x-equipment screenshot "$D/screenshots/r10-equipment.png" 2>&1 | head -1
echo "=== PAGE gems ==="
"$AB" --session lyp-r10x-gems --init-script "$INIT" open "http://127.0.0.1:5174/#/pages/gems/gems" >/dev/null 2>&1
sleep 4
"$AB" --session lyp-r10x-gems get url 2>&1 | head -1
"$AB" --session lyp-r10x-gems eval --stdin < "$DOM" 2>&1 | head -1
"$AB" --session lyp-r10x-gems screenshot "$D/screenshots/r10-gems.png" 2>&1 | head -1
echo "=== PAGE bag ==="
"$AB" --session lyp-r10x-bag --init-script "$INIT" open "http://127.0.0.1:5174/#/pages/bag/bag" >/dev/null 2>&1
sleep 4
"$AB" --session lyp-r10x-bag get url 2>&1 | head -1
"$AB" --session lyp-r10x-bag eval --stdin < "$DOM" 2>&1 | head -1
"$AB" --session lyp-r10x-bag screenshot "$D/screenshots/r10-bag.png" 2>&1 | head -1
echo "=== PAGE battles ==="
"$AB" --session lyp-r10x-battles --init-script "$INIT" open "http://127.0.0.1:5174/#/pages/battles/battles" >/dev/null 2>&1
sleep 4
"$AB" --session lyp-r10x-battles get url 2>&1 | head -1
"$AB" --session lyp-r10x-battles eval --stdin < "$DOM" 2>&1 | head -1
"$AB" --session lyp-r10x-battles screenshot "$D/screenshots/r10-battles.png" 2>&1 | head -1
echo "=== PAGE battle ==="
"$AB" --session lyp-r10x-battle --init-script "$INIT" open "http://127.0.0.1:5174/#/pages/battle/battle?level=1" >/dev/null 2>&1
sleep 4
"$AB" --session lyp-r10x-battle get url 2>&1 | head -1
"$AB" --session lyp-r10x-battle eval --stdin < "$DOM" 2>&1 | head -1
"$AB" --session lyp-r10x-battle screenshot "$D/screenshots/r10-battle.png" 2>&1 | head -1