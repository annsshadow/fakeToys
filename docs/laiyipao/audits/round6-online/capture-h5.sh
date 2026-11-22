#!/bin/bash
# t61 R6 线上独立回归 H5 三页（真值内存注入+输出全脱敏，非物理真机 390x844）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS" | head -1 | sed "s/^DOMAIN=//;s/[[:space:]]*$//" | tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS" | head -1 | sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//" | tr -d "\r")"
if [ -z "$DOMAIN" ]; then echo DOMAIN_MISSING; exit 2; fi
BASE="https://$DOMAIN/laiyipao-h5/#"
DIR="docs/laiyipao/audits/round6-online"
S="lyp-r6online-h5"; AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo "=== STEP1 open login + probe + viewport ==="
$AB --init-script "$DIR/probe-init.js" open "$BASE/pages/login/login" >/dev/null 2>&1
sleep 6
$AB set viewport 390 844 >/dev/null 2>&1
echo "=== STEP2 REAL player login (clear + type selectors + native click) ==="
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){try{localStorage.removeItem("lyp_token");localStorage.removeItem("lyp_token_exp");localStorage.removeItem("lyp_profile")}catch(e){};return true})()
EOF
$AB open "$BASE/pages/login/login" >/dev/null 2>&1
sleep 6
$AB fill "input[type='text']" "admin" >/dev/null 2>&1
$AB fill "input[type='password']" "$ADMIN_PASS" >/dev/null 2>&1
$AB click ".btn-primary" >/dev/null 2>&1
sleep 8
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/AUTH: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/login/login")>=0,hasToken:!!localStorage.getItem("lyp_token")})
EOF
echo "=== STEP3 three pages equipment/reactions/skills ==="
for P in equipment reactions skills; do
  $AB open "$BASE/pages/$P/$P" >/dev/null 2>&1
  sleep 5
  $AB screenshot "$DIR/screenshots/r6-online-h5-$P.png" >/dev/null 2>&1
  echo "--- $P ---"
  $AB get url 2>&1 | mask | sed "s/^/URL: /"
  $AB get title 2>&1 | mask | sed "s/^/TITLE: /"
  cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/DOM: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/login/login")>=0,title:document.title,hasToken:!!localStorage.getItem("lyp_token"),kicker:document.querySelectorAll(".section-kicker").length,kickerCn:(document.querySelector(".section-kicker")||{}).innerText||"",kpi:document.querySelectorAll(".r5-kpi,.kpi-strip,.r3-kpi,.r4-kpi,[class*=kpi]").length,rec:document.querySelectorAll(".r6-rec,[class*=r6-rec]").length,ai:(document.body.innerText||"").indexOf("AI")>=0,recText:((document.querySelector(".r6-rec,[class*=r6-rec]")||{}).innerText||"").slice(0,240),localTag:(document.body.innerText||"").indexOf("本地规则引擎")>=0,emptyHonest:(document.body.innerText||"").indexOf("NO_HISTORY")>=0||(document.body.innerText||"").indexOf("数据不足")>=0,bodyLen:document.body.innerText.length,probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length,recs:(p.recResponses||[]).map(function(r){return r.status})}})()})
EOF
done
echo "=== STEP4 index honest damage text ==="
$AB open "$BASE/pages/index/index" >/dev/null 2>&1
sleep 4
$AB screenshot "$DIR/screenshots/r6-online-h5-index-honest.png" >/dev/null 2>&1
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/INDEX: /"
JSON.stringify({url:location.href,hasToken:!!localStorage.getItem("lyp_token"),falseClaim:(document.body.innerText.indexOf("与面板攻击力无关")>=0||document.body.innerText.indexOf("无关攻击力")>=0||document.body.innerText.indexOf("全靠攻击力")>=0),honest:(document.body.innerText.indexOf("权重上限")>=0||document.body.innerText.indexOf("占比受限")>=0||document.body.innerText.indexOf("元素侧")>=0),bodyLen:document.body.innerText.length,probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
echo H5_DONE