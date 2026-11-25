#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round44-r22-cta-honesty/data"
S="lyp-r44-qa"
A=(agent-browser --session "$S")
num() { node -e 'const fs=require("fs");const j=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));let v=typeof j==="string"?JSON.parse(j):j;console.log(v[process.argv[2]])' "$1" "$2"; }
"${A[@]}" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" --init-script "$D/auth-init.js" >"$D/r44-open.txt" 2>&1
"${A[@]}" set viewport 390 844 >"$D/r44-viewport.txt" 2>&1
"${A[@]}" wait --text "防线" >"$D/r44-wait.txt" 2>&1 || true
"${A[@]}" eval "$(cat "$D/dialog-probe.js")" >"$D/r44-initial.json" 2>&1
"${A[@]}" screenshot "$ROOT/docs/laiyipao/audits/round44-r22-cta-honesty/screenshots/r44-initial.png" >"$D/r44-initial-shot.txt" 2>&1
node "$D/summary.js" "$D" r44-initial.json
