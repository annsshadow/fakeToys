#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty"
AB=$(command -v agent-browser)
S=lyp-r43-stage
"$AB" --session "$S" open "http://127.0.0.1:5174/#/pages/stage/stage" > "$D/data/stage-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 >> "$D/data/stage-open.txt" 2>&1
sleep 5
"$AB" --session "$S" eval --stdin < "$D/data/stage-probe.js" > "$D/data/stage.json" 2>&1
"$AB" --session "$S" screenshot "$D/screenshots/r43-stage.png" > "$D/data/stage-shot.txt" 2>&1
echo DONE_STAGE
