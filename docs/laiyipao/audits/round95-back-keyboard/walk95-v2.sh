#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
BASE='http://127.0.0.1:5174/'
run_view(){
 local W=$1 H=$2 V=$3 S="r95v$3"
 {
 echo "=== $V START $(date +%T) ==="
 AB "$S" close --all >/dev/null 2>&1 || true
 AB "$S" open >/dev/null 2>&1; echo OPEN=$?
 AB "$S" set viewport "$W" "$H" >/dev/null 2>&1; echo VIEWPORT=$?
 AB "$S" navigate "$BASE" >/dev/null 2>&1; sleep 3
 echo '--- BEFORE NAV ---'; AB "$S" eval --stdin < data/r95-probe.js
 echo '--- NAV TO ME ---'; AB "$S" eval --stdin < data/r95-nav-me.js; sleep 3
 echo '--- ME VIA NAVIGATETO ---'; AB "$S" eval --stdin < data/r95-probe.js > "data/$V-me.txt" 2>&1; cat "data/$V-me.txt"
 echo '--- TAB SEQUENCE ---'; : > "data/$V-tab.txt"
 for i in 1 2 3 4 5 6 7 8; do
  AB "$S" press Tab >/dev/null 2>&1; printf 'TAB%s ' "$i" >> "data/$V-tab.txt"
  AB "$S" eval --stdin < data/r95-probe-active.js >> "data/$V-tab.txt" 2>&1
  if grep -q '"isBack":true' "data/$V-tab.txt"; then echo "HIT=$i" >> "data/$V-tab.txt"; break; fi
 done
 cat "data/$V-tab.txt"
 AB "$S" screenshot "$(pwd)/screenshots/r95-$V-focus.png" >/dev/null 2>&1; echo "SHOT_FOCUS=$?"
 echo '--- ENTER ON BACK ---'; AB "$S" press Enter >/dev/null 2>&1; sleep 2
 AB "$S" eval --stdin < data/r95-probe.js > "data/$V-enter.txt" 2>&1; cat "data/$V-enter.txt"
 echo '--- CLICK CHANNEL ---'; AB "$S" eval --stdin < data/r95-nav-me.js >/dev/null 2>&1; sleep 3
 AB "$S" click '.back-btn' >/dev/null 2>&1; echo "CLICK_RC=$?"; sleep 2
 AB "$S" eval --stdin < data/r95-probe.js > "data/$V-click.txt" 2>&1; cat "data/$V-click.txt"
 echo '--- DIRECT ME STACK=1 ---'; AB "$S" close --all >/dev/null 2>&1 || true
 AB "$S" open >/dev/null 2>&1; AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
 AB "$S" navigate "$BASE#/pages/me/me" >/dev/null 2>&1; sleep 3
 AB "$S" eval --stdin < data/r95-probe.js > "data/$V-direct.txt" 2>&1; cat "data/$V-direct.txt"
 AB "$S" close --all >/dev/null 2>&1 || true
 echo "=== $V DONE $(date +%T) ==="
 } > "data/walk2-$V.log" 2>&1
 cat "data/walk2-$V.log"
}
run_view 390 844 390
run_view 768 1024 768
run_view 1280 800 1280
echo WALK95_ALL_DONE