#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty"
AB=$(command -v agent-browser)
S=lyp-r43-layout
"$AB" --session "$S" --init-script "$D/data/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" > "$D/data/layout-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 >> "$D/data/layout-open.txt" 2>&1
sleep 5
"$AB" --session "$S" eval --stdin < "$D/data/layout-probe.js" > "$D/data/layout.json" 2>&1
echo DONE_LAYOUT
