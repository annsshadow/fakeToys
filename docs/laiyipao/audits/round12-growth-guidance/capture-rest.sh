#!/usr/bin/env bash
AB="/c/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/node_modules/agent-browser/bin/agent-browser-win32-x64.exe"
S=lyp-r12probe
D=docs/laiyipao/audits/round12-growth-guidance/screenshots
for P in equipment/equipment gems/gems bag/bag; do
  N=$(echo $P | cut -d/ -f1)
  "$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/$P" >/dev/null 2>&1
  sleep 3
  "$AB" --session "$S" screenshot "$D/probe-$N.png" >/dev/null 2>&1
  echo "SHOT $N"
done
echo CAPTURE_DONE