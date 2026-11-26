#!/bin/bash
set -u
cd "$(dirname "$0")/../../../.." || exit 1
D=docs/laiyipao/audits/round40-wiki-rating-weights
DATA=$D/data
mkdir -p "$DATA"
AB=agent-browser
S=lyp-r40pf-smoke
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=rating"
$AB close --all >/dev/null 2>&1 || true
$AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
$AB --session "$S" open "$URL" > "$DATA/pf-smoke-open.txt" 2>&1
$AB --session "$S" wait 1500 >/dev/null 2>&1
$AB --session "$S" eval "$(tr '\n' ' ' < "$D/rating-probe.js")" > "$DATA/pf-smoke-rating.json" 2>&1
$AB --session "$S" fill ".search-input" "反应链" > "$DATA/pf-smoke-fill.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval "$(tr '\n' ' ' < "$D/search-probe.js")" > "$DATA/pf-smoke-search.json" 2>&1
echo SMOKE_DONE=YES
