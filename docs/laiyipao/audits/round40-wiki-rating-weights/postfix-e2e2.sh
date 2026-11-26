#!/bin/bash
set -u
cd "$(dirname "$0")/../../../.." || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round40-wiki-rating-weights
DATA=$D/data
SHOTS=$D/screenshots
S=lyp-r40pf2
BASE="http://127.0.0.1:5175/laiyipao-admin"
mkdir -p "$DATA" "$SHOTS"
curl -s --max-time 15 http://127.0.0.1:8080/api/v1/config -o "$DATA/config.json"
echo CONFIG_BYTES=$(wc -c < "$DATA/config.json")
python "$D/gen-auth-init.py" docs/laiyipao/audits/round38-wiki-recipes/data/login.json "$DATA/admin-auth-init.js" > "$DATA/pf2-auth.txt" 2>&1
AUTH="$PWD/$DATA/admin-auth-init.js"
$AB close --all >/dev/null 2>&1 || true

for V in 390:844 768:1024 1280:800; do
  W=${V%:*}; H=${V#*:}
  $AB --session "$S" set viewport "$W" "$H" >/dev/null 2>&1
  $AB --session "$S" open --init-script "$AUTH" "$BASE/#/wiki?tab=rating" > "$DATA/pf2-open-$W.txt" 2>&1
  $AB --session "$S" wait 1500 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/rating-probe.js" > "$DATA/probe-$W.json" 2>&1
  $AB --session "$S" screenshot "$SHOTS/pf2-$W-rating.png" >/dev/null 2>&1
  for Q in fanyinglian:反应链 mechanic_depth:mechanic_depth; do
    TAG=${Q%%:*}; TERM=${Q#*:}
    $AB --session "$S" open --init-script "$AUTH" "$BASE/#/wiki?tab=rating" >/dev/null 2>&1
    $AB --session "$S" wait 1200 >/dev/null 2>&1
    $AB --session "$S" fill ".search-input" "$TERM" > "$DATA/pf2-fill-$W-$TAG.txt" 2>&1
    $AB --session "$S" wait 700 >/dev/null 2>&1
    $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/pf2-search-$W-$TAG.json" 2>&1
    $AB --session "$S" click ".search-hit" > "$DATA/pf2-click-$W-$TAG.txt" 2>&1
    $AB --session "$S" wait 900 >/dev/null 2>&1
    $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/pf2-after-$W-$TAG.json" 2>&1
    $AB --session "$S" screenshot "$SHOTS/pf2-$W-$TAG-focus.png" >/dev/null 2>&1
    echo "VP=$W Q=$TAG FILL=$(head -c 8 "$DATA/pf2-fill-$W-$TAG.txt") CLICK=$(head -c 8 "$DATA/pf2-click-$W-$TAG.txt")"
  done
done

# CTA real click
$AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
$AB --session "$S" open --init-script "$AUTH" "$BASE/#/wiki" > "$DATA/pf2-cta-open.txt" 2>&1
$AB --session "$S" wait 1200 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/cta-pre.js" > "$DATA/cta-pre.json" 2>&1
$AB --session "$S" screenshot "$SHOTS/pf2-cta-before.png" >/dev/null 2>&1
$AB --session "$S" click ".r40-ai-note a" > "$DATA/cta-click.txt" 2>&1
echo CTA_CLICK=$(head -c 20 "$DATA/cta-click.txt")
$AB --session "$S" wait 800 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/cta-post.js" > "$DATA/cta-post.json" 2>&1
$AB --session "$S" screenshot "$SHOTS/pf2-cta-after.png" >/dev/null 2>&1

# R31-R39 regression 9 tabs (fresh page, avoid stale state)
$AB --session "$S" open --init-script "$AUTH" "$BASE/#/wiki" > "$DATA/pf2-reg-open.txt" 2>&1
$AB --session "$S" wait 1200 >/dev/null 2>&1
click_tab() {
  K=$1; SEL=$2
  $AB --session "$S" click "$SEL" > "$DATA/reg-click-$K.txt" 2>&1
  $AB --session "$S" wait 500 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/reg-probe.js" > "$DATA/reg-$K.json" 2>&1
}
click_tab basics '.nav-item[href="#/wiki"]'
click_tab skills '.nav-item[href="#/wiki?tab=skills"]'
click_tab composite '.nav-sub-item[href="#/wiki?tab=skills&anchor=composite"]'
click_tab enemies '.nav-item[href="#/wiki?tab=enemies"]'
click_tab levels '.nav-item[href="#/wiki?tab=levels"]'
click_tab gems '.nav-item[href="#/wiki?tab=gems"]'
click_tab mastery '.nav-item[href="#/wiki?tab=mastery"]'
click_tab skins '.nav-item[href="#/wiki?tab=skins"]'
click_tab rules '.nav-item[href="#/wiki?tab=rules"]'
$AB --session "$S" close >/dev/null 2>&1 || true
echo POSTFIX2_DONE=YES
