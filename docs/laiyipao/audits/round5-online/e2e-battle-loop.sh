#!/bin/bash
# t56 R5 线上回放结算闭环（真值内存，输出动态 mask，非物理真机）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS" | head -1 | sed "s/^DOMAIN=//;s/[[:space:]]*$//" | tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS" | head -1 | sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//" | tr -d "\r")"
BASE="https://$DOMAIN/laiyipao-h5/#"
DIR="docs/laiyipao/audits/round5-online"
S="lyp-r5online-h5"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo "=== navigate stage ==="
$AB open "$BASE/pages/stage/stage" >/dev/null 2>&1
sleep 5
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/STAGE: /"
JSON.stringify({url:location.href,hasToken:!!localStorage.getItem("lyp_token"),bodyLen:document.body.innerText.length,startBtns:[].slice.call(document.querySelectorAll("uni-view,view,text,button,.btn")).filter(function(e){return (e.innerText||"").indexOf("开始战斗")>=0&&e.children.length===0}).map(function(e){return (e.innerText||"").trim().slice(0,40)})})
EOF
echo "=== click start battle ==="
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){var all=document.querySelectorAll("uni-view,view,text,button,.btn");for(var i=0;i<all.length;i++){var e=all[i];var t=(e.innerText||"").trim();if(t.indexOf("开始战斗")>=0&&e.children.length===0){e.click();return "clicked"}}return "no-btn"})()
EOF
sleep 6
echo "=== battle loop (click skip each wave) ==="
for i in $(seq 1 40); do
  ST=$($AB eval --stdin < "$DIR/e2e-battle-check.js" 2>&1 | mask)
  echo "t=${i}x3s $ST"
  echo "$ST" | grep -q '"hasResult":true' && { echo "=== RESULT REACHED ==="; break; }
  sleep 3
done
echo "=== final ==="
$AB get url 2>&1 | mask | sed "s/^/FINAL_URL: /"
$AB screenshot "$DIR/screenshots/r5-online-e2e-settle.png" >/dev/null 2>&1
echo "settle shot saved"
echo DONE