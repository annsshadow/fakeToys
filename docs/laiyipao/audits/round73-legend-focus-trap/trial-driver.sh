#!/bin/bash
set -u
AB(){ agent-browser --session "$1" "${@:2}"; }
D="docs/laiyipao/audits/round73-legend-focus-trap/data"
S="r73-trial"
PROBE="$D/r73-probe.js"
AB "$S" close --all >/dev/null 2>&1
echo "=== OPEN ==="
AB "$S" open --init-script "$D/auth-init.js" 2>&1 | tail -3
AB "$S" set viewport 768 1024 2>&1 | tail -2
echo "=== NAVIGATE ==="
AB "$S" navigate "http://127.0.0.1:5201/#/pages/battle/battle?level=1" 2>&1 | tail -4
sleep 6
echo "=== BEFORE ==="
AB "$S" eval "$(< "$PROBE")" 2>&1 | tail -5
echo "=== CLICK HELP CHIP ==="
AB "$S" eval "(function(){var c=document.querySelector(".help-chip");if(!c)return "NO_CHIP";c.click();return "CLICKED"})()" 2>&1 | tail -3
sleep 1
echo "=== AFTER OPEN ==="
AB "$S" eval "$(< "$PROBE")" 2>&1 | tail -5
AB "$S" screenshot "docs/laiyipao/audits/round73-legend-focus-trap/screenshots/trial-open-768.png" 2>&1 | tail -2
echo "=== PRESS TAB ==="
AB "$S" press Tab 2>&1 | tail -2
AB "$S" eval "$(< "$PROBE")" 2>&1 | tail -3
echo "=== PRESS ESCAPE ==="
AB "$S" press Escape 2>&1 | tail -2
sleep 0.5
AB "$S" eval "$(< "$PROBE")" 2>&1 | tail -3
echo "=== TRIAL_DONE ==="
