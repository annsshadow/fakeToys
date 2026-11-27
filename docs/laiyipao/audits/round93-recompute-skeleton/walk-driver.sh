#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
run_view(){
  local W="$1" H="$2" V="$3"
  local S="r93w$V"
  AB "$S" close --all >/dev/null 2>&1 || true
  AB "$S" open --init-script data/r93-init.js >/dev/null 2>&1
  AB "$S" open --init-script data/r93-init.js >/dev/null 2>&1
  AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$S" navigate "$BASE/admin/recomputes" >/dev/null 2>&1
  sleep 1.5
  echo "=== $V LOAD ==="
  AB "$S" eval --stdin < data/r93-probe.js > "data/$V-load.txt" 2>&1
  cat "data/$V-load.txt"
  AB "$S" screenshot "$(pwd)/screenshots/r93-$V-load.png" >/dev/null 2>&1; echo "SHOT_${V}_LOAD_RC=$?"
  sleep 6.5
  echo "=== $V ARRIVE ==="
  AB "$S" eval --stdin < data/r93-probe.js > "data/$V-arrive.txt" 2>&1
  cat "data/$V-arrive.txt"
  AB "$S" screenshot "$(pwd)/screenshots/r93-$V-arrive.png" >/dev/null 2>&1; echo "SHOT_${V}_ARRIVE_RC=$?"
  echo "=== $V REFRESH CLICK ==="
  AB "$S" eval --stdin < data/r93-click-refresh.js
  sleep 0.3
  echo "=== $V REFRESH MID ==="
  AB "$S" eval --stdin < data/r93-refresh-probe.js > "data/$V-refresh-mid.txt" 2>&1
  cat "data/$V-refresh-mid.txt"
  sleep 6.5
  echo "=== $V REFRESH AFTER ==="
  AB "$S" eval --stdin < data/r93-refresh-probe.js > "data/$V-refresh-after.txt" 2>&1
  cat "data/$V-refresh-after.txt"
  echo "=== $V PATTERN BATTLES LOAD ==="
  AB "$S" navigate "$BASE/admin/battles" >/dev/null 2>&1
  sleep 1.2
  AB "$S" eval --stdin < data/r93-probe.js > "data/$V-pattern.txt" 2>&1
  cat "data/$V-pattern.txt"
  AB "$S" screenshot "$(pwd)/screenshots/r93-$V-pattern.png" >/dev/null 2>&1; echo "SHOT_${V}_PATTERN_RC=$?"
  AB "$S" close --all >/dev/null 2>&1 || true
}
run_view 390 844 390
run_view 768 1024 768
run_view 1280 800 1280
echo WALK_ALL_DONE