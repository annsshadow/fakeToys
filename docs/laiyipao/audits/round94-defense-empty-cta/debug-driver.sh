#!/usr/bin/env bash
set -u
cd "$(dirname "$0")"
AB(){ agent-browser --session "$1" "${@:2}"; }
S="r94dbg"
AB "$S" close --all >/dev/null 2>&1 || true
AB "$S" open --init-script data/r94-init.js >/dev/null 2>&1
AB "$S" open --init-script data/r94-init.js >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "http://127.0.0.1:5175/laiyipao-admin/#/admin/defenses" >/dev/null 2>&1
sleep 2
echo "=== INITIAL DEBUG ==="
AB "$S" eval --stdin < data/r94-debug.js
echo "=== FILL real ==="
AB "$S" fill "input[placeholder]" "NO_SUCH_zzz" >/dev/null 2>&1; echo "FILL_RC=$?"
AB "$S" press Enter >/dev/null 2>&1; echo "ENTER_RC=$?"
sleep 2
echo "=== AFTER FILL DEBUG ==="
AB "$S" eval --stdin < data/r94-debug.js
echo "=== CLICK query button ==="
AB "$S" click "button:has-text('查询')" >/dev/null 2>&1; echo "CLICKQ_RC=$?"
sleep 2
echo "=== AFTER QUERY DEBUG ==="
AB "$S" eval --stdin < data/r94-debug.js
echo "=== PROBE ==="
AB "$S" eval --stdin < data/r94-probe.js
AB "$S" close --all >/dev/null 2>&1 || true
echo DBG_DONE