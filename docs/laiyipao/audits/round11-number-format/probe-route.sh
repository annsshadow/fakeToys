#!/usr/bin/env bash
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
S=lyp-r11probe
"$AB" --session "$S" open >/dev/null 2>&1
echo "R=$("$AB" --session "$S" network route "**/recommendations*" --body "{"probe":123}" --allow-origin http://127.0.0.1:5174 2>&1 | head -1)"
"$AB" --session "$S" open "http://127.0.0.1:5174/" >/dev/null 2>&1
sleep 2
echo "FETCH=$("$AB" --session "$S" eval "fetch('http://127.0.0.1:8080/api/v1/me/recommendations?context=skills').then(r=>r.text()).then(t=>t.slice(0,80))" 2>&1 | tail -3)"
