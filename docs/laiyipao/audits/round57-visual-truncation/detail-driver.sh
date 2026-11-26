#!/usr/bin/env bash
set -uo pipefail
AB(){ agent-browser --session "$1" "${@:2}"; }
S=lyp-r57-vis
D=docs/laiyipao/audits/round57-visual-truncation
AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/announcements/announcements" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
sleep 2
AB "$S" eval --stdin < "$D/r57-detail-probe.js" > "$D/data/detail-768-announcements.json" 2>"$D/data/detail-ann-err.txt"
AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/enemies/enemies" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1
sleep 2
AB "$S" eval --stdin < "$D/r57-detail-probe.js" > "$D/data/detail-768-enemies.json" 2>"$D/data/detail-ene-err.txt"
echo DETAIL_DONE