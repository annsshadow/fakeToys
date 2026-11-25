#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round44-r22-cta-honesty/data"
S="lyp-r44-diag"
A=(agent-browser --session "$S")
"${A[@]}" open "http://127.0.0.1:5174/oa4rust-h5/#/pages/battle/battle?level=5" --init-script "$D/auth-init.js" >"$D/diag-open.txt" 2>&1
"${A[@]}" set viewport 390 844 >"$D/diag-view.txt" 2>&1
"${A[@]}" eval "$(cat "$D/dialog-probe.js")" >"$D/diag-initial.json" 2>&1
"${A[@]}" wait 12000 >"$D/diag-wait.txt" 2>&1
"${A[@]}" eval "$(cat "$D/dialog-probe.js")" >"$D/diag-after12.json" 2>&1
node "$D/summary.js" "$D" diag-initial.json diag-after12.json
