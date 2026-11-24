#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round29-live-reaction-timeline
S=lyp-r29-settled
E="$PWD/$D"
SH="$PWD/$D/screenshots"
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 8
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 8
echo '=== WAIT FOR SETTLED ==='
for i in $(seq 1 220); do
  ST=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  STATE=$(echo "$ST" | cut -d'|' -f1)
  if [ "$STATE" = "SETTLED" ]; then echo "SETTLED_AT=$i"; break; fi
  sleep 0.3
done
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$D/data/settled-cta-pre.json" 2>&1
"$AB" --session $S eval --stdin < "$E/click-cta-eval.js" > "$D/data/settled-cta-hit.json" 2>&1
echo '=== CLICK CTA SETTLED ==='
"$AB" --session $S click '.r28-feed-cta' 2>&1
sleep 1
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$D/data/settled-cta-post.json" 2>&1
"$AB" --session $S eval --stdin < "$E/timeline-eval.js" > "$D/data/settled-cta-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r29-07-settled-drawer-live.png" >/dev/null 2>&1
echo SHOT_SETTLED_DRAWER
echo '=== FINAL ==='
"$AB" --session $S eval --stdin < "$E/final-eval.js" > "$D/data/settled-final.json" 2>&1
echo '=== DONE ==='
