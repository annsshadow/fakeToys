#!/bin/bash
AB=agent-browser; S=lyp-r26-threat2; E=docs/laiyipao/audits/round26-next-wave-brief
SH="$PWD/$E/screenshots"; DATA="$PWD/$E/data"; mkdir -p "$SH" "$DATA"
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$E/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=22" >/dev/null 2>&1
sleep 6
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 6
: > "$DATA/threat-samples.jsonl"
SHOT=0
for i in $(seq 1 160); do
  SV=$("$AB" --session $S eval --stdin < "$E/state-eval.js" 2>&1)
  ST=$("$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1)
  echo "$ST" >> "$DATA/threat-samples.jsonl"
  echo "[$i] $(echo "$SV" | head -c 200)"
  if echo "$SV" | grep -q SETTLED; then echo SETTLED; break; fi
  if echo "$SV" | grep -q CARD_PHASE; then
    if [ "$SHOT" = 0 ] && echo "$ST" | grep -q '第 4 波'; then
      echo "THREAT_BRIEF: $(echo "$ST" | head -c 1400)"
      "$AB" --session $S screenshot "$SH/r26-06-brief-threat-tag.png" >/dev/null 2>&1; SHOT=1; echo SHOT_THREAT_TAG; fi
    "$AB" --session $S click '.hand-card' >/dev/null 2>&1; fi
  sleep 0.5
done
echo "SHOT=$SHOT"; echo DONE