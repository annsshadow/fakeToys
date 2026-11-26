mkdir -p docs/laiyipao/audits/round7-growth/screenshots
BIN="/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe"
"$BIN" --session lyp-g1 open "http://127.0.0.1:5174/#/pages/skills/skills" >/dev/null 2>&1
"$BIN" --session lyp-g1 wait 2000 >/dev/null 2>&1
"$BIN" --session lyp-g1 set viewport 390 844 >/dev/null 2>&1
"$BIN" --session lyp-g1 screenshot "docs/laiyipao/audits/round7-growth/screenshots/after-skills.png" 2>&1 | head -n 2
echo "CAPTURED skills"
"$BIN" --session lyp-g1 open "http://127.0.0.1:5174/#/pages/equipment/equipment" >/dev/null 2>&1
"$BIN" --session lyp-g1 wait 2000 >/dev/null 2>&1
"$BIN" --session lyp-g1 set viewport 390 844 >/dev/null 2>&1
"$BIN" --session lyp-g1 screenshot "docs/laiyipao/audits/round7-growth/screenshots/after-equipment.png" 2>&1 | head -n 2
echo "CAPTURED equipment"
"$BIN" --session lyp-g1 open "http://127.0.0.1:5174/#/pages/gems/gems" >/dev/null 2>&1
"$BIN" --session lyp-g1 wait 2000 >/dev/null 2>&1
"$BIN" --session lyp-g1 set viewport 390 844 >/dev/null 2>&1
"$BIN" --session lyp-g1 screenshot "docs/laiyipao/audits/round7-growth/screenshots/after-gems.png" 2>&1 | head -n 2
echo "CAPTURED gems"
