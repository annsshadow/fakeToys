#!/bin/bash
# 重截 admin 截图：隐藏 .api-tag 真域名后截图覆盖（脱敏）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
PASS=$(grep -E '^ADMIN_PASS=' "$KEYS"|head -1|sed 's/^ADMIN_PASS=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-postadmin"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$PASS|<ADMIN_PASS>|g"; }
BASE="https://$DOMAIN/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round3-postdeploy/screenshots"
echo "=== login ==="
$AB open "$BASE/admin/login" >/dev/null 2>&1; sleep 3
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
localStorage.clear();"cleared"
EOF
$AB reload >/dev/null 2>&1; sleep 3
$AB fill "input[placeholder='admin']" admin >/dev/null 2>&1
$AB fill "input[placeholder='至少 10 位']" "$PASS" >/dev/null 2>&1
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){var b=document.querySelectorAll("button");for(var i=0;i<b.length;i++){if((b[i].innerText||"").trim()==="登录"){b[i].click();return "ok"}}return "no"})()
EOF
sleep 4
echo "=== dashboard (mask api-tag + close palette) ==="
$AB open "$BASE/admin/dashboard" >/dev/null 2>&1; sleep 3
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
document.querySelectorAll(".api-tag").forEach(function(e){e.style.display="none"});var o=document.querySelector(".cmd-panel-overlay,.cmd-overlay");if(o)o.click();"masked"
EOF
$AB screenshot "$DIR/post-admin-dashboard.png" >/dev/null 2>&1
echo "dashboard masked"
echo "=== four views ==="
for V in levels skills users recomputes; do
  $AB open "$BASE/admin/$V" >/dev/null 2>&1; sleep 3
  cat <<EOF | $AB eval --stdin >/dev/null 2>&1
document.querySelectorAll(".api-tag").forEach(function(e){e.style.display="none"});"masked"
EOF
  $AB screenshot "$DIR/post-admin-$V.png" >/dev/null 2>&1
  echo "$V masked"
done
echo "=== palette ==="
$AB open "$BASE/admin/dashboard" >/dev/null 2>&1; sleep 3
$AB press Control+k >/dev/null 2>&1; sleep 1
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
document.querySelectorAll(".api-tag").forEach(function(e){e.style.display="none"});"masked"
EOF
$AB screenshot "$DIR/post-admin-palette.png" >/dev/null 2>&1
echo "palette masked"
echo "=== DONE ==="