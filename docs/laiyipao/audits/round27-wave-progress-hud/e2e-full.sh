#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round27-wave-progress-hud
S=lyp-r27-full
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
SH_W1=0; SH_MID=0; SH_CLEAR=0; SH_NEXT=0
CARDCLICKS=0; SKIPS=0
LASTIDX=-1
for i in $(seq 1 260); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  PJ=$("$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1)
  echo "$PJ" >> "$DATA/samples.jsonl"
  echo "[$i] $TK" >> "$DATA/tick-log.txt"
  IFS='|' read -r ST IDX TOT PCTTAG CARD SKIP DOMPCT <<< "$TK"
  echo "[$i] $TK"
  if [ "$SH_W1" = 0 ] && [ "$ST" = "WAIT" ] && [ "$IDX" = "0" ] && [ "$TOT" -gt 0 ] 2>/dev/null; then
    "$AB" --session $S screenshot "$SH/r27-01-first-wave.png" >/dev/null 2>&1; SH_W1=1; echo SHOT_W1; fi
  if [ "$SH_MID" = 0 ] && [ "$ST" = "WAIT" ] && [ "$PCTTAG" = "MID" ]; then
    "$AB" --session $S screenshot "$SH/r27-02-mid-wave.png" >/dev/null 2>&1; SH_MID=1; echo SHOT_MID; fi
  if [ "$SH_CLEAR" = 0 ] && [ "$ST" = "CARD_PHASE" ] && [ "$PCTTAG" = "FULL" ]; then
    "$AB" --session $S screenshot "$SH/r27-03-clear-100.png" >/dev/null 2>&1; SH_CLEAR=1; echo SHOT_CLEAR; fi
  if [ "$SH_CLEAR" = 1 ] && [ "$SH_NEXT" = 0 ] && [ "$ST" = "WAIT" ] && [ "$IDX" != "$LASTIDX" ] && [ "$IDX" != "0" ] && [ "$PCTTAG" != "FULL" ]; then
    "$AB" --session $S screenshot "$SH/r27-04-next-wave.png" >/dev/null 2>&1; SH_NEXT=1; echo SHOT_NEXT; fi
  LASTIDX="$IDX"
  if [ "$ST" = "SETTLED" ]; then echo BATTLE_DONE; break; fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1; CARDCLICKS=$((CARDCLICKS+1));
  elif [ "$ST" = "SKIP_ONLY" ]; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; SKIPS=$((SKIPS+1)); fi
  sleep 0.5
done
echo "CARD_CLICKS=$CARDCLICKS SKIPS=$SKIPS W1=$SH_W1 MID=$SH_MID CLEAR=$SH_CLEAR NEXT=$SH_NEXT"
echo '=== FINAL ==='
"$AB" --session $S eval --stdin < "$E/state-eval.js" 2>&1
"$AB" --session $S screenshot "$SH/r27-05-settle.png" >/dev/null 2>&1
echo SHOT_SETTLE
sha256sum "$SH"/r27-*.png 2>/dev/null
echo '=== DONE ==='
