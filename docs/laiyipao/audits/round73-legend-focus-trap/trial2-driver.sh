#!/bin/bash
set -uo pipefail
P="docs/laiyipao/audits/round73-legend-focus-trap"
D="$P/data"; S="$P/screenshots"; OUT="$D/trial2-out.txt"
AB(){ agent-browser --session "$1" "${@:2}"; }
S="r73-t2"
: > "$OUT"
AB "$S" close --all >/dev/null 2>&1 || true
AB "$S" open --init-script "$D/auth-init.js" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "http://127.0.0.1:5201/#/pages/battle/battle?level=1" >/dev/null 2>&1
sleep 6
echo "BEFORE:" >> "$OUT"
AB "$S" eval --stdin < "$D/r73-probe.js" >> "$OUT" 2>&1
echo "CLICK:" >> "$OUT"
AB "$S" eval --stdin < "$D/r73-click.js" >> "$OUT" 2>&1
sleep 1
echo "AFTER_OPEN:" >> "$OUT"
AB "$S" eval --stdin < "$D/r73-probe.js" >> "$OUT" 2>&1
AB "$S" screenshot "$S/trial2-open-768.png" >> "$OUT" 2>&1
for i in 1 2 3 4 5 6; do AB "$S" press Tab >/dev/null 2>&1; echo "TAB$i:" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-probe.js" >> "$OUT" 2>&1; done
echo "ESC:" >> "$OUT"
AB "$S" press Escape >/dev/null 2>&1
sleep 0.5
AB "$S" eval --stdin < "$D/r73-probe.js" >> "$OUT" 2>&1
AB "$S" close >/dev/null 2>&1 || true
echo TRIAL2_DONE >> "$OUT"
