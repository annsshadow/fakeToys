#!/bin/bash
set -u
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
D="docs/laiyipao/audits/round23-loadout-reaction-coverage"
S="r23-deeplink-review"
mkdir -p "$D/data" "$D/screenshots"
echo "=== MINT TOKEN ==="
bash "$D/mint-token.sh" 10 || { echo MINT_FAIL; exit 1; }
echo "=== DEEP LINK DIRECT TO SKILLS ==="
"$AB" --session "$S" close >/dev/null 2>&1
"$AB" --session "$S" open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session "$S" set viewport 1258 588 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/skills/skills" >/dev/null 2>&1
sleep 7
"$AB" --session "$S" reload >/dev/null 2>&1
sleep 7
echo "=== PROBE ==="
"$AB" --session "$S" eval --stdin < "$D/probe.js" > "$D/data/probe-deeplink-review.json" 2>&1
cat "$D/data/probe-deeplink-review.json"
echo "=== SCREENSHOT ==="
"$AB" --session "$S" screenshot "$D/screenshots/r23-05-deeplink-fixed.png" >/dev/null 2>&1
echo SHOT_DONE
echo "=== CLEANUP TOKEN ==="
rm -f "$D/auth-init.js"
if [ -f "$D/auth-init.js" ]; then echo CLEANUP_FAIL; else echo CLEANED; fi
