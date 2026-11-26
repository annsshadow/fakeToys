#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty"
AB=$(command -v agent-browser)
S=lyp-r43-auth
"$AB" --session "$S" --init-script "$D/data/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" > "$D/data/auth-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 >> "$D/data/auth-open.txt" 2>&1
sleep 6
"$AB" --session "$S" eval --stdin < "$D/data/battle-probe.js" > "$D/data/auth-initial.json" 2>&1
"$AB" --session "$S" screenshot "$D/screenshots/r43-auth-initial.png" > "$D/data/auth-shot.txt" 2>&1
echo DONE_AUTH_PROBE
