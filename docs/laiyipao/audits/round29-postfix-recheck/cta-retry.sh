#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round29-postfix-recheck
S=lyp-r29-cta2
E="$PWD/$D"
SH="$PWD/$D/screenshots"
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 8
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 8
CLICKED=0
for i in $(seq 1 120); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  IFS='|' read -r ST LINES ROWS OV LEGEND CARD SKIP <<< "$TK"
  if [ "$ST" = "WAIT" ] && [ "$LINES" -ge 1 ] 2>/dev/null; then
    OUT=$("$AB" --session $S click '.r28-feed-cta' 2>&1)
    echo "CLICK_TRY=$i $OUT"
    "$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$D/data/cta2-probe.json" 2>&1
    OPEN=$(node -e 'var fs=require("fs");var s=fs.readFileSync(process.argv[1],"utf8");var o=JSON.parse(s);if(typeof o==="string")o=JSON.parse(o);process.stdout.write(String(o.timelineOpen))' "$D/data/cta2-probe.json")
    if [ "$OPEN" = "true" ]; then CLICKED=1; echo CTA_OPENED; break; fi
  fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1;
  elif [ "$ST" = "SKIP_ONLY" ]; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; fi
  sleep 0.25
done
echo CLICKED=$CLICKED
"$AB" --session $S eval --stdin < "$E/timeline-eval.js" > "$D/data/cta2-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r29-10-drawer-conclusion.png" >/dev/null 2>&1
echo DONE
