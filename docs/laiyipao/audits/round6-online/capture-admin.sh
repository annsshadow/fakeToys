#!/bin/bash
# t61 R6 线上独立回归 admin 五视图+login+命令面板+顶栏脱敏（真值内存，输出全脱敏，非物理真机 1258x588）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS" | head -1 | sed "s/^DOMAIN=//;s/[[:space:]]*$//" | tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS" | head -1 | sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//" | tr -d "\r")"
if [ -z "$DOMAIN" ]; then echo DOMAIN_MISSING; exit 2; fi
BASE="https://$DOMAIN/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round6-online"
S="lyp-r6online-admin"; AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo "PASS_LEN=${#ADMIN_PASS}"
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
echo "=== STEP2 five views content/defenses/equipment/wiki/login ==="
for V in content defenses equipment wiki login; do
  if [ "$V" = "wiki" ]; then U="$BASE/wiki"; else U="$BASE/admin/$V"; fi
  $AB open "$U" >/dev/null 2>&1
  sleep 5
  $AB screenshot "$DIR/screenshots/r6-online-admin-$V.png" >/dev/null 2>&1
  echo "--- $V ---"
  $AB get url 2>&1 | mask | sed "s/^/URL: /"
  $AB get title 2>&1 | mask | sed "s/^/TITLE: /"
  cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/DOM: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/admin/login")>=0,title:document.title,apiTag:(document.querySelector(".api-tag")||{}).innerText||"(none)",domainInBody:(function(){var t=document.body.innerText;var m=t.match(/[a-zA-Z0-9-]+\.[a-z]{2,}/g)||[];for(var i=0;i<m.length;i++){if(m[i].indexOf("localhost")<0&&m[i].indexOf("127.0.0.1")<0)return true}return false})(),kicker:document.querySelectorAll(".section-kicker").length,kickerCn:(document.querySelector(".section-kicker")||{}).innerText||"",kpi:document.querySelectorAll(".kpi-primary > *, .kpi-secondary > *, .kpi-card, .kpi-strip > *").length,rows:document.querySelectorAll("tbody tr").length,aiCard:document.querySelectorAll(".ai-insight-card,.ai-insight").length,aiHeadline:(document.querySelector(".ai-headline")||{}).innerText||"",aiEvidence:(document.querySelector(".ai-evidence")||{}).innerText||"",aiCta:(document.querySelector(".ai-cta")||{}).innerText||"",localTag:((document.querySelector("#ai-insight-card")||{}).innerText||"").indexOf("本地规则引擎")>=0,emptyHonest:((document.querySelector("#ai-insight-card")||{}).innerText||"").indexOf("数据不足")>=0||((document.querySelector("#ai-insight-card")||{}).innerText||"").indexOf("NO_HISTORY")>=0,bodyLen:document.body.innerText.length,probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length,recs:(p.recResponses||[]).map(function(r){return r.status})}})()})
EOF
done
echo "=== STEP3 command palette ==="
$AB open "$BASE/admin/dashboard" >/dev/null 2>&1
sleep 4
$AB press "Control+k" >/dev/null 2>&1
sleep 2
cat <<EOF | $AB eval --stdin 2>&1 | mask | sed "s/^/PALETTE: /"
JSON.stringify({panel:document.querySelectorAll(".cmd-panel").length,aiGroup:document.querySelectorAll(".cmd-ai-group-label").length,aiOpts:document.querySelectorAll(".cmd-ai-option").length,ariaModal:(document.querySelector(".cmd-panel")||{}).getAttribute?((document.querySelector(".cmd-panel")||{}).getAttribute("aria-modal")):null,focused:(document.activeElement||{}).className||""})
EOF
$AB screenshot "$DIR/screenshots/r6-online-admin-palette.png" >/dev/null 2>&1
echo ADMIN_DONE