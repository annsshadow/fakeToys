#!/usr/bin/env bash
set -euo pipefail
P="docs/laiyipao/audits/round64-battles-detail-race"; D="$P/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
AB r64fin open --init-script "$D/auth-init.js" --init-script "$P/race-init2.js" >/dev/null 2>&1
AB r64fin set viewport 768 1024 >/dev/null 2>&1
AB r64fin navigate "http://127.0.0.1:5201/#/pages/battles/battles" >/dev/null 2>&1
sleep 4
echo "SET=$(AB r64fin eval --stdin < "$P/race-set2.js" 2>&1)"
echo "CLICK_A=$(AB r64fin eval --stdin < "$P/r64-clickB.js" 2>&1)"
sleep 0.4
echo "CLICK_B=$(AB r64fin eval --stdin < "$P/r64-clickI4.js" 2>&1)"
sleep 4.2
AB r64fin eval --stdin < "$P/r64-probe.js" > "$D/race-final-witness.txt" 2>&1
AB r64fin screenshot --path "$D/../screenshots/race-final-768.png" >/dev/null 2>&1 || true
echo DONE_FINAL
