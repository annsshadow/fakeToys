#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round28-live-reaction-attribution
S=lyp-r28-cta
E="$PWD/$D"
SH="$PWD/$D/screenshots"
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 6
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 6
echo '=== WAIT FEED ==='
for i in $(seq 1 40); do
  ST=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  echo "[$i] $ST"
  LINES=$(echo "$ST" | cut -d'|' -f2)
  if [ "$LINES" -ge 1 ] 2>/dev/null; then echo FEED_READY; break; fi
  sleep 0.5
done
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$D/data/cta-pre.json" 2>&1
echo '=== CLICK CTA ==='
"$AB" --session $S click '.r28-feed-cta' 2>&1
sleep 1
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$D/data/cta-post.json" 2>&1
"$AB" --session $S screenshot "$SH/r28-05-cta-open.png" >/dev/null 2>&1
echo SHOT_CTA_OPEN
echo '=== CLOSE ==='
"$AB" --session $S click '.legend-close' 2>&1
sleep 1
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$D/data/cta-closed.json" 2>&1
echo '=== DONE ==='
