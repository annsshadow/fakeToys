#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round29-live-reaction-timeline
S=lyp-r29-final
E="$PWD/$D"
SH="$PWD/$D/screenshots"
DATA="$PWD/$D/data"
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 8
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 8
: > "$DATA/samples-final.jsonl"
SH_FEED=0; SH_AI=0; DONE=0
for i in $(seq 1 260); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  "$AB" --session $S eval --stdin < "$E/probe-eval.js" >> "$DATA/samples-final.jsonl" 2>&1
  IFS='|' read -r ST LINES ROWS OV LEGEND CARD SKIP <<< "$TK"
  if [ "$SH_FEED" = 0 ] && [ "$LINES" -ge 3 ] 2>/dev/null; then "$AB" --session $S screenshot "$SH/r29-13-feed.png" >/dev/null 2>&1; SH_FEED=1; fi
  if [ "$SH_AI" = 0 ] && [ "$LINES" -ge 1 ] 2>/dev/null; then "$AB" --session $S screenshot "$SH/r29-14-feed-ai.png" >/dev/null 2>&1; SH_AI=1; fi
  if [ "$ST" = "SETTLED" ]; then DONE=1; echo SETTLED_AT=$i; break; fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1;
  elif [ "$ST" = "SKIP_ONLY" ]; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; fi
  sleep 0.35
done
"$AB" --session $S eval --stdin < "$E/final-eval.js" > "$DATA/final-final.json" 2>&1
"$AB" --session $S screenshot "$SH/r29-15-settle.png" >/dev/null 2>&1
echo "DONE=$DONE SH_FEED=$SH_FEED SH_AI=$SH_AI"
sha256sum "$SH"/r29-*.png 2>/dev/null | tail -20
echo END
