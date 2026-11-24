#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round26-next-wave-brief
S=lyp-r26-diag
E="$PWD/$D"
"$AB" --session $S close >/dev/null 2>&1; "$AB" --session $S open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 6
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; "$AB" --session $S reload >/dev/null 2>&1; sleep 6
echo '=== BEFORE ==='
"$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1 | head -c 700
echo
echo '=== REAL CLICK hand-card ==='
"$AB" --session $S click '.hand-card' 2>&1 | head -c 300
echo
sleep 3
echo '=== AFTER CLICK ==='
"$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1 | head -c 700
echo
echo '=== SKIP x3 ==='
for k in 1 2 3; do
  "$AB" --session $S eval --stdin < "$E/skip-act.js" >/dev/null 2>&1; sleep 2;
  echo "skip$k: $("$AB" --session $S eval --stdin < "$E/probe-eval.js" 2>&1 | grep -o '"waveNow":"[0-9]*"' | head -1)";
done
echo DONE