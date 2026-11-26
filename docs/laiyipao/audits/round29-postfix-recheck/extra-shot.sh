#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round29-postfix-recheck
S=lyp-r29-postfix
E="$PWD/$D"
SH="$PWD/$D/screenshots"
DATA="$PWD/$D/data"
echo '=== REOPEN VIA REAL CLI CLICK ==='
"$AB" --session $S click '.r29-result-timeline-cta' > "$DATA/reopen-click.txt" 2>&1
echo "REOPEN_RC=$?"
sleep 0.6
"$AB" --session $S eval --stdin < "$E/scroll-probe.js" > "$DATA/reopen-scroll.json" 2>&1
"$AB" --session $S screenshot "$SH/r29p-04-scroll-last.png" >/dev/null 2>&1
echo SHOT_04
"$AB" --session $S eval --stdin < "$E/probe-eval.js" > "$DATA/reopen-probe.json" 2>&1
echo '=== DONE ==='
