#!/bin/bash
AB=agent-browser; S=lyp-r26-threat3
BASE='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248'
D="$BASE/docs/laiyipao/audits/round26-next-wave-brief"
SH="$D/screenshots"; DATA="$D/data"; E="$D"
mkdir -p "$SH" "$DATA"
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$E/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=22" >/dev/null 2>&1
sleep 6
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 6
: > "$DATA/threat-v2-samples.jsonl"
SHOT=0
for i in $(seq 1 200); do
  ST=$("$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1)
  SV=$("$AB" --session $S eval --stdin < "$E/state-eval.js" 2>&1)
  echo "$ST" >> "$DATA/threat-v2-samples.jsonl"
  echo "[$i] $(echo "$SV" | head -c 180)"
  if echo "$ST" | grep -qE '\[(远程|飞行|潜地|首领)\]'; then
    if [ "$SHOT" = 0 ]; then
      "$AB" --session $S screenshot "$SH/r26-06-brief-threat-tag.png" >/dev/null 2>&1
      SHOT=1; echo "THREAT_TAG_FOUND [$i]: $(echo "$ST" | head -c 1400)"; echo SHOT_THREAT; break
    fi
  fi
  if echo "$SV" | grep -q SETTLED; then echo SETTLED; break; fi
  if echo "$SV" | grep -q CARD_PHASE; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1
  elif echo "$SV" | grep -q SKIP_ONLY; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; fi
  sleep 0.4
done
echo "SHOT=$SHOT"
sha256sum "$SH/r26-06-brief-threat-tag.png" 2>/dev/null
echo DONE