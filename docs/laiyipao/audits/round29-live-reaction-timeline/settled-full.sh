#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round29-live-reaction-timeline
S=lyp-r29-sfull
E="$PWD/$D"
SH="$PWD/$D/screenshots"
DATA="$PWD/$D/data"
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 8
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 8
for i in $(seq 1 300); do
  TK=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"')
  IFS='|' read -r ST LINES ROWS OV LEGEND CARD SKIP <<< "$TK"
  if [ "$ST" = "SETTLED" ]; then echo "SETTLED_AT=$i"; break; fi
  if [ "$ST" = "CARD_PHASE" ]; then "$AB" --session $S click '.hand-card' >/dev/null 2>&1;
  elif [ "$ST" = "SKIP_ONLY" ]; then "$AB" --session $S click '.btn:has-text("全部不要了")' >/dev/null 2>&1; fi
  sleep 0.35
done
"$AB" --session $S eval --stdin < "$E/final-eval.js" > "$DATA/final-sfull.json" 2>&1
"$AB" --session $S screenshot "$SH/r29-08-settled-before-drawer.png" >/dev/null 2>&1
echo '=== SETTLED DRAWER OPEN via DOM click (result sheet covers CTA) ==='
"$AB" --session $S eval '(()=>{var e=document.querySelector(".r28-feed-cta");if(e){e.click();return "clicked"}return "no-cta"})()' > "$DATA/settled-open-click.json" 2>&1
sleep 0.6
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/settled-open-probe.json" 2>&1
"$AB" --session $S eval --stdin < "$E/timeline-eval.js" > "$DATA/settled-open-geometry.json" 2>&1
"$AB" --session $S screenshot "$SH/r29-09-settled-drawer.png" >/dev/null 2>&1
echo SHOT_SETTLED_DRAWER
echo '=== SCROLL TEST ==='
"$AB" --session $S eval '(()=>{var s=document.querySelector(".r29-timeline-scroll");if(!s)return JSON.stringify({ok:false});s.scrollTop=s.scrollHeight;var a=s.scrollTop;s.scrollTop=0;return JSON.stringify({ok:true,max:a,h:s.scrollHeight,ch:s.clientHeight})})()' > "$DATA/settled-scroll.json" 2>&1
echo '=== CLOSE ==='
"$AB" --session $S click '.r29-timeline-close' >/dev/null 2>&1
sleep 0.4
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/settled-closed-probe.json" 2>&1
echo '=== DONE ==='
