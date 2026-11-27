#!/bin/bash
set -uo pipefail
P="docs/laiyipao/audits/round73-legend-focus-trap"
D="$P/data"; S="$P/screenshots"
AB(){ agent-browser --session "$1" "${@:2}"; }
run_view(){
  local V="$1" W="$2" H="$3"
  local SS="r73w-$V"
  local SHOT_OPEN="$S/r73-$V-open.png"
  local SHOT_CLOSED="$S/r73-$V-closed.png"
  AB "$SS" close >/dev/null 2>&1 || true
  AB "$SS" open --init-script "$D/auth-init.js" >/dev/null 2>&1
  AB "$SS" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$SS" navigate "http://127.0.0.1:5201/#/pages/battle/battle?level=1" >/dev/null 2>&1
  sleep 6
  { echo "=== $V BEFORE ==="; AB "$SS" eval --stdin < "$D/r73-probe.js"; } > "$D/r73-$V-core.txt" 2>&1
  AB "$SS" eval --stdin < "$D/r73-click.js" >/dev/null 2>&1
  sleep 1
  { echo "=== $V AFTER_OPEN ==="; AB "$SS" eval --stdin < "$D/r73-probe.js"; } >> "$D/r73-$V-core.txt" 2>&1
  AB "$SS" screenshot "$SHOT_OPEN" >> "$D/r73-$V-core.txt" 2>&1
  : > "$D/r73-$V-tab.txt"
  for i in 1 2 3 4 5 6; do AB "$SS" press Tab >/dev/null 2>&1; echo "TAB$i" >> "$D/r73-$V-tab.txt"; AB "$SS" eval --stdin < "$D/r73-probe-lite.js" >> "$D/r73-$V-tab.txt" 2>&1; done
  : > "$D/r73-$V-shifttab.txt"
  for i in 1 2 3 4 5 6; do AB "$SS" press Shift+Tab >/dev/null 2>&1; echo "SHIFT_TAB$i" >> "$D/r73-$V-shifttab.txt"; AB "$SS" eval --stdin < "$D/r73-probe-lite.js" >> "$D/r73-$V-shifttab.txt" 2>&1; done
  AB "$SS" press Escape >/dev/null 2>&1
  sleep 0.5
  { echo "=== $V AFTER_ESC ==="; AB "$SS" eval --stdin < "$D/r73-probe.js"; } >> "$D/r73-$V-core.txt" 2>&1
  AB "$SS" screenshot "$SHOT_CLOSED" >> "$D/r73-$V-core.txt" 2>&1
  AB "$SS" close >/dev/null 2>&1 || true
  echo "DONE $V"
}
run_view 390 390 844
run_view 768 768 1024
run_view 1280 1280 800
echo WALK_ALL_DONE
