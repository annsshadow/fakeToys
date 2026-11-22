#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10final
D=docs/laiyipao/audits/round10-causality
ev() { "$AB" --session "$S" eval "$1" 2>/dev/null; }
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/battles/battles" >/dev/null 2>&1
sleep 3
"$AB" --session "$S" scrollintoview ".battle-row" >/dev/null 2>&1
sleep 1
"$AB" --session "$S" click ".battle-row" >/dev/null 2>&1
sleep 5
echo "EXPANDED=$(ev "document.querySelectorAll('.battle-row.expanded').length")"
echo "GROWTH_CNT=$(ev "document.querySelectorAll('.r10-growth').length")"
ev "(()=>{const e=document.querySelector('.r10-growth');return e?e.innerText.slice(0,900):'NONE'})()" > "$D/growth-native.txt" 2>&1
echo "GROWTH:"; cat "$D/growth-native.txt"
ev "document.body.innerText.includes('可信差值')" > "$D/growth-visible.txt" 2>&1
echo "VISIBLE:"; cat "$D/growth-visible.txt"
"$AB" --session "$S" screenshot "$D/screenshots/after-09-battles-growth.png" >/dev/null 2>&1
echo NATIVE_CLICK_DONE