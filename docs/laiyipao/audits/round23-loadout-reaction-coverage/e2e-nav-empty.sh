#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round23-loadout-reaction-coverage
SH="$PWD/$D/screenshots"
mkdir -p "$SH" "$D/data"
echo '=== MINT UID 127 ==='
bash "$PWD/$D/mint-token.sh" 127
S=r23-empty
"$AB" --session "$S" close >/dev/null 2>&1
"$AB" --session "$S" open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/bag/bag" >/dev/null 2>&1
sleep 6
"$AB" --session "$S" reload >/dev/null 2>&1
sleep 6
"$AB" --session "$S" eval --stdin <<'EOF'
(() => {location.hash="#/pages/skills/skills";return "NAV"})()
EOF
sleep 5
"$AB" --session "$S" eval --stdin < "$PWD/$D/probe.js" > "$D/data/probe-empty.json" 2>&1
head -c 3000 "$D/data/probe-empty.json"
echo
"$AB" --session "$S" screenshot "$SH/r23-04-empty-user127.png" >/dev/null 2>&1
echo SHOT_EMPTY_DONE
echo '=== DONE EMPTY ==='
