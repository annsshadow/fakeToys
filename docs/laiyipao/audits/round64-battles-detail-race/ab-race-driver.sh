#!/usr/bin/env bash
set -euo pipefail
P="docs/laiyipao/audits/round64-battles-detail-race"; D="$P/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
AB r64ab open --init-script "$D/auth-init.js" --init-script "$P/race-init.js" >/dev/null 2>&1
AB r64ab set viewport 768 1024 >/dev/null 2>&1
AB r64ab navigate "http://127.0.0.1:5201/#/pages/battles/battles" >/dev/null 2>&1
sleep 4
echo "SET=$(AB r64ab eval --stdin < "$P/race-set-list.js" 2>&1)"
echo "CLICK_A=$(AB r64ab eval --stdin < "$P/r64-clickI4.js" 2>&1)"
sleep 0.4
echo "CLICK_B=$(AB r64ab eval --stdin < "$P/r64-clickA.js" 2>&1)"
sleep 3.4
AB r64ab eval --stdin < "$P/r64-probe.js" > "$D/race-ab-i4-A.txt" 2>&1
echo DONE_AB
