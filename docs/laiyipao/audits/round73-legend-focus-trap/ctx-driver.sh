#!/bin/bash
set -uo pipefail
P="docs/laiyipao/audits/round73-legend-focus-trap"; D="$P/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
S="r73ctx"; OUT="$D/ctx-out.txt"; : > "$OUT"
AB "$S" close --all >/dev/null 2>&1 || true
AB "$S" open --init-script "$D/auth-init.js" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
AB "$S" navigate "http://127.0.0.1:5201/#/pages/battle/battle?level=1" >/dev/null 2>&1
sleep 6
AB "$S" eval --stdin < "$D/r73-ctx.js" >> "$OUT" 2>&1
AB "$S" close >/dev/null 2>&1 || true
echo CTX_DONE >> "$OUT"
