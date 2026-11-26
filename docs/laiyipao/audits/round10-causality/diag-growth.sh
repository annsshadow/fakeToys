#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10final
D=docs/laiyipao/audits/round10-causality
ev() { "$AB" --session "$S" eval "$1" 2>/dev/null; }
# ensure on battles page with 2 battles
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/battles/battles" >/dev/null 2>&1
sleep 3
echo "BATTLES_COUNT_LINE=$(ev "document.body.innerText.split('\n').filter(l=>l.includes('场')&&l.includes('共')).join(';')")"
echo "ROW_COUNT=$(ev "document.querySelectorAll('.battle-row').length")"
# expand first row via class selector
ev "document.querySelectorAll('.battle-row')[0]?.click()" >/dev/null 2>&1
sleep 5
echo "EXPANDED=$(ev "document.querySelectorAll('.battle-row.expanded').length")"
echo "DETAIL_GROWTH_CNT=$(ev "document.querySelectorAll('.r10-growth').length")"
ev "(()=>{const e=document.querySelector('.r10-growth');return e?e.innerText.slice(0,800):'NONE'})()" > "$D/growth-class.txt" 2>&1
echo "GROWTH_TEXT:"; cat "$D/growth-class.txt"
echo "HAS_SETtle_TEXT=$(ev "document.body.innerText.includes('可信差值')")"
"$AB" --session "$S" screenshot "$D/screenshots/after-09-battles-growth.png" >/dev/null 2>&1
echo DIAG_DONE