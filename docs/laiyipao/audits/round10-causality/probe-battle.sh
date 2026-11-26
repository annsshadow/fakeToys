#!/usr/bin/env bash
set -u
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10battle
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/battle/battle" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 2
echo "=== BUTTONS ==="
"$AB" --session "$S" eval --json --stdin <<"EVAL"
JSON.stringify([...document.querySelectorAll("view,text,button")].filter(e=>/开始|出战|进入|战斗|速|倍/.test(e.textContent||"")&&e.children.length===0).slice(0,30).map(e=>({t:e.textContent.trim().slice(0,40),cls:e.className})))
EVAL
echo PROBE_DONE