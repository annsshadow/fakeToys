#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r9-h5; D='docs/laiyipao/audits/round9-after/'; SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round9-after/screenshots'
j(){ "$AB" --session "$S" eval --stdin < "$D/$1" 2>&1 | head -1; }
echo '=== RESTART BATTLE L1 ==='
"$AB" --session "$S" open 'http://127.0.0.1:5174/#/pages/battle/battle?level=1' >/dev/null 2>&1; sleep 6
echo "SPEED: $(j speed8.js)"
for i in $(seq 1 25); do
  R=$(j next-wave.js)
  SP=$(j settle-probe.js)
  echo "ROUND $i: $R | $SP"
  case "$SP" in *'"nextLevel":"前往'*) echo '=== SETTLED ==='; break;; esac
  sleep 3
done
echo '=== FINAL SETTLE PROBE ==='
j settle-probe.js
"$AB" --session "$S" screenshot "$SHOT/r9-h5-settle-nextlevel.png" >/dev/null 2>&1; echo SHOT_DONE
echo '=== ERRORS ==='
"$AB" --session "$S" errors --json 2>&1 | head -3
echo SETTLE_DONE