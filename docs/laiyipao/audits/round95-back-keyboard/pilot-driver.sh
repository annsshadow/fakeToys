#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
BASE='http://127.0.0.1:5174/'
S='r95pilot'
AB "$S" close --all >/dev/null 2>&1 || true
AB "$S" open >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "$BASE" >/dev/null 2>&1
sleep 3
echo '=== OPEN INDEX ==='
AB "$S" eval --stdin < data/r95-probe.js
echo '=== CLICK 我的 (grid-item by text) ==='
AB "$S" eval --stdin < data/r95-click-me.js
sleep 3
echo '=== AFTER NAV ME ==='
AB "$S" eval --stdin < data/r95-probe.js
echo PILOT_DONE