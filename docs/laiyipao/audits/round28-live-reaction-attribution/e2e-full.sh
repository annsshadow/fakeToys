#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round28-live-reaction-attribution
S=lyp-r28-full
SH="$PWD/$D/screenshots"
DATA="$PWD/$D/data"
E="$PWD/$D"
mkdir -p "$SH" "$DATA"
echo '=== RESET ==='
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 6
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 6
echo '=== RUN ==='
: > "$DATA/samples.jsonl"
: > "$DATA/tick-log.txt"
SH_MULTI=0; SH_AI=0; SH_CTA=0
CARDCLICKS=0; SKIPS=0
for i in $(seq 1 260); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  PJ=$("$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1)
  echo "$PJ" >> "$DATA/samples.jsonl"
  echo "[$i] $TK" >> "$DATA/tick-log.txt"
  IFS='|' read -r ST LINES AVAIL RATIO LEGEND CARD SKIP <<< "$TK"
  echo "[$i] $TK"
  if [ "$SH_MULTI" = 0 ] && [ "$LINES" -ge 3 ] 2>/dev/null && [ "$AVAIL" -ge 2 ] 2>/dev/null; then
    "$AB" --session $S screenshot "$SH/r28-01-attribution-multi.png" >/dev/null 2>&1; SH_MULTI=1; echo SHOT_MULTI; fi
  if [ "$SH_AI" = 0 ] && [ "$LINES" -ge 1 ] 2>/dev/null; then
    "$AB" --session $S screenshot "$SH/r28-02-feed-ai.png" >/dev/null 2>&1; SH_AI=1; echo SHOT_AI; fi
  if [ "$ST" = "SETTLED" ]; then echo BATTLE_DONE; break; fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1; CARDCLICKS=$((CARDCLICKS+1));
  elif [ "$ST" = "SKIP_ONLY" ]; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; SKIPS=$((SKIPS+1)); fi
  sleep 0.5
done
echo "CARD_CLICKS=$CARDCLICKS SKIPS=$SKIPS MULTI=$SH_MULTI AI=$SH_AI"
echo '=== CTA TEST ==='
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/pre-cta-probe.json" 2>&1
"$AB" --session $S click '.r28-feed-cta' >/dev/null 2>&1
sleep 1
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/post-cta-probe.json" 2>&1
"$AB" --session $S screenshot "$SH/r28-03-cta-help.png" >/dev/null 2>&1; echo SHOT_CTA
"$AB" --session $S click '.legend-close' >/dev/null 2>&1
echo '=== FINAL ==='
"$AB" --session $S eval --stdin < "$E/final-eval.js" > "$DATA/final.json" 2>&1
"$AB" --session $S screenshot "$SH/r28-04-settle.png" >/dev/null 2>&1; echo SHOT_SETTLE
sha256sum "$SH"/r28-*.png 2>/dev/null
echo '=== DONE ==='
