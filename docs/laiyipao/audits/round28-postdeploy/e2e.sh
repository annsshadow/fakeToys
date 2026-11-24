#!/bin/bash
set -u
cd D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248
AB=agent-browser
D=docs/laiyipao/audits/round28-postdeploy
E="$PWD/$D"
SH="$E/screenshots"
DATA="$E/data"
S=lyp-r28-postdeploy
mkdir -p "$SH" "$DATA"
echo '=== MINT ==='
bash "$E/mint-token.sh" || exit 1
echo '=== RESET ==='
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$E/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 6
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S reload >/dev/null 2>&1
sleep 6
echo '=== BATTLE LOOP ==='
: > "$DATA/samples.jsonl"
: > "$DATA/tick-log.txt"
SH_MULTI=0; SH_AI=0; CTA_OPEN=0; CARDCLICKS=0; SKIPS=0
for i in $(seq 1 300); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  PJ=$("$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1)
  echo "$PJ" >> "$DATA/samples.jsonl"
  echo "[$i] $TK" >> "$DATA/tick-log.txt"
  IFS='|' read -r ST LINES AVAIL RATIO LEGEND CARD SKIP <<< "$TK"
  echo "[$i] $ST lines=$LINES avail=$AVAIL ratio=$RATIO"
  if [ "$SH_MULTI" = 0 ] && [ "$LINES" -ge 3 ] 2>/dev/null && [ "$AVAIL" -ge 2 ] 2>/dev/null; then
    "$AB" --session $S screenshot "$SH/postdeploy-r28-01-attribution.png" >/dev/null 2>&1
    SH_MULTI=1; echo SHOT_MULTI
  fi
  if [ "$SH_AI" = 0 ] && [ "$LINES" -ge 1 ] 2>/dev/null; then
    "$AB" --session $S screenshot "$SH/postdeploy-r28-02-ai.png" >/dev/null 2>&1
    SH_AI=1; echo SHOT_AI
  fi
  if [ "$CTA_OPEN" = 0 ] && [ "$ST" = "WAIT" ] && [ "$LEGEND" = "0" ] && [ "$LINES" -ge 1 ] 2>/dev/null; then
    "$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/cta-pre.json" 2>&1
    "$AB" --session $S click '.r28-feed-cta' >/dev/null 2>&1
    sleep 1
    "$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/cta-post.json" 2>&1
    "$AB" --session $S screenshot "$SH/postdeploy-r28-03-cta-open.png" >/dev/null 2>&1
    "$AB" --session $S click '.legend-close' >/dev/null 2>&1
    sleep 1
    "$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/cta-closed.json" 2>&1
    CTA_OPEN=1; echo CTA_TESTED
  fi
  if [ "$ST" = "SETTLED" ]; then echo BATTLE_DONE; break; fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1; CARDCLICKS=$((CARDCLICKS+1));
  elif [ "$ST" = "SKIP_ONLY" ]; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; SKIPS=$((SKIPS+1)); fi
  sleep 0.5
done
echo "CARD_CLICKS=$CARDCLICKS SKIPS=$SKIPS MULTI=$SH_MULTI AI=$SH_AI CTA=$CTA_OPEN"
"$AB" --session $S eval --stdin < "$E/final-eval.js" > "$DATA/final.json" 2>&1
"$AB" --session $S screenshot "$SH/postdeploy-r28-04-settle.png" >/dev/null 2>&1
echo SHOT_SETTLE
"$AB" --session $S close >/dev/null 2>&1 || true
echo '=== SHA ==='
sha256sum "$SH"/*.png 2>/dev/null
echo '=== DONE ==='
