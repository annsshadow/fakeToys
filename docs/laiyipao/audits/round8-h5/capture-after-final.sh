#!/usr/bin/env bash
AB="/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7final
$AB --session "$S" open "http://127.0.0.1:5174/#/pages/index/index"
sleep 3
$AB --session "$S" set viewport 390 844
$AB --session "$S" eval "!!localStorage.getItem('lyp_token')"
$AB --session "$S" open "http://127.0.0.1:5174/#/pages/skills/skills"
sleep 3
$AB --session "$S" set viewport 390 844
$AB --session "$S" screenshot docs/laiyipao/audits/round8-h5/screenshots/after-skills.png
$AB --session "$S" open "http://127.0.0.1:5174/#/pages/equipment/equipment"
sleep 3
$AB --session "$S" set viewport 390 844
$AB --session "$S" screenshot docs/laiyipao/audits/round8-h5/screenshots/after-equipment.png
echo FINAL_CAPTURE_DONE
