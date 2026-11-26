#!/usr/bin/env bash
set -euo pipefail
P="docs/laiyipao/audits/round64-battles-detail-race"; D="$P/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
AB r64ref open --init-script "$D/auth-init.js" >/dev/null 2>&1
AB r64ref set viewport 768 1024 >/dev/null 2>&1
AB r64ref navigate "http://127.0.0.1:5201/#/pages/battles/battles" >/dev/null 2>&1
sleep 4
echo "CLICK=$(AB r64ref eval --stdin < "$P/r64-clickI4.js" 2>&1)"
sleep 2
AB r64ref eval --stdin < "$P/r64-probe.js" > "$D/race-ref-i4.txt" 2>&1
echo DONE
