#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round23-loadout-reaction-coverage
S=r23-data
SH="$PWD/$D/screenshots"
mkdir -p "$SH" "$D/data"
echo '=== RESET DATA SESSION ==='
"$AB" --session "$S" close >/dev/null 2>&1
"$AB" --session "$S" open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/skills/skills" >/dev/null 2>&1
sleep 5
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" reload >/dev/null 2>&1
sleep 5
echo '=== PROBE COVERAGE CARD ==='
"$AB" --session "$S" eval --stdin < "$PWD/$D/probe.js" > "$D/data/probe-data.json" 2>&1
head -c 3000 "$D/data/probe-data.json"
echo
"$AB" --session "$S" screenshot "$SH/r23-01-skills-top.png" >/dev/null 2>&1
echo SHOT_TOP_DONE
echo '=== SCROLL COVERAGE CARD ==='
"$AB" --session "$S" eval --stdin <<'EOF'
(() => { const c=document.querySelector('.r23-coverage-card'); if(c){c.scrollIntoView({block:'center'});return 'SCROLLED'} return 'NO_CARD' })()
EOF
sleep 1
"$AB" --session "$S" screenshot "$SH/r23-02-coverage-card.png" >/dev/null 2>&1
echo SHOT_CARD_DONE
echo '=== DONE DATA ==='
