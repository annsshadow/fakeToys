#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round27-postdeploy
S=lyp-r27-postdeploy
E="$PWD/$D"
DATA="$PWD/$D/data"
echo '=== COLLECT FINAL ==='
"$AB" --session $S eval --stdin < "$E/final-eval.js" > "$DATA/postdeploy-final.json" 2>&1
cat "$DATA/postdeploy-final.json"
echo '=== DONE ==='
