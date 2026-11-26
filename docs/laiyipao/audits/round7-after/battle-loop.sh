#!/bin/bash
# t67 真实战斗闭环：简单文本轮询 + 真实鼠标点击（非物理真机）
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7-h5; BASE="http://127.0.0.1:5174/#"; D="."
run(){ $AB --session "$S" "$@" 2>&1 | head -1; }
j(){ $AB --session "$S" eval --stdin < "$D/$1" 2>&1 | head -1 | sed 's/^"//;s/"$//'; }
click_xy(){ run mouse move "$1" "$2" >/dev/null; sleep 0.15; run mouse down >/dev/null; sleep 0.15; run mouse up >/dev/null; sleep 0.35; }
echo "=== WALLET BEFORE: $(j battle-wallet.js) ==="
echo "RECORDS BEFORE: $(j battle-records.js)"
for ROUND in 1 2 3; do
  echo "=== ROUND $ROUND ==="
  if [ "$ROUND" = "1" ]; then run open "$BASE/pages/stage/stage" >/dev/null; sleep 6; echo "START: $(j battle-start.js)"; sleep 5;
  else R=$(j battle-retry.js); echo "RETRY_AT: $R"; set -- $R; if [ -n "$1" ]; then click_xy "$1" "$2"; fi; sleep 5; fi
  if [ "$ROUND" != "1" ]; then echo "START: $(j battle-start.js)"; sleep 5; fi
  SP=$(j battle-speed.js); echo "SPEED: $SP"
  SX=$(printf "%s" "$SP" | sed -n 's/.*"x":\([0-9-]*\).*/\1/p')
  SY=$(printf "%s" "$SP" | sed -n 's/.*"y":\([0-9-]*\).*/\1/p')
  if [ -n "$SX" ] && [ -n "$SY" ]; then click_xy "$SX" "$SY"; fi
  for I in $(seq 1 120); do
    ST=$(j battle-poll.js)
    case "$ST" in RESULT*) echo "FINAL: $ST"; break;; esac
    case "$ST" in SKIP*) set -- $ST; click_xy "$2" "$3"; echo "SKIPPED wave at $2,$3";; esac
    if [ $((I % 20)) -eq 0 ]; then echo "POLL $I: $ST"; fi
    sleep 1
  done
  run screenshot "$D/screenshots/r7-battle-settle-$ROUND.png" >/dev/null
  echo "RECORDS: $(j battle-records.js)"
done
echo "=== WALLET AFTER: $(j battle-wallet.js) ==="
echo "IMPACT: $(j impact.js)"
echo BATTLE_DONE
