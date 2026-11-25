#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty"
AB=$(command -v agent-browser || echo '')
if [ -z "$AB" ]; then AB="$(command -v node)/../agent-browser"; fi
S=lyp-r43-hud
BASE="http://127.0.0.1:5174/#/pages/battle/battle?level=5"
"$AB" --session "$S" open "$BASE" > "$D/data/open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 >> "$D/data/open.txt" 2>&1
sleep 5
"$AB" --session "$S" eval --stdin < "$D/data/battle-probe.js" > "$D/data/initial.json" 2>&1
"$AB" --session "$S" get url >> "$D/data/initial.json" 2>&1
"$AB" --session "$S" screenshot "$D/screenshots/r43-initial.png" > "$D/data/shot-initial.txt" 2>&1
echo DONE_PROBE
