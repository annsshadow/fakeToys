#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round29-postdeploy
S=lyp-r29-postdeploy
E="$PWD/$D"
SH="$PWD/$D/screenshots"
DATA="$PWD/$D/data"
mkdir -p "$SH" "$DATA"
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$E/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 8
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 8
: > "$DATA/tick-log.txt"; : > "$DATA/samples.jsonl"
CARD_CLICKS=0; SKIPS=0; SHOT_FEED=0
for i in $(seq 1 320); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  "$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/samples.jsonl.tmp" 2>&1
  cat "$DATA/samples.jsonl.tmp" >> "$DATA/samples.jsonl"
  echo "[$i] $TK" >> "$DATA/tick-log.txt"
  IFS='|' read -r ST LINES ROWS OV LEGEND CARD SKIP <<< "$TK"
  echo "[$i] $TK"
  if [ "$SHOT_FEED" = 0 ] && [ "$LINES" -ge 1 ] 2>/dev/null; then
    "$AB" --session $S screenshot "$SH/r29post-01-live.png" >/dev/null 2>&1; SHOT_FEED=1; echo SHOT_01; fi
  if [ "$ST" = "SETTLED" ]; then echo BATTLE_DONE; break; fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1; CARD_CLICKS=$((CARD_CLICKS+1));
  elif [ "$ST" = "SKIP_ONLY" ]; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; SKIPS=$((SKIPS+1)); fi
  sleep 0.35
done
echo "CARD_CLICKS=$CARD_CLICKS SKIPS=$SKIPS"
"$AB" --session $S eval --stdin < "$E/final-eval.js" > "$DATA/final.json" 2>&1
"$AB" --session $S eval --stdin < "$E/result-cta-eval.js" > "$DATA/result-cta-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r29post-02-result-entry.png" >/dev/null 2>&1; echo SHOT_02
"$AB" --session $S click '.r29-result-timeline-cta' > "$DATA/result-cta-click.txt" 2>&1; echo "CLICK_RC=$?"
sleep 0.8
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/result-open-probe.json" 2>&1
"$AB" --session $S eval --stdin < "$E/timeline-eval.js" > "$DATA/result-open-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r29post-03-result-drawer.png" >/dev/null 2>&1; echo SHOT_03
"$AB" --session $S click '.r29-timeline-close' > "$DATA/result-close-click.txt" 2>&1
sleep 0.5
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/result-closed-probe.json" 2>&1
"$AB" --session $S screenshot "$SH/r29post-04-result-closed.png" >/dev/null 2>&1; echo SHOT_04
"$AB" --session $S click '.r29-result-timeline-cta' > "$DATA/reopen-click.txt" 2>&1
sleep 0.6
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/reopen-probe.json" 2>&1
"$AB" --session $S screenshot "$SH/r29post-05-reopen.png" >/dev/null 2>&1; echo SHOT_05
echo '=== SHA ==='
sha256sum "$SH"/r29post-*.png
echo '=== DONE ==='
