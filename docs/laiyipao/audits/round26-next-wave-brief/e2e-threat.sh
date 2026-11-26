#!/bin/bash
AB=agent-browser; S=lyp-r26-threat; E=docs/laiyipao/audits/round26-next-wave-brief
SH="$PWD/$E/screenshots"; mkdir -p "$SH"
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$E/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=22" >/dev/null 2>&1
sleep 6
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 6
for i in $(seq 1 120); do
  SV=$("$AB" --session $S eval --stdin < "$E/state-eval.js" 2>&1)
  echo "[$i] $(echo "$SV" | head -c 200)"
  if echo "$SV" | grep -q CARD_PHASE; then
    "$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1 | head -c 2000
    "$AB" --session $S screenshot "$SH/r26-05-brief-threat.png" >/dev/null 2>&1; echo SHOT_THREAT; break; fi
  if echo "$SV" | grep -q SETTLED; then echo SETTLED_NO_CARD; break; fi
  sleep 0.5
done
echo '=== THREAT DONE ==='