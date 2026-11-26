#!/usr/bin/env bash
set -u
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
SESSION=lyp-r10c2
OUT=docs/laiyipao/audits/round10-causality/screenshots
"$AB" --session "$SESSION" open "http://127.0.0.1:5174/#/pages/gems/gems" >/dev/null 2>&1
"$AB" --session "$SESSION" set viewport 390 844 >/dev/null 2>&1
sleep 2
"$AB" --session "$SESSION" screenshot "$OUT/after-03-gems.png" >/dev/null 2>&1
echo RECAPTURE_DONE