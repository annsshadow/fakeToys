#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
S="r94trial"
URL="http://127.0.0.1:5175/laiyipao-admin/#/admin/defenses"
AB "$S" close --all >/dev/null 2>&1 || true
AB "$S" open --init-script data/r94-init.js >/dev/null 2>&1
AB "$S" open --init-script data/r94-init.js >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "$URL" >/dev/null 2>&1
sleep 2
echo "=== FILL keyword ==="
AB "$S" eval --stdin < data/r94-fill.js
AB "$S" press Enter >/dev/null 2>&1
sleep 2
echo "=== BEFORE CTA ==="
AB "$S" eval --stdin < data/r94-probe.js
echo "=== CLICK CTA ==="
AB "$S" click ".empty-state .empty-state-action" >/dev/null 2>&1; echo "CLICK_RC=$?"
sleep 2
echo "=== AFTER CTA ==="
AB "$S" eval --stdin < data/r94-probe.js
AB "$S" close --all >/dev/null 2>&1 || true
echo TRIAL_DONE