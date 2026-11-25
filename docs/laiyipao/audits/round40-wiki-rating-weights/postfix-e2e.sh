#!/bin/bash
# t30 independent postfix review: real CLI (agent-browser) three-viewport rating search/highlight,
# config reconciliation, CTA, R31-R39 regression. Does NOT modify business source or run git.
set -u
cd "$(dirname "$0")/../../../.." || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round40-wiki-rating-weights
DATA=$D/data
SHOTS=$D/screenshots
S=lyp-r40pf
BASE="http://127.0.0.1:5175/laiyipao-admin"
mkdir -p "$DATA" "$SHOTS"

# Fresh public config snapshot for truth reconciliation
curl -s --max-time 15 http://127.0.0.1:8080/api/v1/config -o "$DATA/config.json"
echo CONFIG_BYTES=$(wc -c < "$DATA/config.json")

# Temp auth from prior local login artifact (path only; never printed)
python "$D/gen-auth-init.py" docs/laiyipao/audits/round38-wiki-recipes/data/login.json "$DATA/admin-auth-init.js" > "$DATA/pf-auth.txt" 2>&1
AUTH="$PWD/$DATA/admin-auth-init.js"
$AB close --all >/dev/null 2>&1 || true

# ---- three viewports: rating table probe + search hit/click/highlight for two queries ----
for V in 390:844 768:1024 1280:800; do
  W=${V%:*}; H=${V#*:}
  $AB --session "$S" set viewport "$W" "$H" >/dev/null 2>&1
  $AB --session "$S" open --init-script "$AUTH" "$BASE/#/wiki?tab=rating" > "$DATA/pf-open-$W.txt" 2>&1
  $AB --session "$S" wait 1200 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/rating-probe.js" > "$DATA/probe-$W.json" 2>&1
  $AB --session "$S" screenshot "$SHOTS/pf-$W-rating.png" >/dev/null 2>&1
  for Q in fanyinglian:反应链 mechanic_depth:mechanic_depth; do
    TAG=${Q%%:*}; TERM=${Q#*:}
    $AB --session "$S" click ".search-input" >/dev/null 2>&1
    $AB --session "$S" press Control+a >/dev/null 2>&1
    $AB --session "$S" press Backspace >/dev/null 2>&1
    $AB --session "$S" fill ".search-input" "$TERM" > "$DATA/pf-fill-$W-$TAG.txt" 2>&1
    $AB --session "$S" wait 500 >/dev/null 2>&1
    $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/pf-search-$W-$TAG.json" 2>&1
    $AB --session "$S" click ".search-hit" > "$DATA/pf-click-$W-$TAG.txt" 2>&1
    $AB --session "$S" wait 600 >/dev/null 2>&1
    $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/pf-after-$W-$TAG.json" 2>&1
    $AB --session "$S" screenshot "$SHOTS/pf-$W-$TAG-focus.png" >/dev/null 2>&1
    echo "VP=$W Q=$TAG CLICK=$(head -c 20 "$DATA/pf-click-$W-$TAG.txt")"
  done
done

# ---- CTA real CLI click (stay on rating, no dashboard jump) ----
$AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
$AB --session "$S" open --init-script "$AUTH" "$BASE/#/wiki" > "$DATA/pf-cta-open.txt" 2>&1
$AB --session "$S" wait 1200 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/cta-pre.js" > "$DATA/cta-pre.json" 2>&1
$AB --session "$S" screenshot "$SHOTS/pf-cta-before.png" >/dev/null 2>&1
$AB --session "$S" click ".r40-ai-note a" > "$DATA/cta-click.txt" 2>&1
echo CTA_CLICK=$(head -c 20 "$DATA/cta-click.txt")
$AB --session "$S" wait 800 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/cta-post.js" > "$DATA/cta-post.json" 2>&1
$AB --session "$S" screenshot "$SHOTS/pf-cta-after.png" >/dev/null 2>&1

# ---- R31-R39 regression tabs ----
$AB --session "$S" open --init-script "$AUTH" "$BASE/#/wiki" > "$DATA/pf-reg-open.txt" 2>&1
$AB --session "$S" wait 1200 >/dev/null 2>&1
click_tab() {
  K=$1; SEL=$2
  $AB --session "$S" click "$SEL" > "$DATA/reg-click-$K.txt" 2>&1
  $AB --session "$S" wait 500 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/reg-probe.js" > "$DATA/reg-$K.json" 2>&1
  if [ "$K" = basics ] || [ "$K" = skins ] || [ "$K" = rules ] || [ "$K" = composite ]; then
    $AB --session "$S" screenshot "$SHOTS/pf-reg-$K.png" >/dev/null 2>&1
  fi
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
echo POSTFIX_E2E_DONE=YES
