#!/bin/bash
# R3 admin 四视图真浏览器复查（非物理真机，桌面 Chrome 1258x588）
# 口令由 ADMIN_PASS 环境变量注入，脚本/日志/报告均不写真值
S="lyp-admin-r3"
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round3-after"
PROBE="$DIR/probe-init.js"
AB="agent-browser --session $S"
PASS="${ADMIN_PASS:-}"
echo "=== STEP1 open login + inject probe ==="
$AB --init-script "$PROBE" open "$BASE/admin/login" >/dev/null 2>&1
sleep 2
# 清残留登录态，确保走真实登录
cat <<'EOF' | $AB eval --stdin >/dev/null 2>&1
localStorage.clear(); 'cleared'
EOF
$AB reload >/dev/null 2>&1
sleep 2
$AB get url 2>&1 | sed 's/^/LOGIN_URL: /'
echo "=== STEP2 login ==="
$AB fill "input[placeholder='admin']" "admin" >/dev/null 2>&1
$AB fill "input[placeholder='至少 10 位']" "$PASS" >/dev/null 2>&1
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/CLICK: /'
(function(){var bs=document.querySelectorAll('button');for(var i=0;i<bs.length;i++){if((bs[i].innerText||'').trim()==='登录'){bs[i].click();return 'clicked login btn idx='+i;}}return 'login btn not found';})()
EOF
sleep 3
$AB get url 2>&1 | sed 's/^/AFTER_LOGIN_URL: /'
echo "=== STEP3 four views ==="
for V in levels skills users recomputes; do
  $AB open "$BASE/admin/$V" >/dev/null 2>&1
  sleep 2
  $AB get url 2>&1 | sed 's/^/URL: /'
  cat <<EOF | $AB eval --stdin 2>&1 | sed 's/^/DOM: /'
JSON.stringify({url:location.href,title:document.title,page:document.querySelectorAll('.page').length,kicker:document.querySelectorAll('.section-kicker').length,kickerCn:(document.querySelector('.section-kicker-cn')?.innerText||'').trim(),kickerEn:(document.querySelector('.section-kicker-en')?.innerText||'').trim(),kpiStrip:document.querySelectorAll('.kpi-strip').length,kpiMain:document.querySelectorAll('.kpi-main > *').length,kpiSub:document.querySelectorAll('.kpi-sub > *').length,filterBar:document.querySelectorAll('.filter-bar').length,resultCount:(document.querySelector('.result-count')?.innerText||'').trim(),dataGrid:document.querySelectorAll('.data-grid').length,rows:document.querySelectorAll('.el-table__body-wrapper tbody tr').length,aiInsight:document.querySelectorAll('.ai-insight').length,aiText:(document.querySelector('.ai-insight')?.innerText||'').slice(0,180).replace(/\n/g,' '),bodyLen:document.body.innerText.length,probe:window.__LYP_PROBE__?{pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length,fr:window.__LYP_PROBE__.failedRequests.length}:null})
EOF
  $AB screenshot "$DIR/screenshots/r3-admin-$V.png" 2>&1 | sed 's/^/SHOT: /'
done
echo "=== DONE ==="
