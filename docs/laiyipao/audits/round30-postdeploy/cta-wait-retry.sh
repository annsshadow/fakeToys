#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round30-postdeploy
S=lyp-r30-waitcta
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
CLICKED=0
for i in $(seq 1 140); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  IFS='|' read -r ST LINES ROWS OV LEGEND CARD SKIP <<< "$TK"
  echo "[$i] $TK"
  if [ "$ST" = "WAIT" ] && [ "$LINES" -ge 1 ] 2>/dev/null; then
    "$AB" --session $S eval --stdin < "$E/click-cta-eval.js" > "$DATA/wait-cta-hit.json" 2>&1
    HIT=$(cat "$DATA/wait-cta-hit.json")
    echo "HIT_TRY=$i $HIT"
    case "$HIT" in
      *r28-feed-cta*)
        "$AB" --session $S click '.r28-feed-cta' > "$DATA/wait-cta-click.txt" 2>&1
        echo "CLICK_TRY=$i $(cat "$DATA/wait-cta-click.txt")"
        sleep 0.6
        "$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/wait-cta-open-probe.json" 2>&1
        "$AB" --session $S eval --stdin < "$E/timeline-eval.js" > "$DATA/wait-cta-open-geometry.json" 2>&1
        OPEN=$(node -e 'var fs=require("fs");var s=fs.readFileSync(process.argv[1],"utf8");var o=JSON.parse(s);if(typeof o==="string")o=JSON.parse(o);process.stdout.write(String(o.timelineOpen))' "$DATA/wait-cta-open-probe.json")
        if [ "$OPEN" = "true" ]; then
          "$AB" --session $S screenshot "$SH/r30post-08-wait-cta.png" >/dev/null 2>&1
          "$AB" --session $S click '.r29-timeline-close' > "$DATA/wait-cta-close.txt" 2>&1
          sleep 0.5
          "$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/wait-cta-closed-probe.json" 2>&1
          CLICKED=1; echo CTA_OPENED; break
        fi
        ;;
    esac
  fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1;
  elif [ "$ST" = "SKIP_ONLY" ]; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; fi
  sleep 0.25
done
echo CLICKED=$CLICKED
"$AB" --session $S close >/dev/null 2>&1 || true
if [ "$CLICKED" = "1" ]; then sha256sum "$SH/r30post-08-wait-cta.png"; fi
echo DONE
