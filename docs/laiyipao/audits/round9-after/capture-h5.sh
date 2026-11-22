#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r9-h5; BASE='http://127.0.0.1:5174/#'; D='docs/laiyipao/audits/round9-after/'; SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round9-after/screenshots'
run(){ "$AB" --session "$S" "$@" 2>&1; }
j(){ "$AB" --session "$S" eval --stdin < "$D/$1" 2>&1 | head -1; }
echo '=== LOGIN ==='
run --init-script "$D/probe-init.js" open "$BASE/pages/login/login" >/dev/null 2>&1; sleep 4
run set viewport 390 844 >/dev/null 2>&1
j clear-token.js >/dev/null 2>&1
run fill "input[type='text']" 'admin' >/dev/null 2>&1
run fill "input[type='password']" 'admin12345' >/dev/null 2>&1
run click '.btn-primary' >/dev/null 2>&1; sleep 6
echo "$(j token-len.js)"
for P in index stage skills equipment reactions battles me; do
  if [ "$P" = stage ]; then URL="$BASE/pages/stage/stage?level=1"; else URL="$BASE/pages/$P/$P"; fi
  run open "$URL" >/dev/null 2>&1; sleep 5
  echo "--- $P ---"
  echo "PROBE: $(j h5-probe.js)"
  run screenshot "$SHOT/r9-h5-$P.png" >/dev/null 2>&1
done
echo '=== BATTLE ==='
run open "$BASE/pages/stage/stage?level=1" >/dev/null 2>&1; sleep 4
run click '.start-btn' >/dev/null 2>&1; sleep 8
echo "BATTLE0: $(j h5-probe.js)"
run screenshot "$SHOT/r9-h5-battle.png" >/dev/null 2>&1
sleep 6
echo "BATTLE1: $(j h5-probe.js)"
echo H5_DONE