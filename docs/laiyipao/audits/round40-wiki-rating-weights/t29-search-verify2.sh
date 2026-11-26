#!/bin/bash
set -u
cd "$(dirname "$0")/../../../.." || exit 1
D=docs/laiyipao/audits/round40-wiki-rating-weights
DATA=$D/data
SHOTS=$D/screenshots
SESSION=lyp-t29-search2
agent-browser close --all >/dev/null 2>&1 || true
for V in 390:844 768:1024 1280:800; do
  W=${V%:*}; H=${V#*:}
  agent-browser --session "$SESSION" set viewport "$W" "$H" >/dev/null 2>&1
  agent-browser --session "$SESSION" open --init-script "$DATA/admin-auth-init.js" "http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=rating" > "$DATA/t29b-open-$W.txt" 2>&1
  agent-browser --session "$SESSION" wait 1200 >/dev/null 2>&1
  for Q in 反应链 mechanic_depth; do
    if [ "$Q" = "反应链" ]; then TAG=fanyinglian; else TAG=mechanic_depth; fi
    agent-browser --session "$SESSION" fill ".search-input" "$Q" > "$DATA/t29b-fill-$W-$TAG.txt" 2>&1
    agent-browser --session "$SESSION" wait 400 >/dev/null 2>&1
    agent-browser --session "$SESSION" eval "$(tr "\n" " " < "$D/search-probe.js")" > "$DATA/t29b-search-$W-$TAG.json" 2>&1
    agent-browser --session "$SESSION" click ".search-hit" > "$DATA/t29b-click-$W-$TAG.txt" 2>&1
    agent-browser --session "$SESSION" wait 500 >/dev/null 2>&1
    agent-browser --session "$SESSION" eval "$(tr "\n" " " < "$D/search-probe.js")" > "$DATA/t29b-after-$W-$TAG.json" 2>&1
    agent-browser --session "$SESSION" screenshot "$SHOTS/t29b-$W-$TAG.png" > "$DATA/t29b-shot-$W-$TAG.txt" 2>&1
  done
done
agent-browser --session "$SESSION" close >/dev/null 2>&1 || true
echo T29B_DONE=YES
