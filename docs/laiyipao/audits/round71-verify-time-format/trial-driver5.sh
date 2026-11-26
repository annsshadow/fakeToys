#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round71-verify-time-format"
AB(){ agent-browser --session "$1" "${@:2}"; }
AB rt5 close >/dev/null 2>&1 || true
AB rt5 open --init-script "$P/data/auth-init.js" --init-script "$P/data/r71-init.js" >/dev/null 2>&1
AB rt5 set viewport 768 1024 >/dev/null 2>&1
AB rt5 navigate "http://127.0.0.1:5201/#/pages/verify/verify" >/dev/null 2>&1
sleep 5
echo TRIAL_BEFORE
AB rt5 eval --stdin < "$P/data/r71-probe.js" > "$P/data/trial5-before.txt" 2>&1
grep -o "{.*}" "$P/data/trial5-before.txt" | head -1
echo TRIAL_CLICK
AB rt5 eval --stdin < "$P/data/r71-click.js" 2>&1 | tail -2
sleep 2
echo TRIAL_AFTER
AB rt5 eval --stdin < "$P/data/r71-probe.js" > "$P/data/trial5-after.txt" 2>&1
grep -o "{.*}" "$P/data/trial5-after.txt" | head -1
AB rt5 screenshot "$P/screenshots/trial5-768.png" 2>&1 | tail -2
AB rt5 close >/dev/null 2>&1 || true
echo TRIAL_DONE