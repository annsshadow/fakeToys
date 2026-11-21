#!/bin/bash
# t49 R5 admin 四视图复查（本地回环 QA，非物理真机 1258x588）
# admin 口令取源码公开默认值 config.go:599（非真值），长度校验
AB="agent-browser --session lyp-r5after-admin"
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round5-after"
PASS="$(grep -oE 'BOOTSTRAP_ADMIN_PASS", "[^"]+"' laiyipao/server/internal/config/config.go 2>/dev/null | head -1 | sed 's/.*"\([^"]*\)"/\1/')"
if [ ${#PASS} -ne 10 ]; then PASS="admin12345"; fi
echo "PASS_LEN=${#PASS}"
echo "=== login ==="
$AB --init-script "$DIR/probe-init.js" open "$BASE/admin/login" >/dev/null 2>&1
sleep 4
$AB set viewport 1258 588 >/dev/null 2>&1
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){try{localStorage.clear()}catch(e){};return true})()
EOF
$AB fill "input[placeholder='admin']" "admin" >/dev/null 2>&1
$AB fill "input[placeholder='至少 10 位']" "$PASS" >/dev/null 2>&1
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){var bs=[].slice.call(document.querySelectorAll("button"));for(var i=0;i<bs.length;i++){if((bs[i].innerText||"").trim().indexOf("登录")>=0){bs[i].click();return "clicked"}}return "no-btn"})()
EOF
sleep 5
$AB get url 2>&1 | sed "s/^/LOGIN_URL: /"
echo "=== four views ==="
for V in skills users recomputes battles; do
  $AB open "$BASE/admin/$V" >/dev/null 2>&1
  sleep 4
  $AB screenshot "$DIR/screenshots/r5-admin-$V.png" >/dev/null 2>&1
  echo "--- $V ---"
  $AB get url 2>&1 | sed "s/^/URL: /"
  $AB get title 2>&1 | sed "s/^/TITLE: /"
  cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/DOM: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/admin/login")>=0,title:document.title,apiTag:(document.querySelector(".api-tag")||{}).innerText||"(none)",kicker:document.querySelectorAll(".section-kicker").length,kickerCn:(document.querySelector(".section-kicker")||{}).innerText||"",kpiStrip:document.querySelectorAll(".kpi-strip").length,kpiCards:document.querySelectorAll(".kpi-strip > *, .kpi-card, .kpi-primary > *").length,filterBar:document.querySelectorAll("[class*=filter-bar],.filter-row").length,grid:document.querySelectorAll("[class*=data-grid],.el-table").length,tables:document.querySelectorAll("table").length,rows:document.querySelectorAll("tbody tr").length,aiCard:document.querySelectorAll(".ai-insight-card,.ai-insight").length,aiBadge:(document.querySelector(".ai-badge,.ai-token")||{}).innerText||"",aiTitle:(document.querySelector(".ai-title,.ai-insight-title")||{}).innerText||"",aiHeadline:(document.querySelector(".ai-headline,.ai-insight-headline")||{}).innerText||"",aiEvidence:(document.querySelector(".ai-evidence,.ai-insight-evidence")||{}).innerText||"",aiCta:(document.querySelector(".ai-cta,.ai-insight-cta")||{}).innerText||"",empty:(document.querySelector("[class*=empty-state],[class*=el-empty]")||{}).innerText||"",bodyLen:document.body.innerText.length,probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
done
echo DONE