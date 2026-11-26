#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round26-next-wave-brief
S=lyp-r26-e2e
SH="$PWD/$D/screenshots"
DATA="$PWD/$D/data"
E="$PWD/$D"
mkdir -p "$SH" "$DATA"
echo '=== RESET SESSION ==='
"$AB" --session $S close >/dev/null 2>&1
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 5
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S reload >/dev/null 2>&1
sleep 5
echo '=== CARD_SELECT BRIEF CAPTURE ==='
: > "$DATA/brief-samples.jsonl"
SHOT1=0
SHOT2=0
for i in $(seq 1 90); do
  ST=$("$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1)
  echo "$ST" >> "$DATA/brief-samples.jsonl"
  echo "[$i] $(echo "$ST" | head -c 300)"
  ACT=$(echo "$ST" | grep -o '"act":"[A-Z_]*"' | head -1 | cut -d'"' -f4)
  if [ "$ACT" = "SETTLED" ]; then echo BATTLE_DONE; break; fi
  if [ "$SHOT1" = 0 ] && echo "$ST" | grep -q '第 2 波'; then "$AB" --session $S screenshot "$SH/r26-01-brief-wave2.png" >/dev/null 2>&1; SHOT1=1; echo SHOT1; fi
  if [ "$SHOT2" = 0 ] && echo "$ST" | grep -q '第 4 波'; then "$AB" --session $S screenshot "$SH/r26-02-brief-wave4.png" >/dev/null 2>&1; SHOT2=1; echo SHOT2;
    "$AB" --session $S eval --stdin < "$E/scroll-act.js" >/dev/null 2>&1
    sleep 0.5; "$AB" --session $S screenshot "$SH/r26-04-brief-scroll.png" >/dev/null 2>&1; echo SHOT4; fi
  if [ "$ACT" = "CARD_CLICK" ]; then
    "$AB" --session $S eval --stdin < "$E/card-act.js" >/dev/null 2>&1
  elif [ "$ACT" = "SKIP" ]; then
    "$AB" --session $S eval --stdin < "$E/skip-act.js" >/dev/null 2>&1
  fi
  sleep 0.7
done
echo '=== SETTLE STATE ==='
"$AB" --session $S eval --stdin < "$E/settle-eval.js" 2>&1
"$AB" --session $S screenshot "$SH/r26-03-settle.png" >/dev/null 2>&1
echo SHOT3
echo '=== SHA256 ==='
sha256sum "$SH"/r26-0*.png 2>/dev/null
echo '=== E2E DONE ==='