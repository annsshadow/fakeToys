#!/bin/bash
# t38 R4 真浏览器复查 - admin 四视图+看板（本地回环，非物理真机）
PASS="$(grep -rhoP 'BOOTSTRAP_ADMIN_PASS"\s*,\s*"\K[^"]+' laiyipao/server --include=*.go | head -1 | tr -d '\r')"
if [ ${#PASS} -ne 10 ]; then echo "PASS_LEN_BAD=${#PASS}"; exit 2; fi
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round4-after"
S="lyp-r4after-admin"
AB="agent-browser --session $S"
echo "=== STEP1 login ==="
$AB --init-script "$DIR/probe-init.js" open "$BASE/admin/login" >/dev/null 2>&1
sleep 4
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){try{localStorage.clear()}catch(e){};return true})()
EOF
$AB set viewport 1258 588 >/dev/null 2>&1
$AB fill "input[placeholder='admin']" "admin" >/dev/null 2>&1
$AB fill "input[placeholder='至少 10 位']" "$PASS" >/dev/null 2>&1
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){var bs=[].slice.call(document.querySelectorAll("button"));for(var i=0;i<bs.length;i++){if((bs[i].innerText||"").trim().indexOf("登录")>=0){bs[i].click();return "clicked"}}return "no-btn"})()
EOF
sleep 5
$AB get url 2>&1 | sed "s/^/LOGIN_URL: /"
echo "=== STEP2 views ==="
for V in dashboard content economy defenses equipment; do
  $AB open "$BASE/admin/$V" >/dev/null 2>&1
  sleep 4
  $AB screenshot "$DIR/screenshots/r4-admin-$V.png" >/dev/null 2>&1
  echo "--- $V ---"
  $AB get url 2>&1 | sed "s/^/URL: /"
  $AB get title 2>&1 | sed "s/^/TITLE: /"
  cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/DOM: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/admin/login")>=0,title:document.title,kicker:document.querySelectorAll(".section-kicker").length,kickerCn:(document.querySelector(".section-kicker-cn")||{}).innerText||"",kpi:document.querySelectorAll(".kpi-strip,.kpi-main,.kpi-primary").length,ai:document.querySelectorAll(".r3-ai,.r4-ai,.ai-insight").length,aiBadge:(document.querySelector(".ai-badge,.r3-ai-badge,.ai-insight-badge")||{}).innerText||"",aiConclusion:(document.querySelector(".ai-conclusion,.r3-ai-conclusion,.ai-insight-headline")||{}).innerText||"",tables:document.querySelectorAll("table").length,rows:document.querySelectorAll("tbody tr").length,bodyLen:document.body.innerText.length,probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
done
echo "=== STEP3 palette ==="
$AB open "$BASE/admin/dashboard" >/dev/null 2>&1
sleep 3
$AB press "Control+k" >/dev/null 2>&1
sleep 1
$AB screenshot "$DIR/screenshots/r4-admin-palette.png" >/dev/null 2>&1
cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/PALETTE: /"
JSON.stringify({panel:document.querySelectorAll(".cmd-panel").length,aiGroup:document.querySelectorAll(".cmd-ai-group-label").length,aiOpts:document.querySelectorAll(".cmd-ai-option").length,ariaModal:(document.querySelector(".cmd-panel")||{}).getAttribute?document.querySelector(".cmd-panel").getAttribute("aria-modal"):null})
EOF
echo DONE