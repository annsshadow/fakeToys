#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10final
D=docs/laiyipao/audits/round10-causality
ev() { "$AB" --session "$S" eval "$1" 2>/dev/null; }
# re-ensure expanded (same session still on battles page)
echo "EXPANDED=$(ev "document.querySelectorAll('.battle-row.expanded').length")"
"$AB" --session "$S" scrollintoview ".r10-growth" >/dev/null 2>&1
sleep 1
echo "GROWTH_TEXT=$(ev "(()=>{const e=document.querySelector('.r10-growth');return e?e.innerText.slice(0,400):'NONE'})()")"
"$AB" --session "$S" screenshot "$D/screenshots/after-09-battles-growth.png" >/dev/null 2>&1
echo RESHOT_DONE