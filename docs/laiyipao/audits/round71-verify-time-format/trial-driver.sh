#!/usr/bin/env bash
P=docs/laiyipao/audits/round71-verify-time-format
function AB(){ agent-browser --session "$1" "${@:2}"; }
S="r71-trial"
AB "$S" close --all >/dev/null 2>&1
AB "$S" open --init-script "$P/data/auth-init.js" --init-script "$P/data/r71-init.js" "http://127.0.0.1:5201/#/pages/verify/verify" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" reload >/dev/null 2>&1
sleep 5
echo "=== BEFORE CLICK ==="
AB "$S" eval --stdin < "$P/data/r71-probe.js" 2>&1 | grep PROBE_RESULT | sed "s/^PROBE_RESULT //"
echo "=== CLICK ==="
AB "$S" eval --stdin < "$P/data/r71-click.js" 2>&1 | tail -6
sleep 2
echo "=== AFTER CLICK ==="
AB "$S" eval --stdin < "$P/data/r71-probe.js" 2>&1 | grep PROBE_RESULT | sed "s/^PROBE_RESULT //" > "$P/data/trial-768.txt"
cat "$P/data/trial-768.txt"
AB "$S" screenshot "$P/screenshots/trial-768.png" >/dev/null 2>&1
echo TRIAL_DONE