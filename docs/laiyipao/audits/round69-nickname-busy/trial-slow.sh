#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round69-nickname-busy"; D="$P/data"; S="$P/screenshots"
AB(){ agent-browser --session "$1" "${@:2}"; }
S=r69trial768
AB "$S" open --init-script "$D/auth-init.js" --init-script "$D/r69-init.js" >/dev/null 2>&1; echo "OPEN_RC=$?"
AB "$S" set viewport 768 1024 >/dev/null 2>&1; echo "VIEWPORT_RC=$?"
AB "$S" navigate "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1; echo "NAV_RC=$?"
sleep 5
echo "=== HASH ==="; AB "$S" eval "location.hash"
echo "=== EDIT ==="; AB "$S" eval --stdin < "$P/r69-open-edit.js"
echo "=== FILL ==="; AB "$S" eval --stdin < "$P/r69-fill.js"
AB "$S" eval "window.__qaNet.slow=true; window.__qaNet.puts=[]; 'SLOW_ON'"
echo "=== CLICK ==="; AB "$S" eval --stdin < "$P/r69-click.js"
sleep 1
echo "=== PROBE_1S ==="; AB "$S" eval --stdin < "$P/r69-probe.js"
AB "$S" screenshot "$S/r69-trial-slow-768.png" >/dev/null 2>&1; echo "SHOT_RC=$?"
sleep 3
echo "=== PROBE_AFTER ==="; AB "$S" eval --stdin < "$P/r69-probe.js"
AB "$S" close >/dev/null 2>&1 || true
echo TRIAL_DONE
