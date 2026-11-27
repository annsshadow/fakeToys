#!/bin/bash
set -uo pipefail
PD=docs/laiyipao/audits/round73-legend-focus-trap-postdeploy
D=$PD/data
S=$PD/screenshots
AB(){ agent-browser --session "$1" "${@:2}"; }
run_case(){
  local V="$1" W="$2" H="$3"
  local SESS="r73pd-$V"
  local O="$D/pd-$V-ops.txt"
  : > "$O"
  AB "$SESS" close --all >/dev/null 2>&1 || true
  AB "$SESS" open --init-script "$D/auth-init.js" >/dev/null 2>&1
  AB "$SESS" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$SESS" navigate "http://127.0.0.1:5201/#/pages/battle/battle?level=1" >/dev/null 2>&1
  sleep 6
  echo "=== $V BEFORE ===" >> "$O"; AB "$SESS" eval --stdin < "$PD/pd-probe.js" >> "$O" 2>&1
  AB "$SESS" screenshot "$S/pd-$V-before.png" >> "$O" 2>&1
  AB "$SESS" eval --stdin < "$D/r73-click.js" >/dev/null 2>&1
  sleep 1
  echo "=== $V OPEN ===" >> "$O"; AB "$SESS" eval --stdin < "$PD/pd-probe.js" >> "$O" 2>&1
  AB "$SESS" screenshot "$S/pd-$V-open.png" >> "$O" 2>&1
  : > "$D/pd-$V-tab.txt"
  for i in 1 2 3 4 5 6; do AB "$SESS" press Tab >/dev/null 2>&1; echo "TAB$i" >> "$D/pd-$V-tab.txt"; AB "$SESS" eval --stdin < "$D/r73-probe-lite.js" >> "$D/pd-$V-tab.txt" 2>&1; done
  : > "$D/pd-$V-shifttab.txt"
  for i in 1 2 3 4 5 6; do AB "$SESS" press Shift+Tab >/dev/null 2>&1; echo "SHIFT_TAB$i" >> "$D/pd-$V-shifttab.txt"; AB "$SESS" eval --stdin < "$D/r73-probe-lite.js" >> "$D/pd-$V-shifttab.txt" 2>&1; done
  AB "$SESS" press Escape >/dev/null 2>&1
  sleep 0.5
  echo "=== $V CLOSED ===" >> "$O"; AB "$SESS" eval --stdin < "$PD/pd-probe.js" >> "$O" 2>&1
  AB "$SESS" screenshot "$S/pd-$V-closed.png" >> "$O" 2>&1
  echo "=== $V MANUAL_PAUSE ===" >> "$O"
  AB "$SESS" eval --stdin < "$D/r73-click-pause.js" >> "$O" 2>&1
  AB "$SESS" eval --stdin < "$D/r73-focus-chip.js" >> "$O" 2>&1
  AB "$SESS" press Enter >/dev/null 2>&1; sleep 0.4
  AB "$SESS" eval --stdin < "$PD/pd-probe.js" >> "$O" 2>&1
  AB "$SESS" press Escape >/dev/null 2>&1; sleep 0.4
  AB "$SESS" eval --stdin < "$PD/pd-probe.js" >> "$O" 2>&1
  echo "=== $V MUTEX ===" >> "$O"
  AB "$SESS" eval --stdin < "$D/r73-cta-probe.js" >> "$O" 2>&1
  AB "$SESS" eval --stdin < "$D/r73-click.js" >/dev/null 2>&1
  AB "$SESS" eval --stdin < "$PD/pd-probe.js" >> "$O" 2>&1
  AB "$SESS" eval --stdin < "$D/r73-cta-click.js" >> "$O" 2>&1
  AB "$SESS" eval --stdin < "$PD/pd-probe.js" >> "$O" 2>&1
  AB "$SESS" close >/dev/null 2>&1 || true
  echo "CASE_DONE $V"
}
run_case 414 414 896
run_case 820 820 1180
run_case 1440 1440 900
echo PD_DONE_ALL