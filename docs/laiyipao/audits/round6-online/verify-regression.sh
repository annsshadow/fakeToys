#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS"|head -1|sed "s/^DOMAIN=//;s/[[:space:]]*$//"|tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS"|head -1|sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//"|tr -d "\r")"
BASE="https://$DOMAIN/laiyipao-h5/#"; DIR="docs/laiyipao/audits/round6-online"; AB="agent-browser --session lyp-r6online-h5"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo "=== AoE legend 25% online check ==="
$AB open "$BASE/pages/battle/battle" >/dev/null 2>&1; sleep 6
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/LEGEND_BTN: /"
JSON.stringify({btns:document.querySelectorAll("[aria-label='战场图例']").length,labels:[].slice.call(document.querySelectorAll("[aria-label]")).map(function(e){return e.getAttribute("aria-label")}).slice(0,20)})
EOF
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){var b=document.querySelector("[aria-label='战场图例']");if(b){b.click();return "clicked"}return "no-btn"})()
EOF
sleep 2
$AB screenshot "$DIR/screenshots/r6-online-h5-legend25.png" >/dev/null 2>&1
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/AOE: /"
JSON.stringify({url:location.href,pct25:(document.body.innerText.match(/25%/g)||[]).length,radius120:(document.body.innerText.match(/半径 120/g)||[]).length,steam:(document.body.innerText.indexOf("蒸汽")>=0||document.body.innerText.indexOf("蒸腾")>=0),overheat:(document.body.innerText.indexOf("过热")>=0),bodyLen:document.body.innerText.length})
EOF
echo "=== config base_damage via curl ==="
curl -s --max-time 30 "https://$DOMAIN/laiyipao-api/api/v1/config" -o "$DIR/data/config-env.json"
echo "CONFIG_BYTES=$(wc -c < "$DIR/data/config-env.json")"
node "$DIR/parse-config.js" "$DIR/data/config-env.json" 2>&1 | sed "s|$DOMAIN|<DOMAIN>|g;s|$ADMIN_PASS|<PASS>|g"
echo REGRESSION_DONE