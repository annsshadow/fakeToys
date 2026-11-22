#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10b2
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/battle/battle" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 2
"$AB" --session "$S" eval "document.body.innerText.slice(0,2500)" > docs/laiyipao/audits/round10-causality/battle-probe.txt 2>&1
echo DONE