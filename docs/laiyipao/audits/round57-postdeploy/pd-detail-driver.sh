#!/usr/bin/env bash
set -uo pipefail
AB(){ agent-browser --session "$1" "${@:2}"; }
S=lyp-r57-pd2
D=docs/laiyipao/audits/round57-postdeploy
INIT="docs/laiyipao/audits/round57-visual-truncation/data/auth-init.js"
BASE="http://127.0.0.1:5201/#/pages"
AB "$S" open --init-script "$INIT" "$BASE/announcements/announcements" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
AB "$S" eval --stdin < "$D/pd-detail-probe.js" > "$D/data/pd-detail-768-ann.json" 2>"$D/data/pddetailerr.txt"
echo DETAIL_ANN_DONE
AB "$S" open --init-script "$INIT" "$BASE/index/index" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
AB "$S" eval --stdin < "$D/pd-detail-probe.js" > "$D/data/pd-detail-768-index.json" 2>"$D/data/pdindexerr.txt"
echo DETAIL_INDEX_DONE