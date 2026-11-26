#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round26-next-wave-brief
S=lyp-r26-full2
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
: > "$DATA/brief-samples.jsonl"
: > "$DATA/state-log.txt"
SHOT1=0
SHOT2=0
SHOT4=0
CARDCLICKS=0
SKIPS=0
for i in $(seq 1 220); do
  ST=$("$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1)
  SV=$("$AB" --session $S eval --stdin < "$E/state-eval.js" 2>&1)
  echo "$ST" >> "$DATA/brief-samples.jsonl"
  echo "[$i] $SV" >> "$DATA/state-log.txt"
  echo "[$i] $(echo "$SV" | head -c 250)"
  if echo "$SV" | grep -q SETTLED; then echo BATTLE_DONE; break; fi
  if [ "$SHOT1" = 0 ] && echo "$ST" | grep -q '第 2 波'; then "$AB" --session $S screenshot "$SH/r26-01-brief-wave2.png" >/dev/null 2>&1; SHOT1=1; echo SHOT1; fi
  if [ "$SHOT2" = 0 ] && echo "$ST" | grep -q '第 4 波'; then "$AB" --session $S screenshot "$SH/r26-02-brief-wave4.png" >/dev/null 2>&1; SHOT2=1; echo SHOT2;
    "$AB" --session $S eval --stdin < "$E/scroll-act.js" >/dev/null 2>&1; sleep 0.3;
    "$AB" --session $S screenshot "$SH/r26-04-brief-scroll.png" >/dev/null 2>&1; SHOT4=1; echo SHOT4; fi
  if echo "$SV" | grep -q CARD_PHASE; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1; CARDCLICKS=$((CARDCLICKS+1));
  elif echo "$SV" | grep -q SKIP_ONLY; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; SKIPS=$((SKIPS+1)); fi
  sleep 0.5
done
echo "CARD_CLICKS=$CARDCLICKS SKIPS=$SKIPS SHOT1=$SHOT1 SHOT2=$SHOT2 SHOT4=$SHOT4"
echo '=== FINAL ==='
"$AB" --session $S eval --stdin < "$E/final-eval.js" 2>&1
"$AB" --session $S screenshot "$SH/r26-03-settle.png" >/dev/null 2>&1
echo SHOT_SETTLE
sha256sum "$SH"/r26-0*.png 2>/dev/null
echo '=== DONE ==='