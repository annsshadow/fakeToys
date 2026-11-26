#!/usr/bin/env bash
set -euo pipefail
P="docs/laiyipao/audits/round64-battles-detail-race"; D="$P/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
AB r64pr open --init-script "$D/auth-init.js" >/dev/null 2>&1
AB r64pr set viewport 768 1024 >/dev/null 2>&1
AB r64pr navigate "http://127.0.0.1:5201/#/pages/battles/battles" >/dev/null 2>&1
sleep 4
echo "CLICK_PAIR=$(AB r64pr eval --stdin < "$P/r64-clickI4pair.js" 2>&1)"
sleep 2
AB r64pr eval --stdin < "$P/r64-probe.js" > "$D/race-paired-i4.txt" 2>&1
echo DONE_PAIR
