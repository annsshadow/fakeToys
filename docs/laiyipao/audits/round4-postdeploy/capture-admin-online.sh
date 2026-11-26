#!/bin/bash
# t44 线上 admin 五视图+命令面板+顶栏脱敏（真值内存，输出全脱敏，非物理真机 1258x588）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS" | head -1 | sed "s/^DOMAIN=//;s/[[:space:]]*$//" | tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS" | head -1 | sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//" | tr -d "\r")"
BASE="https://$DOMAIN/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round4-postdeploy"
S="lyp-r4online-admin"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo "=== STEP1 login ==="
$AB --init-script "$DIR/probe-init.js" open "$BASE/admin/login" >/dev/null 2>&1
sleep 5
$AB set viewport 1258 588 >/dev/null 2>&1
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){try{localStorage.clear()}catch(e){};return true})()
EOF
$AB fill "input[placeholder='admin']" "admin" >/dev/null 2>&1
$AB fill "input[placeholder='至少 10 位']" "$ADMIN_PASS" >/dev/null 2>&1
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){var bs=[].slice.call(document.querySelectorAll("button"));for(var i=0;i<bs.length;i++){if((bs[i].innerText||"").trim().indexOf("登录")>=0){bs[i].click();return "clicked"}}return "no-btn"})()
EOF
sleep 6
$AB get url 2>&1 | mask | sed "s/^/LOGIN_URL: /"
echo "=== STEP2 topbar mask check ==="
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/TOPBAR: /"
JSON.stringify({apiTagText:(document.querySelector(".api-tag")||{}).innerText||"(none)",hasDomainInBody:(function(){var t=document.body.innerText;return /[a-zA-Z0-9-]+\.[a-z]{2,}/.test(t)})(),title:document.title})
EOF
echo "=== STEP3 five views ==="
for V in dashboard content economy defenses equipment; do
  $AB open "$BASE/admin/$V" >/dev/null 2>&1
  sleep 4
  $AB screenshot "$DIR/screenshots/r4-online-admin-$V.png" >/dev/null 2>&1
  echo "--- $V ---"
  $AB get url 2>&1 | mask | sed "s/^/URL: /"
  $AB get title 2>&1 | mask | sed "s/^/TITLE: /"
  cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/DOM: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/admin/login")>=0,title:document.title,apiTag:(document.querySelector(".api-tag")||{}).innerText||"(none)",kicker:document.querySelectorAll(".section-kicker").length,kickerCn:(document.querySelector(".section-kicker")||{}).innerText||"",kpi:document.querySelectorAll(".kpi-strip,.kpi-main,.kpi-primary").length,ai:document.querySelectorAll(".r3-ai,.r4-ai,.ai-insight").length,aiBadge:(document.querySelector(".ai-badge,.r3-ai-badge,.ai-insight-badge")||{}).innerText||"",aiConclusion:(document.querySelector(".ai-conclusion,.r3-ai-conclusion,.ai-insight-headline")||{}).innerText||"",tables:document.querySelectorAll("table").length,rows:document.querySelectorAll("tbody tr").length,bodyLen:document.body.innerText.length,probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
done
echo "=== STEP4 palette ==="
$AB open "$BASE/admin/dashboard" >/dev/null 2>&1
sleep 3
$AB press "Control+k" >/dev/null 2>&1
sleep 1
$AB screenshot "$DIR/screenshots/r4-online-admin-palette.png" >/dev/null 2>&1
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/PALETTE: /"
JSON.stringify({panel:document.querySelectorAll(".cmd-panel").length,aiGroup:document.querySelectorAll(".cmd-ai-group-label").length,aiOpts:document.querySelectorAll(".cmd-ai-option").length,ariaModal:(document.querySelector(".cmd-panel")||{}).getAttribute?document.querySelector(".cmd-panel").getAttribute("aria-modal"):null})
EOF
echo DONE