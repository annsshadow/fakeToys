#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round23-loadout-reaction-coverage
S=r23-nav
SH="$PWD/$D/screenshots"
mkdir -p "$SH" "$D/data"
echo '=== NAV SESSION RESET ==='
"$AB" --session "$S" close >/dev/null 2>&1
"$AB" --session "$S" open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/bag/bag" >/dev/null 2>&1
sleep 6
"$AB" --session "$S" reload >/dev/null 2>&1
sleep 6
echo '=== CHECK LOADOUT LOADED IN BAG ==='
"$AB" --session "$S" eval --stdin <<'EOF'
(() => {var p=(window.__R23_PROBE__||{});return JSON.stringify({me:p.meResponses,url:location.href})})()
EOF
echo '=== NAV TO SKILLS VIA HASH ==='
"$AB" --session "$S" eval --stdin <<'EOF'
(() => {location.hash="#/pages/skills/skills";return "NAV"})()
EOF
sleep 5
"$AB" --session "$S" eval --stdin < "$PWD/$D/probe.js" > "$D/data/probe-nav.json" 2>&1
head -c 3000 "$D/data/probe-nav.json"
echo
"$AB" --session "$S" screenshot "$SH/r23-03-nav-skills.png" >/dev/null 2>&1
echo SHOT_NAV_DONE
echo '=== DONE NAV ==='
