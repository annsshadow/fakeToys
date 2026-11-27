#!/bin/bash
set -uo pipefail
P="docs/laiyipao/audits/round73-legend-focus-trap"; D="$P/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
S="r73real"; OUT="$D/real-out.txt"; : > "$OUT"
AB "$S" close --all >/dev/null 2>&1 || true
AB "$S" open --init-script "$D/auth-init.js" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "http://127.0.0.1:5201/#/pages/battle/battle?level=1" >/dev/null 2>&1
sleep 6
echo "=== BEFORE ===" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-probe.js" >> "$OUT" 2>&1
echo "=== CTA ===" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-cta-probe.js" >> "$OUT" 2>&1
echo "=== FOCUS_CHIP ===" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-focus-chip.js" >> "$OUT" 2>&1
AB "$S" press Enter >/dev/null 2>&1; sleep 0.4
echo "=== AFTER_ENTER_OPEN ===" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-probe.js" >> "$OUT" 2>&1
echo "=== FOCUS_CLOSE ===" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-focus-close.js" >> "$OUT" 2>&1
AB "$S" press Enter >/dev/null 2>&1; sleep 0.4
echo "=== AFTER_CLOSE_ENTER ===" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-probe.js" >> "$OUT" 2>&1
echo "=== CLICK_PAUSE ===" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-click-pause.js" >> "$OUT" 2>&1
echo "=== AFTER_MANUAL_PAUSE ===" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-probe.js" >> "$OUT" 2>&1
echo "=== FOCUS_CHIP2 ===" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-focus-chip.js" >> "$OUT" 2>&1
AB "$S" press Enter >/dev/null 2>&1; sleep 0.4
echo "=== LEGEND_OVER_MANUAL_PAUSE ===" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-probe.js" >> "$OUT" 2>&1
AB "$S" press Escape >/dev/null 2>&1; sleep 0.4
echo "=== AFTER_ESC_MANUAL_PAUSE ===" >> "$OUT"; AB "$S" eval --stdin < "$D/r73-probe.js" >> "$OUT" 2>&1
AB "$S" close >/dev/null 2>&1 || true
echo REAL_DONE >> "$OUT"
