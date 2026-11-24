#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round30-reaction-feed-mobile-fit
S=lyp-r30-pressure
SH="$PWD/$D/screenshots"
DATA="$PWD/$D/data"
E="$PWD/$D"
echo '=== OPEN ==='
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 8
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 8
echo '=== WAIT LIVE ==='
READY=0
for i in $(seq 1 120); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  IFS='|' read -r ST LINES ROWS OV LEGEND CARD SKIP <<< "$TK"
  echo "[$i] $TK"
  if [ "$ST" = "WAIT" ] && [ "$LINES" -ge 3 ] 2>/dev/null; then READY=1; break; fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1; fi
  sleep 0.3
done
echo "READY=$READY"
echo '=== PRESSURE VISIBLE ==='
if [ "$READY" = 1 ]; then
  "$AB" --session $S eval --stdin < "$E/stress-apply-eval.js" > "$DATA/stress2-applied.json" 2>&1
  "$AB" --session $S eval --stdin < "$E/measure-eval.js" > "$DATA/stress2-measure.json" 2>&1
  "$AB" --session $S screenshot "$SH/r30-05-stress-visible.png" >/dev/null 2>&1
  "$AB" --session $S eval --stdin < "$E/stress-restore-eval.js" > "$DATA/stress2-restored.json" 2>&1
  "$AB" --session $S screenshot "$SH/r30-06-stress-restored.png" >/dev/null 2>&1
fi
echo '=== WAIT CTA ==='
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/cta-wait-pre.json" 2>&1
"$AB" --session $S click '.r28-feed-cta' > "$DATA/cta-wait-click.txt" 2>&1
sleep 1
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/cta-wait-post.json" 2>&1
"$AB" --session $S eval --stdin < "$E/timeline-eval.js" > "$DATA/cta-wait-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r30-07-cta-drawer.png" >/dev/null 2>&1
"$AB" --session $S click '.r29-timeline-close' > "$DATA/cta-wait-close.txt" 2>&1
sleep 1
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/cta-wait-closed.json" 2>&1
echo '=== SHA =='
sha256sum "$SH"/r30-*.png 2>/dev/null
echo '=== DONE ==='
