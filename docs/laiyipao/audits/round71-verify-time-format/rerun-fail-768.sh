#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round71-verify-time-format"
D="$P/data"; S="$P/screenshots"
AB(){ agent-browser --session "$1" "${@:2}"; }
SS="r71-fail-768-rerun"
AB "$SS" close >/dev/null 2>&1 || true
AB "$SS" open --init-script "$D/auth-init.js" --init-script "$D/r71-init.js" >/dev/null 2>&1
AB "$SS" set viewport 768 1024 >/dev/null 2>&1
AB "$SS" navigate "http://127.0.0.1:5201/#/pages/verify/verify" >/dev/null 2>&1
sleep 6
AB "$SS" eval "window.__qaCfg.fail=true; 'FAIL_ON'" 2>&1 | tail -2
AB "$SS" eval --stdin < "$D/r71-click.js" 2>&1 | tail -2
sleep 2
AB "$SS" eval --stdin < "$D/r71-probe.js" > "$D/fail-768-probe.txt" 2>&1
AB "$SS" screenshot "$S/r71-fail-768.png" > "$D/fail-768-shot.txt" 2>&1
AB "$SS" close >/dev/null 2>&1 || true
echo RERUN_DONE