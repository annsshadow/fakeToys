#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round29-live-reaction-timeline
S=lyp-r29-own
E="$PWD/$D"
"$AB" --session $S close >/dev/null 2>&1 || true
"$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 8
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 8
for i in $(seq 1 40); do ST=$("$AB" --session $S eval --stdin < "$E/tick-eval.js" 2>&1 | tr -d '"'); [ "$(echo "$ST"|cut -d'|' -f2)" -ge 1 ] 2>/dev/null && break; sleep 0.3; done
snap(){ sleep 0.3; "$AB" --session $S eval --stdin < "$E/ownership-eval.js" > "$D/data/$1" 2>&1; }
snap own-0-before.json
echo '=== MANUAL PAUSE ==='
"$AB" --session $S click '.pause-btn' >/dev/null 2>&1; snap own-1-manual-pause.json
echo '=== OPEN DRAWER while manual paused (DOM click due pause overlay) ==='
"$AB" --session $S eval '(()=>{var e=document.querySelector(".r28-feed-cta");if(e)e.click();return !!e})()' >/dev/null 2>&1; snap own-2-drawer-open.json
echo '=== CLOSE DRAWER ==='
"$AB" --session $S click '.r29-timeline-close' >/dev/null 2>&1; snap own-3-drawer-closed.json
echo '=== RESUME MANUAL ==='
"$AB" --session $S click '.pause-go' >/dev/null 2>&1; snap own-4-manual-resume.json
echo '=== AUTOPAUSE DRAWER ==='
"$AB" --session $S click '.r28-feed-cta' >/dev/null 2>&1; snap own-5-autopause.json
echo '=== TRY RESUME WHILE DRAWER (overlay shields pause button) ==='
"$AB" --session $S eval '(()=>{var e=document.querySelector(".pause-go");if(e)e.click();return !!e})()' >/dev/null 2>&1; snap own-6-resume-while-drawer.json
echo '=== CLOSE DRAWER (should auto resume) ==='
"$AB" --session $S click '.r29-timeline-close' >/dev/null 2>&1; snap own-7-autoresume.json
echo '=== MUTUAL EXCLUSION: click R27 CTA under drawer ==='
"$AB" --session $S click '.r28-feed-cta' >/dev/null 2>&1; sleep 0.3; snap own-8-drawer-again.json
"$AB" --session $S eval '(()=>{var e=document.querySelector(".r27-ai-cta");if(e)e.click();return !!e})()' >/dev/null 2>&1; snap own-9-after-r27-cta.json
echo '=== DONE ==='
