#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round30-reaction-feed-mobile-fit
S=lyp-r30-full
SH="$PWD/$D/screenshots"
DATA="$PWD/$D/data"
E="$PWD/$D"
mkdir -p "$SH" "$DATA"
echo '=== RESET ==='
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 8
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 8
echo '=== RUN ==='
: > "$DATA/samples.jsonl"
: > "$DATA/tick-log.txt"
SH_ROWS=0; SH_AI=0; SH_DRAWER=0; SH_SETTLE=0; SH_DRAWER_SETTLED=0
CARDCLICKS=0; SKIPS=0
for i in $(seq 1 300); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  PJ=$("$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1)
  echo "$PJ" >> "$DATA/samples.jsonl"
  echo "[$i] $TK" >> "$DATA/tick-log.txt"
  IFS='|' read -r ST LINES ROWS OV LEGEND CARD SKIP <<< "$TK"
  echo "[$i] $TK"
  if [ "$SH_ROWS" = 0 ] && [ "$LINES" -ge 3 ] 2>/dev/null; then
    "$AB" --session $S screenshot "$SH/r30-01-live-feed.png" >/dev/null 2>&1; SH_ROWS=1; echo SHOT_FEED; fi
  if [ "$SH_AI" = 0 ] && [ "$LINES" -ge 1 ] 2>/dev/null; then
    "$AB" --session $S screenshot "$SH/r30-02-feed-ai.png" >/dev/null 2>&1; SH_AI=1; echo SHOT_AI; fi
  if [ "$ST" = "SETTLED" ]; then echo BATTLE_DONE; break; fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1; CARDCLICKS=$((CARDCLICKS+1));
  elif [ "$ST" = "SKIP_ONLY" ]; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; SKIPS=$((SKIPS+1)); fi
  sleep 0.4
done
echo "CARD_CLICKS=$CARDCLICKS SKIPS=$SKIPS"
echo '=== DRAWER LIVE TEST ==='
"$AB" --session $S eval --stdin < "$E/click-cta-eval.js" > "$DATA/cta-hit.json" 2>&1
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/drawer-pre.json" 2>&1
"$AB" --session $S click '.r28-feed-cta' >/dev/null 2>&1
sleep 1
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/drawer-open.json" 2>&1
"$AB" --session $S eval --stdin < "$E/timeline-eval.js" > "$DATA/drawer-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r30-03-drawer-open.png" >/dev/null 2>&1; echo SHOT_DRAWER
"$AB" --session $S click '.r29-timeline-close' >/dev/null 2>&1 || "$AB" --session $S click '.r29-timeline-overlay .r30-close' >/dev/null 2>&1
sleep 1
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/drawer-closed.json" 2>&1
echo '=== FINAL ==='
"$AB" --session $S eval --stdin < "$E/final-eval.js" > "$DATA/final.json" 2>&1
"$AB" --session $S screenshot "$SH/r30-04-settle.png" >/dev/null 2>&1
echo '=== DRAWER SETTLED TEST ==='
"$AB" --session $S click '.r28-feed-cta' >/dev/null 2>&1
sleep 1
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/settled-drawer-open.json" 2>&1
"$AB" --session $S eval --stdin < "$E/timeline-eval.js" > "$DATA/settled-drawer-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r30-05-settled-drawer.png" >/dev/null 2>&1
echo '=== SHA ==='
sha256sum "$SH"/r30-*.png 2>/dev/null
echo '=== DONE ==='
