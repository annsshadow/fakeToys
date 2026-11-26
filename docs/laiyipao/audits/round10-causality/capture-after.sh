#!/usr/bin/env bash
# R10 causality QA capture — 390x844 desktop browser emulated mobile viewport, NOT physical device, loopback only.
set -u
AB=/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe
SESSION=lyp-r10c
OUT=docs/laiyipao/audits/round10-causality/screenshots
mkdir -p "$OUT"
open_page() {
  "$AB" --session "$SESSION" open "http://127.0.0.1:5174/#/pages/$1" >/dev/null 2>&1
  "$AB" --session "$SESSION" set viewport 390 844 >/dev/null 2>&1
  sleep 2
}
shot() {
  "$AB" --session "$SESSION" screenshot "$OUT/$1" >/dev/null 2>&1
  echo "SHOT $1"
}
open_page index/index
shot after-01-home.png
open_page battles/battles
shot after-02-battles.png
open_page gems/gems
shot after-03-gems.png
open_page skills/skills
shot after-04-skills.png
open_page equipment/equipment
shot after-05-equipment.png
open_page bag/bag
shot after-06-bag.png
echo CAPTURE_DONE