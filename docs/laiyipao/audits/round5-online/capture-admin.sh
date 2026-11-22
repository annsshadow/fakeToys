#!/bin/bash
# t56 R5 线上独立回归 admin dashboard+四视图+命令面板+顶栏脱敏（真值内存，输出全脱敏，非物理真机 1258x588）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS" | head -1 | sed "s/^DOMAIN=//;s/[[:space:]]*$//" | tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS" | head -1 | sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//" | tr -d "\r")"
if [ -z "$DOMAIN" ]; then echo DOMAIN_MISSING; exit 2; fi
echo "PASS_LEN=${#ADMIN_PASS}"
BASE="https://$DOMAIN/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round5-online"
S="lyp-r5online-admin"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo "=== STEP1 login ==="
$AB --init-script "$DIR/probe-init.js" open "$BASE/admin/login" >/dev/null 2>&1
sleep 6
$AB set viewport 1258 588 >/dev/null 2>&1
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){try{localStorage.clear()}catch(e){};return true})()
EOF
$AB fill "input[placeholder='admin']" "admin" >/dev/null 2>&1
$AB fill "input[placeholder='至少 10 位']" "$ADMIN_PASS" >/dev/null 2>&1
cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){var bs=[].slice.call(document.querySelectorAll("button"));for(var i=0;i<bs.length;i++){if((bs[i].innerText||"").trim().indexOf("登录")>=0){bs[i].click();return "clicked"}}return "no-btn"})()
EOF
sleep 7
$AB get url 2>&1 | mask | sed "s/^/LOGIN_URL: /"
echo "=== STEP2 five views: dashboard skills users recomputes battles ==="
for V in dashboard skills users recomputes battles; do
  $AB open "$BASE/admin/$V" >/dev/null 2>&1
  sleep 5
  $AB screenshot "$DIR/screenshots/r5-online-admin-$V.png" >/dev/null 2>&1
  echo "--- $V ---"
  $AB get url 2>&1 | mask | sed "s/^/URL: /"
  $AB get title 2>&1 | mask | sed "s/^/TITLE: /"
  cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/DOM: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/admin/login")>=0,title:document.title,apiTag:(document.querySelector(".api-tag")||{}).innerText||"(none)",domainInBody:(function(){var t=document.body.innerText;var m=t.match(/[a-zA-Z0-9-]+\.[a-z]{2,}/g)||[];for(var i=0;i<m.length;i++){if(m[i].indexOf("localhost")<0&&m[i].indexOf("127.0.0.1")<0)return true}return false})(),kicker:document.querySelectorAll(".section-kicker").length,kickerCn:(document.querySelector(".section-kicker")||{}).innerText||"",kpiStrip:document.querySelectorAll(".kpi-strip").length,kpiCards:document.querySelectorAll(".kpi-strip > *, .kpi-card, .kpi-primary > *").length,filterBar:document.querySelectorAll("[class*=filter-bar],.filter-row").length,grid:document.querySelectorAll("[class*=data-grid],.el-table").length,tables:document.querySelectorAll("table").length,rows:document.querySelectorAll("tbody tr").length,aiCard:document.querySelectorAll(".ai-insight-card,.ai-insight").length,aiBadge:(document.querySelector(".ai-badge,.ai-token")||{}).innerText||"",aiTitle:(document.querySelector(".ai-title,.ai-insight-title")||{}).innerText||"",aiHeadline:(document.querySelector(".ai-headline,.ai-insight-headline")||{}).innerText||"",aiEvidence:(document.querySelector(".ai-evidence,.ai-insight-evidence")||{}).innerText||"",aiCta:(document.querySelector(".ai-cta,.ai-insight-cta")||{}).innerText||"",bodyLen:document.body.innerText.length,probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
done
echo "=== STEP3 command palette Ctrl+K ==="
$AB open "$BASE/admin/dashboard" >/dev/null 2>&1
sleep 4
$AB press "Control+k" >/dev/null 2>&1
sleep 2
$AB screenshot "$DIR/screenshots/r5-online-admin-palette.png" >/dev/null 2>&1
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/PALETTE: /"
JSON.stringify({panel:document.querySelectorAll(".cmd-panel").length,aiGroup:document.querySelectorAll(".cmd-ai-group-label").length,aiOpts:document.querySelectorAll(".cmd-ai-option").length,ariaModal:(document.querySelector(".cmd-panel")||{}).getAttribute?document.querySelector(".cmd-panel").getAttribute("aria-modal"):null,focus:(document.activeElement||{}).className||""})
EOF
echo DONE