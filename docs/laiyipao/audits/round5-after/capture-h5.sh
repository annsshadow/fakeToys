#!/bin/bash
# t49 R5 H5 五页 + 诚实文案复查（本地回环 QA，非物理真机 390x844）
AB="agent-browser --session lyp-r5after-h5"
BASE="http://127.0.0.1:5174/#"
DIR="docs/laiyipao/audits/round5-after"
echo "=== open + probe + login ==="
$AB --init-script "$DIR/probe-init.js" open "$BASE/pages/login/login" >/dev/null 2>&1
sleep 4
$AB set viewport 390 844 >/dev/null 2>&1
$AB fill "input[type='text']" "admin" >/dev/null 2>&1
$AB click ".btn-primary" >/dev/null 2>&1
sleep 4
cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/AUTH: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/login/login")>=0,hasToken:!!localStorage.getItem("lyp_token")})
EOF
echo "=== five R5 pages ==="
for P in announcements login signin stage verify; do
  $AB open "$BASE/pages/$P/$P" >/dev/null 2>&1
  sleep 4
  $AB screenshot "$DIR/screenshots/r5-h5-$P.png" >/dev/null 2>&1
  echo "--- $P ---"
  $AB get url 2>&1 | sed "s/^/URL: /"
  $AB get title 2>&1 | sed "s/^/TITLE: /"
  cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/DOM: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/login/login")>=0,title:document.title,hasToken:!!localStorage.getItem("lyp_token"),kicker:document.querySelectorAll(".section-kicker").length,kickerCn:(document.querySelector(".section-kicker")||{}).innerText||"",kpi:document.querySelectorAll(".r5-kpi,.kpi-strip,.r3-kpi,.r4-kpi").length,ai:document.querySelectorAll(".r5-ai,.r4-ai,.r3-ai,[class*=ai-insight]").length,aiBadge:(document.querySelector(".ai-badge,.ai-token,[class*=ai-badge]")||{}).innerText||"",aiText:(document.querySelector("[class*=ai-insight],[class*=ai-card],[class*=ai-hint]")||{}).innerText||"",empty:(document.querySelector("[class*=empty],[class*=empty-state]")||{}).innerText||"",filter:document.querySelectorAll(".r5-filter,.chip-row,[class*=filter-bar]").length,grid:document.querySelectorAll("[class*=data-grid],[class*=r5-grid]").length,bodyLen:document.body.innerText.length,probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
done
echo "=== index honest damage text ==="
$AB open "$BASE/pages/index/index" >/dev/null 2>&1
sleep 4
cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/INDEX: /"
JSON.stringify({url:location.href,hasToken:!!localStorage.getItem("lyp_token"),bodyText:(document.body.innerText.match(/反应伤害[^
]{0,100}/g)||[]).slice(0,6),falseClaim:(document.body.innerText.indexOf("与面板攻击力无关")>=0||document.body.innerText.indexOf("无关攻击力")>=0),honest:(document.body.innerText.indexOf("占比受限")>=0||document.body.innerText.indexOf("权重上限")>=0),probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
echo DONE