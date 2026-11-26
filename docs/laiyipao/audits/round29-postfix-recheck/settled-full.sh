#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round29-postfix-recheck
S=lyp-r29-postfix
E="$PWD/$D"
SH="$PWD/$D/screenshots"
DATA="$PWD/$D/data"
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 8
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 8
for i in $(seq 1 320); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  IFS='|' read -r ST LINES ROWS OV LEGEND CARD SKIP <<< "$TK"
  if [ "$ST" = "SETTLED" ]; then echo "SETTLED_AT=$i"; break; fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1;
  elif [ "$ST" = "SKIP_ONLY" ]; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; fi
  sleep 0.35
done
"$AB" --session $S eval --stdin < "$E/final-eval.js" > "$DATA/final.json" 2>&1
echo '=== RESULT ENTRY BEFORE CLICK ==='
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/result-before-probe.json" 2>&1
"$AB" --session $S eval --stdin < "$E/result-cta-eval.js" > "$DATA/result-cta-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r29p-01-result-entry.png" >/dev/null 2>&1
echo SHOT_01
echo '=== REAL CLI CLICK .r29-result-timeline-cta ==='
"$AB" --session $S click '.r29-result-timeline-cta' > "$DATA/result-cta-click.txt" 2>&1
echo "CLICK_RC=$?"
sleep 0.8
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/result-open-probe.json" 2>&1
"$AB" --session $S eval --stdin < "$E/timeline-eval.js" > "$DATA/result-open-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r29p-02-result-drawer.png" >/dev/null 2>&1
echo SHOT_02
echo '=== REAL CLI CLICK .r29-timeline-close ==='
"$AB" --session $S click '.r29-timeline-close' > "$DATA/result-close-click.txt" 2>&1
echo "CLOSE_RC=$?"
sleep 0.5
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/result-closed-probe.json" 2>&1
"$AB" --session $S screenshot "$SH/r29p-03-result-closed.png" >/dev/null 2>&1
echo SHOT_03
echo '=== DONE ==='
