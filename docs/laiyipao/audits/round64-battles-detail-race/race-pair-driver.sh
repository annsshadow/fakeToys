#!/usr/bin/env bash
set -euo pipefail
P="docs/laiyipao/audits/round64-battles-detail-race"; D="$P/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
# REF: single-expand stage40 (i4) to capture its unique growth baseline
AB r64ref open --init-script "$D/auth-init.js" --init-script "$P/race-init.js" >/dev/null 2>&1
AB r64ref set viewport 768 1024 >/dev/null 2>&1
AB r64ref navigate "http://127.0.0.1:5201/#/pages/battles/battles" >/dev/null 2>&1
sleep 4
echo "REF_CLICK=$(AB r64ref eval --stdin < "$P/r64-clickI4.js" 2>&1)"
sleep 2
AB r64ref eval --stdin < "$P/r64-probe.js" > "$D/race-ref-i4.txt" 2>&1
# FINAL: A=i4(stage40) delayed, B=i0(stage5 8888) fast-switch
AB r64final open --init-script "$D/auth-init.js" --init-script "$P/race-init.js" >/dev/null 2>&1
AB r64final set viewport 768 1024 >/dev/null 2>&1
AB r64final navigate "http://127.0.0.1:5201/#/pages/battles/battles" >/dev/null 2>&1
sleep 4
echo "SET=$(AB r64final eval --stdin < "$P/race-set-list.js" 2>&1)"
echo "CLICK_A=$(AB r64final eval --stdin < "$P/r64-clickI4.js" 2>&1)"
sleep 0.4
echo "CLICK_B=$(AB r64final eval --stdin < "$P/r64-clickA.js" 2>&1)"
sleep 3.4
AB r64final eval --stdin < "$P/r64-probe.js" > "$D/race-final-i4-A.txt" 2>&1
echo DONE
