#!/bin/bash
# 线上 reactions 页 25% 图例文案真浏览器验证
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS" | head -1 | sed "s/^DOMAIN=//;s/[[:space:]]*$//" | tr -d "\r")"
AB="agent-browser --session lyp-r4online-h5"
BASE="https://$DOMAIN/laiyipao-h5/#"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"; }
$AB open "$BASE/pages/reactions/reactions" >/dev/null 2>&1
sleep 4
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/REACT: /"
JSON.stringify({url:location.href,title:document.title,hasToken:!!localStorage.getItem("lyp_token"),pct25:(document.body.innerText.match(/25%/g)||[]).length,radius120:(document.body.innerText.match(/半径 120/g)||[]).length,steam:(document.body.innerText.indexOf("蒸汽")>=0||document.body.innerText.indexOf("蒸腾")>=0),overheat:(document.body.innerText.indexOf("过热")>=0),bodyLen:document.body.innerText.length})
EOF
echo DONE