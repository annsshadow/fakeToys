#!/usr/bin/env bash
set -uo pipefail
cd /d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round95-back-keyboard-postdeploy
AB(){ agent-browser --session "$1" "${@:2}"; }
BASE='http://127.0.0.1:5201/'
run_view(){
 local W=$1 H=$2 V=$3 S="r95pd3$3"
 {
 echo "=== PD3 $V START $(date +%T) ==="
 AB "$S" close --all >/dev/null 2>&1 || true
 AB "$S" open >/dev/null 2>&1; echo "OPEN=$?"
 AB "$S" set viewport "$W" "$H" >/dev/null 2>&1; echo "VIEWPORT=$?"
 AB "$S" navigate "$BASE" >/dev/null 2>&1; sleep 5
 AB "$S" eval --stdin < data/pd-click-me.js > "data/pd3-$V-clickme.txt" 2>&1; echo "--- CLICK ME ---"; cat "data/pd3-$V-clickme.txt"; sleep 3
 AB "$S" eval --stdin < data/pd-probe.js > "data/pd3-$V-me.txt" 2>&1; echo "--- AT ME ---"; cat "data/pd3-$V-me.txt"
 HIT=NONE
 for i in 1 2 3 4 5 6 7 8; do
  AB "$S" press Tab >/dev/null 2>&1; printf "TAB%s " "$i" >> "data/pd3-$V-tab.txt"
  AB "$S" eval --stdin < data/pd-probe-active.js >> "data/pd3-$V-tab.txt" 2>&1
  if grep -q "isBack.*true" "data/pd3-$V-tab.txt"; then HIT=$i; echo "HIT=$i" >> "data/pd3-$V-tab.txt"; break; fi
 done
 echo "TAB_HIT=$HIT"
 AB "$S" snapshot > "data/pd3-$V-ax.txt" 2>&1; echo "AX_CAPTURED=$?"
 AB "$S" screenshot "$(pwd)/screenshots/pd3-r95-$V-focus.png" >/dev/null 2>&1; echo "SHOT_FOCUS=$?"
 AB "$S" press Enter >/dev/null 2>&1; ENTER_RC=$?; echo "ENTER_RC=$ENTER_RC"; sleep 2
 AB "$S" eval --stdin < data/pd-probe.js > "data/pd3-$V-enter.txt" 2>&1; echo "--- ENTER ---"; cat "data/pd3-$V-enter.txt"
 AB "$S" eval --stdin < data/pd-click-me.js >/dev/null 2>&1; sleep 3
 AB "$S" click '.back-btn' >/dev/null 2>&1; CLICK_RC=$?; echo "CLICK_RC=$CLICK_RC"; sleep 2
 AB "$S" eval --stdin < data/pd-probe.js > "data/pd3-$V-click.txt" 2>&1; echo "--- CLICK ---"; cat "data/pd3-$V-click.txt"
 AB "$S" close --all >/dev/null 2>&1 || true
 echo "=== PD3 $V DONE $(date +%T) ==="
 } > "data/pd3-$V.log" 2>&1
 cat "data/pd3-$V.log"
}
run_view 414 896 414
run_view 820 1180 820
run_view 1440 900 1440
echo PD3_ALL_DONE
