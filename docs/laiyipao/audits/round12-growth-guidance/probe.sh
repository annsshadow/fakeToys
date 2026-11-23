#!/usr/bin/env bash
AB="/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe"
S=lyp-r12probe
D=docs/laiyipao/audits/round12-growth-guidance/screenshots
mkdir -p "$D"
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/index/index" >/dev/null 2>&1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
sleep 3
echo "=== HOME URL ==="
"$AB" --session "$S" get url 2>/dev/null || true
"$AB" --session "$S" screenshot "$D/probe-home.png" >/dev/null 2>&1
echo "HOME_SHOT_DONE"
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/skills/skills" >/dev/null 2>&1
sleep 3
echo "=== SKILLS URL ==="
"$AB" --session "$S" get url 2>/dev/null || true
"$AB" --session "$S" screenshot "$D/probe-skills.png" >/dev/null 2>&1
echo "SKILLS_SHOT_DONE"
echo "PROBE_DONE"