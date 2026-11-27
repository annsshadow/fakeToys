#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
run_view(){
  local W="$1" H="$2" V="$3"
  local S="r94w$V"
  AB "$S" close --all >/dev/null 2>&1 || true
  AB "$S" open --init-script data/r94-init.js >/dev/null 2>&1
  AB "$S" open --init-script data/r94-init.js >/dev/null 2>&1
  AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$S" navigate "$BASE/admin/defenses" >/dev/null 2>&1
  sleep 2
  AB "$S" fill "input[placeholder='按防线名 / 玩家昵称搜索']" "NO_SUCH_zzz" >/dev/null 2>&1; echo "FILL_${V}_RC=$?"
  AB "$S" press Enter >/dev/null 2>&1; echo "ENTER_${V}_RC=$?"
  sleep 2
  echo "=== $V BEFORE ==="
  AB "$S" eval --stdin < data/r94-probe.js > "data/$V-before.txt" 2>&1; cat "data/$V-before.txt"
  AB "$S" screenshot "$(pwd)/screenshots/r94-$V-before.png" >/dev/null 2>&1; echo "SHOT_${V}_BEFORE_RC=$?"
  AB "$S" click ".empty-state .empty-state-action" >/dev/null 2>&1; echo "CLICK_${V}_RC=$?"
  sleep 2
  echo "=== $V AFTER ==="
  AB "$S" eval --stdin < data/r94-probe.js > "data/$V-after.txt" 2>&1; cat "data/$V-after.txt"
  AB "$S" screenshot "$(pwd)/screenshots/r94-$V-after.png" >/dev/null 2>&1; echo "SHOT_${V}_AFTER_RC=$?"
  echo "=== $V BATTLES PATTERN ==="
  AB "$S" navigate "$BASE/admin/battles" >/dev/null 2>&1
  sleep 2
  AB "$S" eval --stdin < data/r94-debug.js > "data/$V-battles.txt" 2>&1; cat "data/$V-battles.txt"
  AB "$S" close --all >/dev/null 2>&1 || true
}
run_view 390 844 390
run_view 768 1024 768
run_view 1280 800 1280
echo WALK_ALL_DONE