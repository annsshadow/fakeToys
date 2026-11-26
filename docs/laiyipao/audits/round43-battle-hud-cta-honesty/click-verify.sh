#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty"
AB=$(command -v agent-browser)
S=lyp-r43-click
"$AB" --session "$S" --init-script "$D/data/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" > "$D/data/click-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 >> "$D/data/click-open.txt" 2>&1
sleep 6
"$AB" --session "$S" eval --stdin < "$D/data/state-probe.js" > "$D/data/r27-before.json" 2>&1
"$AB" --session "$S" click '.r27-ai-cta' > "$D/data/r27-click.txt" 2>&1
sleep 1
"$AB" --session "$S" eval --stdin < "$D/data/state-probe.js" > "$D/data/r27-after.json" 2>&1
"$AB" --session "$S" screenshot "$D/screenshots/r43-r27-after.png" > "$D/data/r27-shot.txt" 2>&1
"$AB" --session "$S" eval --stdin < "$D/data/state-probe.js" > "$D/data/r24-before.json" 2>&1
"$AB" --session "$S" click '.r24-ai-cta' > "$D/data/r24-click.txt" 2>&1
sleep 1
"$AB" --session "$S" eval --stdin < "$D/data/state-probe.js" > "$D/data/r24-after.json" 2>&1
"$AB" --session "$S" screenshot "$D/screenshots/r43-r24-after.png" > "$D/data/r24-shot.txt" 2>&1
echo DONE_CLICKS
