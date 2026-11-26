#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r10settle
"$AB" --session "$S" eval "document.body.innerText.slice(0,1800)" > docs/laiyipao/audits/round10-causality/settle-state.txt 2>&1
echo STATE_DONE