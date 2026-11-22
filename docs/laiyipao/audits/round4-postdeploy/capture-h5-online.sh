#!/bin/bash
# t44 线上 H5 六页回归（真值内存，输出全脱敏，非物理真机 390x844）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS" | head -1 | sed "s/^DOMAIN=//;s/[[:space:]]*$//" | tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS" | head -1 | sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//" | tr -d "\r")"
BASE="https://$DOMAIN/laiyipao-h5/#"
DIR="docs/laiyipao/audits/round4-postdeploy"
S="lyp-r4online-h5"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo "=== STEP1 open login + probe + viewport ==="
$AB --init-script "$DIR/probe-init.js" open "$BASE/pages/login/login" >/dev/null 2>&1
sleep 5
$AB set viewport 390 844 >/dev/null 2>&1
echo "=== STEP2 REAL login (native click .btn-primary) ==="
$AB fill "input[type='text']" "admin" >/dev/null 2>&1
$AB fill "input[type='password']" "$ADMIN_PASS" >/dev/null 2>&1
$AB click ".btn-primary" >/dev/null 2>&1
sleep 5
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/AUTH: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/login/login")>=0,hasToken:!!localStorage.getItem("lyp_token"),hasProfile:!!localStorage.getItem("lyp_profile")})
EOF
echo "=== STEP3 six pages ==="
for P in bag battles enemies me rank skins; do
  $AB open "$BASE/pages/$P/$P" >/dev/null 2>&1
  sleep 4
  $AB screenshot "$DIR/screenshots/r4-online-h5-$P.png" >/dev/null 2>&1
  echo "--- $P ---"
  $AB get url 2>&1 | mask | sed "s/^/URL: /"
  $AB get title 2>&1 | mask | sed "s/^/TITLE: /"
  cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/DOM: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/login/login")>=0,title:document.title,hasToken:!!localStorage.getItem("lyp_token"),kicker:document.querySelectorAll(".section-kicker").length,kpi:document.querySelectorAll(".r3-kpi,.r4-kpi").length,ai:document.querySelectorAll(".r3-ai,.r4-ai").length,aiBadge:(document.querySelector(".r3-ai-badge,.ai-badge")||{}).innerText||"",aiConclusion:(document.querySelector(".r3-ai-conclusion")||{}).innerText||"",aiBasis:(document.querySelector(".r3-ai-basis")||{}).innerText||"",aiCta:(document.querySelector(".r3-ai-cta")||{}).innerText||"",filter:document.querySelectorAll(".r3-filter,.r4-filter,.chip-row").length,grid:document.querySelectorAll(".r3-data-grid,.r4-grid").length,bodyLen:document.body.innerText.length,probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
done
echo "=== STEP4 AoE legend check (battle page) ==="
$AB open "$BASE/pages/stage/stage" >/dev/null 2>&1
sleep 3
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/STAGE: /"
JSON.stringify({url:location.href,hasToken:!!localStorage.getItem("lyp_token"),bodyLen:document.body.innerText.length})
EOF
echo DONE