#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round14-advice-action-e2e
S=lyp-r14-rxguest
SH="$PWD/$D/screenshots"
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 2
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S open http://127.0.0.1:5174/#/pages/reactions/reactions >/dev/null 2>&1
sleep 3
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S reload >/dev/null 2>&1
sleep 7
echo '=== GUEST EMPTY DOM ==='
"$AB" --session $S eval --stdin 2>&1 < "$PWD/$D/rx-state-probe.js"
"$AB" --session $S screenshot "$SH/r14-rxfix-04-guest-empty.png" >/dev/null 2>&1
echo SHOT4=$?