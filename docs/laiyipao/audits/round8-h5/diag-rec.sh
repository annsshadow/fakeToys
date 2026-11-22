#!/usr/bin/env bash
AB="/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7diag
$AB --session "$S" open "http://127.0.0.1:5174/#/pages/skills/skills"
sleep 2
$AB --session "$S" set viewport 390 844
$AB --session "$S" eval "JSON.stringify(Object.keys(localStorage))"
$AB --session "$S" eval "fetch('/api/v1/me/recommendations?context=skills').then(async r=>JSON.stringify({status:r.status,body:(await r.text()).slice(0,800)}))"
echo DIAG_DONE
