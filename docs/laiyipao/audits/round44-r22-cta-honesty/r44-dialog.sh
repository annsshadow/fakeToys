#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round44-r22-cta-honesty/data"
S="lyp-r44-dialog"
A=(agent-browser --session "$S")
"${A[@]}" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" --init-script "$D/auth-init.js" >"$D/d-open.txt" 2>&1
"${A[@]}" set viewport 390 844 >"$D/d-viewport.txt" 2>&1
"${A[@]}" wait --text "这一波给你 3 张卡" >"$D/d-wait.txt" 2>&1
"${A[@]}" eval "$(cat "$D/dialog-probe.js")" >"$D/d-dialog.json" 2>&1
"${A[@]}" screenshot "$ROOT/docs/laiyipao/audits/round44-r22-cta-honesty/screenshots/r44-dialog.png" >"$D/d-shot.txt" 2>&1
node "$D/summary.js" "$D" d-dialog.json
