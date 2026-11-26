#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round71-verify-time-format"
AB(){ agent-browser --session "$1" "${@:2}"; }
AB r71-trial close >/dev/null 2>&1 || true
AB r71-trial open --init-script "$P/data/auth-init.js" --init-script "$P/data/r71-init.js" "http://127.0.0.1:5201/#/pages/verify/verify" >/dev/null 2>&1
AB r71-trial set viewport 768 1024 >/dev/null 2>&1
AB r71-trial reload >/dev/null 2>&1
sleep 5
echo TRIAL_BEFORE
AB r71-trial eval --stdin < "$P/data/r71-probe.js" 2>&1 | tail -3
echo TRIAL_CLICK
AB r71-trial eval --stdin < "$P/data/r71-click.js" 2>&1 | tail -3
sleep 2
echo TRIAL_AFTER
AB r71-trial eval --stdin < "$P/data/r71-probe.js" 2>&1 | tail -3
AB r71-trial screenshot "$P/screenshots/trial-768.png" 2>&1 | tail -2
echo TRIAL_DONE