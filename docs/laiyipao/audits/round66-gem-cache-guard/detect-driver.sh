#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round66-gem-cache-guard"; D="$P/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
AB r66det open --init-script "$D/auth-init.js" --init-script "$D/r66-init.js" >/dev/null 2>&1
AB r66det set viewport 768 1024 >/dev/null 2>&1
AB r66det navigate "http://127.0.0.1:5201/#/pages/gems/gems" >/dev/null 2>&1
sleep 5
AB r66det eval --stdin < "$P/r66-detect.js" > "$D/detect-768.txt" 2>&1
echo "DETECT=$(cat "$D/detect-768.txt")"
AB r66det close >/dev/null 2>&1 || true
echo DONE_DETECT
