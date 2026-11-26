#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty"
AB=$(command -v agent-browser)
S=lyp-r43-helpctl
P="$D/data"; X="$D/screenshots"
"$AB" --session "$S" --init-script "$P/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" > "$P/hc-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 >> "$P/hc-open.txt" 2>&1
sleep 6
"$AB" --session "$S" eval --stdin < "$P/coord-state.js" > "$P/hc-before.json" 2>&1
"$AB" --session "$S" click .help-chip > "$P/hc-click.txt" 2>&1
sleep 2
"$AB" --session "$S" eval --stdin < "$P/coord-state.js" > "$P/hc-open-state.json" 2>&1
"$AB" --session "$S" screenshot "$X/r43-hc-help-open.png" > "$P/hc-open-shot.txt" 2>&1
"$AB" --session "$S" click .legend-close > "$P/hc-close.txt" 2>&1
sleep 1
"$AB" --session "$S" eval --stdin < "$P/coord-state.js" > "$P/hc-closed-state.json" 2>&1
"$AB" --session "$S" screenshot "$X/r43-hc-help-closed.png" > "$P/hc-closed-shot.txt" 2>&1
echo HC_DONE