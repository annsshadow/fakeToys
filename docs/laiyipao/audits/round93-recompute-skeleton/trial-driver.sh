#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
S="r93trial"
URL="http://127.0.0.1:5175/laiyipao-admin/#/admin/recomputes"
AB "$S" close --all >/dev/null 2>&1 || true
# 双open预热：第1次建立会话，第2次确保init-script注入到正确origin
AB "$S" open --init-script data/r93-init.js >/dev/null 2>&1
AB "$S" open --init-script data/r93-init.js >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "$URL" >/dev/null 2>&1
sleep 1.5
echo "=== LOAD 768 ==="
AB "$S" eval --stdin < data/r93-probe.js
AB "$S" screenshot "$(pwd)/screenshots/trial-768-load.png" >/dev/null 2>&1; echo "SHOT_LOAD_RC=$?"
sleep 6.5
echo "=== ARRIVE 768 ==="
AB "$S" eval --stdin < data/r93-probe.js
AB "$S" screenshot "$(pwd)/screenshots/trial-768-arrive.png" >/dev/null 2>&1; echo "SHOT_ARRIVE_RC=$?"
AB "$S" close --all >/dev/null 2>&1 || true
echo TRIAL_DONE