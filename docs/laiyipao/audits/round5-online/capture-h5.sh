#!/bin/bash
# t56 R5 线上独立回归 H5 七页（真值内存注入+输出全脱敏，非物理真机 390x844）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS" | head -1 | sed "s/^DOMAIN=//;s/[[:space:]]*$//" | tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS" | head -1 | sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//" | tr -d "\r")"
if [ -z "$DOMAIN" ]; then echo DOMAIN_MISSING; exit 2; fi
BASE="https://$DOMAIN/laiyipao-h5/#"
DIR="docs/laiyipao/audits/round5-online"
S="lyp-r5online-h5"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo "=== STEP1 open login + probe + viewport ==="
$AB --init-script "$DIR/probe-init.js" open "$BASE/pages/login/login" >/dev/null 2>&1
sleep 6
$AB set viewport 390 844 >/dev/null 2>&1
echo "=== STEP2 REAL login (native click .btn-primary) ==="
$AB fill "input[type='text']" "admin" >/dev/null 2>&1
$AB fill "input[type='password']" "$ADMIN_PASS" >/dev/null 2>&1
$AB click ".btn-primary" >/dev/null 2>&1
sleep 6
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/AUTH: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/login/login")>=0,hasToken:!!localStorage.getItem("lyp_token"),hasProfile:!!localStorage.getItem("lyp_profile")})
EOF
echo "=== STEP3 seven pages: login announcements signin stage verify index reactions ==="
for P in login announcements signin stage verify index reactions; do
  $AB open "$BASE/pages/$P/$P" >/dev/null 2>&1
  sleep 5
  $AB screenshot "$DIR/screenshots/r5-online-h5-$P.png" >/dev/null 2>&1
  echo "--- $P ---"
  $AB get url 2>&1 | mask | sed "s/^/URL: /"
  $AB get title 2>&1 | mask | sed "s/^/TITLE: /"
  cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/DOM: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/login/login")>=0,title:document.title,hasToken:!!localStorage.getItem("lyp_token"),kicker:document.querySelectorAll(".section-kicker").length,kickerCn:(document.querySelector(".section-kicker")||{}).innerText||"",kpi:document.querySelectorAll(".r5-kpi,.kpi-strip,.r3-kpi,.r4-kpi").length,ai:document.querySelectorAll(".r5-ai,.r4-ai,.r3-ai,[class*=ai-insight],[class*=ai-card],[class*=ai-hint]").length,aiBadge:(document.querySelector(".ai-badge,.ai-token,[class*=ai-badge]")||{}).innerText||"",aiText:((document.querySelector("[class*=ai-insight],[class*=ai-card],[class*=ai-hint]")||{}).innerText||"").slice(0,160),emptyBlock:(document.querySelector(".empty-block")||{}).innerText||"",r3Empty:document.querySelectorAll(".r3-empty").length,filter:document.querySelectorAll(".chip-row,[class*=filter-bar]").length,grid:document.querySelectorAll("[class*=data-grid],[class*=r5-grid]").length,bodyLen:document.body.innerText.length,probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
done
echo "=== STEP4 index honest damage text + AoE 25% ==="
$AB open "$BASE/pages/index/index" >/dev/null 2>&1
sleep 4
$AB screenshot "$DIR/screenshots/r5-online-h5-index-honest.png" >/dev/null 2>&1
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/INDEX: /"
JSON.stringify({url:location.href,hasToken:!!localStorage.getItem("lyp_token"),falseClaim:(document.body.innerText.indexOf("与面板攻击力无关")>=0||document.body.innerText.indexOf("无关攻击力")>=0),honest:(document.body.innerText.indexOf("占比受限")>=0||document.body.innerText.indexOf("权重上限")>=0),honestLine:(function(){var a=document.body.innerText.split(String.fromCharCode(10));for(var i=0;i<a.length;i++){if(a[i].indexOf("反应伤害")>=0&&a[i].indexOf("攻击力")>=0)return a[i].slice(0,160)}return ""})(),probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
$AB open "$BASE/pages/battle/battle" >/dev/null 2>&1
sleep 4
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/LEGEND_BEFORE: /"
JSON.stringify({url:location.href,legendBtn:document.querySelectorAll("[aria-label='战场图例']").length,pct25:(document.body.innerText.match(/25%/g)||[]).length})
EOF
echo DONE