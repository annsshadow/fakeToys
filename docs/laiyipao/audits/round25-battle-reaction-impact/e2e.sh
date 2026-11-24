#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round25-battle-reaction-impact
S=lyp-r25-e2e
SH="$PWD/$D/screenshots"
echo '=== DATA STATE battle 378 ==='
"$AB" --session $S close >/dev/null 2>&1
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battles/battles?battle_id=378" >/dev/null 2>&1
sleep 6
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S reload >/dev/null 2>&1
sleep 6
echo '--- PROBE 378 ---'
"$AB" --session $S eval --stdin < "$D/probe.js" > "$D/data/probe-378.json" 2>&1
head -c 3000 "$D/data/probe-378.json"
"$AB" --session $S screenshot "$SH/r25-01-impact-378.png" >/dev/null 2>&1
echo SHOT_01
"$AB" --session $S eval --stdin <<'EOF'
(() => { var el=document.querySelector('.r25-reaction-impact'); if(el){el.scrollIntoView({block:'center'});return 'SCROLLED'} return 'NO_CARD' })()
EOF
sleep 1
"$AB" --session $S screenshot "$SH/r25-02-impact-detail.png" >/dev/null 2>&1
echo SHOT_02
echo '=== OLD STATE battle 339 ==='
"$AB" --session $S close >/dev/null 2>&1
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battles/battles?battle_id=339" >/dev/null 2>&1
sleep 6
"$AB" --session $S reload >/dev/null 2>&1
sleep 6
"$AB" --session $S eval --stdin < "$D/probe.js" > "$D/data/probe-339.json" 2>&1
head -c 3000 "$D/data/probe-339.json"
"$AB" --session $S screenshot "$SH/r25-03-old-empty.png" >/dev/null 2>&1
echo SHOT_03
echo '=== DONE ==='
