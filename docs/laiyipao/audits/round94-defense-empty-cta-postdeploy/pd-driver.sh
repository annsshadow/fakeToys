#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
run_view(){
  local W="$1" H="$2" V="$3"
  local S="r94pd-$V"
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
  AB "$S" eval --stdin < data/r94-probe.js > "data/pd-$V-before.txt" 2>&1; cat "data/pd-$V-before.txt"
  AB "$S" screenshot "$(pwd)/screenshots/pd-r94-$V-before.png" >/dev/null 2>&1; echo "SHOT_${V}_BEFORE_RC=$?"
  AB "$S" click ".empty-state .empty-state-action" >/dev/null 2>&1; echo "CLICK_${V}_RC=$?"
  sleep 2
  echo "=== $V AFTER ==="
  AB "$S" eval --stdin < data/r94-probe.js > "data/pd-$V-after.txt" 2>&1; cat "data/pd-$V-after.txt"
  AB "$S" screenshot "$(pwd)/screenshots/pd-r94-$V-after.png" >/dev/null 2>&1; echo "SHOT_${V}_AFTER_RC=$?"
  AB "$S" close --all >/dev/null 2>&1 || true
}
run_view 414 896 414
run_view 820 1180 820
run_view 1440 900 1440
echo PD_ALL_DONE
