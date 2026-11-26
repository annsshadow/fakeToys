#!/bin/bash
set -u
cd "$(dirname "$0")/../../../.." || exit 1
D=docs/laiyipao/audits/round40-wiki-rating-weights
DATA=$D/data
AB=agent-browser
S=lyp-r40pf-s2
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=rating"
python "$D/gen-auth-init.py" docs/laiyipao/audits/round38-wiki-recipes/data/login.json "$DATA/admin-auth-init.js" > "$DATA/pf-auth-gen.txt" 2>&1
$AB close --all >/dev/null 2>&1 || true
$AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
$AB --session "$S" open --init-script "$DATA/admin-auth-init.js" "$URL" > "$DATA/pf-s2-open.txt" 2>&1
$AB --session "$S" wait 1800 >/dev/null 2>&1
$AB --session "$S" eval "$(tr '\n' ' ' < "$D/rating-probe.js")" > "$DATA/pf-s2-rating.json" 2>&1
$AB --session "$S" fill ".search-input" "反应链" > "$DATA/pf-s2-fill.txt" 2>&1
$AB --session "$S" wait 600 >/dev/null 2>&1
$AB --session "$S" eval "$(tr '\n' ' ' < "$D/search-probe.js")" > "$DATA/pf-s2-search.json" 2>&1
echo S2_DONE=YES
