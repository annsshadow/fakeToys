#!/bin/bash
set -u
cd "$(dirname "$0")/../../../.." || exit 1
D=docs/laiyipao/audits/round40-wiki-rating-weights
DATA=$D/data
SHOTS=$D/screenshots
mkdir -p "$DATA" "$SHOTS"
AUTH=$DATA/admin-auth-init.js
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=rating"
SESSION=lyp-t29-search
agent-browser close --all >/dev/null 2>&1 || true
for V in 390:844 768:1024 1280:800; do
  W=${V%:*}; H=${V#*:}
  agent-browser --session "$SESSION" set viewport "$W" "$H" >/dev/null 2>&1
  agent-browser --session "$SESSION" open --init-script "$AUTH" "$URL" > "$DATA/t29-open-$W.txt" 2>&1
  agent-browser --session "$SESSION" wait 1200 >/dev/null 2>&1
  for Q in 反应链 mechanic_depth; do
    if [ "$Q" = "反应链" ]; then TAG=fanyinglian; else TAG=mechanic_depth; fi
    agent-browser --session "$SESSION" fill ".search-input" "$Q" > "$DATA/t29-fill-$W-$TAG.txt" 2>&1
    agent-browser --session "$SESSION" wait 400 >/dev/null 2>&1
    agent-browser --session "$SESSION" eval "$(cat $D/search-probe.js)" > "$DATA/t29-search-$W-$TAG.json" 2>&1
    agent-browser --session "$SESSION" click ".search-hit" > "$DATA/t29-click-$W-$TAG.txt" 2>&1
    agent-browser --session "$SESSION" wait 500 >/dev/null 2>&1
    agent-browser --session "$SESSION" eval "$(cat $D/search-probe.js)" > "$DATA/t29-after-$W-$TAG.json" 2>&1
    agent-browser --session "$SESSION" screenshot "$SHOTS/t29-$W-$TAG.png" > "$DATA/t29-shot-$W-$TAG.txt" 2>&1
  done
done
agent-browser --session "$SESSION" close >/dev/null 2>&1 || true
echo T29_SEARCH_DONE=YES
