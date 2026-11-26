#!/bin/bash
# 测试环境 admin 复查（真值从 .local.keys 读入内存，输出全部脱敏）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E '^DOMAIN=' "$KEYS" | head -1 | sed 's/^DOMAIN=//;s/[[:space:]]*$//' | tr -d '\r')"
ADMIN_PASS="$(grep -E '^ADMIN_PASS=' "$KEYS" | head -1 | sed 's/^ADMIN_PASS=//;s/[[:space:]]*$//' | tr -d '\r')"
ADMIN_USER="admin"
BASE="https://$DOMAIN/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round3-postdeploy"
PROBE="$DIR/probe-init.js"
S="lyp-postadmin"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g" | sed -E 's#https://[^ ]*<YOUR_TEST_DOMAIN>#https://<YOUR_TEST_DOMAIN>#g'; }
echo "=== STEP1 open login + probe ==="
$AB --init-script "$PROBE" open "$BASE/admin/login" >/dev/null 2>&1
sleep 3
$AB get url 2>&1 | mask | sed 's/^/LOGIN_URL: /'
cat <<'EOF' | $AB eval --stdin >/dev/null 2>&1
localStorage.clear();'cleared'
EOF
$AB reload >/dev/null 2>&1
sleep 3
echo "=== STEP2 login ==="
$AB fill "input[placeholder='admin']" "$ADMIN_USER" >/dev/null 2>&1
$AB fill "input[placeholder='至少 10 位']" "$ADMIN_PASS" >/dev/null 2>&1
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/CLICK: /'
(function(){var bs=document.querySelectorAll('button');for(var i=0;i<bs.length;i++){if((bs[i].innerText||'').trim()==='登录'){bs[i].click();return 'clicked idx='+i;}}return 'not found';})()
EOF
sleep 4
$AB get url 2>&1 | mask | sed 's/^/AFTER_LOGIN: /'
echo "=== STEP3 dashboard ==="
cat <<'EOF' | $AB eval --stdin 2>&1 | mask | sed 's/^/DASH: /'
JSON.stringify({url:location.href,title:document.title,aiCard:(document.querySelector('.ai-insight-card')?.innerText||document.querySelector('.ai-insight')?.innerText||'').slice(0,200).replace(/\n/g,' '),kickers:[...document.querySelectorAll('.section-kicker')].map(function(e){return e.innerText.trim().slice(0,40)}),charts:document.querySelectorAll('canvas,.echarts,.chart').length,kpiMain:document.querySelectorAll('.kpi-main').length,kpiSub:document.querySelectorAll('.kpi-sub-card').length,probe:window.__LYP_PROBE__?{pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length,fr:window.__LYP_PROBE__.failedRequests.length}:null})
EOF
$AB screenshot "$DIR/screenshots/post-admin-dashboard.png" >/dev/null 2>&1
echo "dashboard shot saved"
echo "=== STEP4 four views ==="
for V in levels skills users recomputes; do
  $AB open "$BASE/admin/$V" >/dev/null 2>&1
  sleep 3
  $AB get url 2>&1 | mask | sed 's/^/URL: /'
  cat <<EOF | $AB eval --stdin 2>&1 | mask | sed 's/^/DOM: /'
JSON.stringify({url:location.href,title:document.title,page:document.querySelectorAll('.page').length,kicker:document.querySelectorAll('.section-kicker').length,kickerCn:(document.querySelector('.section-kicker-cn')?.innerText||'').trim(),kpiMain:document.querySelectorAll('.kpi-main').length,kpiSub:document.querySelectorAll('.kpi-sub-card').length,filterBar:document.querySelectorAll('.filter-bar').length,dataGrid:document.querySelectorAll('.data-grid').length,rows:document.querySelectorAll('.el-table__body-wrapper tbody tr').length,aiInsight:document.querySelectorAll('.ai-insight').length,resultCount:(document.querySelector('.result-count')?.innerText||'').trim(),bodyLen:document.body.innerText.length,probe:window.__LYP_PROBE__?{pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length,fr:window.__LYP_PROBE__.failedRequests.length}:null})
EOF
  $AB screenshot "$DIR/screenshots/post-admin-$V.png" >/dev/null 2>&1
  echo "$V shot saved"
done
echo "=== STEP5 command palette AI ==="
$AB open "$BASE/admin/dashboard" >/dev/null 2>&1
sleep 2
$AB press Control+k >/dev/null 2>&1
sleep 1
cat <<'EOF' | $AB eval --stdin 2>&1 | mask | sed 's/^/PALETTE: /'
JSON.stringify({panel:document.querySelectorAll('.cmd-panel').length,aiGroup:document.querySelectorAll('.cmd-ai-group-label').length,aiOptions:document.querySelectorAll('.cmd-ai-option').length,ariaModal:document.querySelector('.cmd-panel')?.getAttribute('aria-modal'),focusInInput:document.activeElement?.className==='cmd-input'})
EOF
$AB screenshot "$DIR/screenshots/post-admin-palette.png" >/dev/null 2>&1
echo "palette shot saved"
echo "=== DONE ==="
