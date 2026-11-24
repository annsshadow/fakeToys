#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round30-postdeploy
S=lyp-r30-postdeploy
E="$PWD/$D"
SH="$PWD/$D/screenshots"
DATA="$PWD/$D/data"
mkdir -p "$SH" "$DATA"
echo "=== RESET ==="
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$E/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 8
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 8
: > "$DATA/samples.jsonl"
: > "$DATA/tick-log.txt"
MEASURED=0; CARDCLICKS=0; SKIPS=0; SETTLED=0
for i in $(seq 1 320); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  "$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/samples.jsonl.tmp" 2>&1
  cat "$DATA/samples.jsonl.tmp" >> "$DATA/samples.jsonl"
  echo "[$i] $TK" >> "$DATA/tick-log.txt"
  IFS='|' read -r ST LINES ROWS OV LEGEND CARD SKIP <<< "$TK"
  echo "[$i] $TK"
  if [ "$MEASURED" = 0 ] && [ "$LINES" -ge 3 ] 2>/dev/null && [ "$ST" = "WAIT" ]; then
    "$AB" --session $S eval --stdin < "$E/measure-eval.js" > "$DATA/measure.json" 2>&1
    "$AB" --session $S screenshot "$SH/r30post-01-normal-three-lines.png" >/dev/null 2>&1; echo SHOT_01
    "$AB" --session $S eval --stdin < "$E/stress-apply-eval.js" > "$DATA/stress-applied.json" 2>&1
    "$AB" --session $S screenshot "$SH/r30post-02-stress-longtext.png" >/dev/null 2>&1; echo SHOT_02
    "$AB" --session $S eval --stdin < "$E/measure-eval.js" > "$DATA/stress-measure.json" 2>&1
    "$AB" --session $S eval --stdin < "$E/stress-restore-eval.js" > "$DATA/stress-restored.json" 2>&1
    "$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/cta-wait-pre.json" 2>&1
    "$AB" --session $S click '.r28-feed-cta' > "$DATA/cta-wait-click.txt" 2>&1
    sleep 0.8
    "$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/cta-wait-post.json" 2>&1
    "$AB" --session $S eval --stdin < "$E/timeline-eval.js" > "$DATA/cta-wait-geometry.json" 2>&1
    "$AB" --session $S screenshot "$SH/r30post-03-wait-drawer.png" >/dev/null 2>&1; echo SHOT_03
    "$AB" --session $S click '.r29-timeline-close' > "$DATA/cta-wait-close.txt" 2>&1
    sleep 0.8
    "$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/cta-wait-closed.json" 2>&1
    MEASURED=1; echo PRESSURE_CTA_DONE
  fi
  if [ "$ST" = "SETTLED" ]; then SETTLED=1; echo BATTLE_DONE; break; fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1; CARDCLICKS=$((CARDCLICKS+1));
  elif [ "$ST" = "SKIP_ONLY" ]; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; SKIPS=$((SKIPS+1)); fi
  sleep 0.4
done
echo "CARD_CLICKS=$CARDCLICKS SKIPS=$SKIPS SETTLED=$SETTLED MEASURED=$MEASURED"
"$AB" --session $S eval --stdin < "$E/final-eval.js" > "$DATA/final.json" 2>&1
"$AB" --session $S screenshot "$SH/r30post-04-settle.png" >/dev/null 2>&1; echo SHOT_04
"$AB" --session $S eval --stdin < "$E/result-cta-eval.js" > "$DATA/result-cta-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r30post-05-result-entry.png" >/dev/null 2>&1; echo SHOT_05
"$AB" --session $S click '.r29-result-timeline-cta' > "$DATA/result-cta-click.txt" 2>&1; echo "CLICK_RC=$?"
sleep 0.8
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/result-open-probe.json" 2>&1
"$AB" --session $S eval --stdin < "$E/timeline-eval.js" > "$DATA/result-open-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r30post-06-result-drawer.png" >/dev/null 2>&1; echo SHOT_06
"$AB" --session $S click '.r29-timeline-close' > "$DATA/result-close-click.txt" 2>&1
sleep 0.6
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/result-closed-probe.json" 2>&1
"$AB" --session $S screenshot "$SH/r30post-07-result-closed.png" >/dev/null 2>&1; echo SHOT_07
"$AB" --session $S click '.r29-result-timeline-cta' > "$DATA/reopen-click.txt" 2>&1
sleep 0.6
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/reopen-probe.json" 2>&1
echo "=== SHA ==="
sha256sum "$SH"/r30post-*.png
echo "=== DONE ==="
