#!/bin/bash
D=docs/laiyipao/audits/round21-player-level-coach
AB=agent-browser
for pair in "lyp-r21-data data-card" "lyp-r21-empty empty-card"; do
  set -- $pair; S="$1"; N="$2"
  "$AB" eval --session "$S" --stdin < "$D/scroll.js" > "$D/data/scroll-${N}.json" 2>&1
  sleep 1
  "$AB" screenshot --session "$S" "$D/screenshots/r21-${N}.png" >/dev/null 2>&1
  echo "SHOT $N done"
done