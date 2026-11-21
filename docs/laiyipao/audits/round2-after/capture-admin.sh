#!/bin/bash
# R2 admin 标杆重构真浏览器复查（非物理真机，桌面 Chrome）
# 口令由 ADMIN_PASS 环境变量注入，脚本与报告均不写真值
S="lyp-admin-r2"
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round2-after"
PROBE="$DIR/probe-init.js"
AB="agent-browser --session $S"
PASS="${ADMIN_PASS:-}"
echo "=== STEP1 open login + inject probe ==="
$AB --init-script "$PROBE" open "$BASE/admin/login" >/dev/null 2>&1
sleep 2
$AB get url 2>&1 | sed 's/^/URL: /'
echo "=== STEP2 login ==="
$AB fill "input[placeholder='admin']" "admin" >/dev/null 2>&1
$AB fill "input[placeholder='至少 10 位']" "$PASS" >/dev/null 2>&1
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/CLICK: /'
(function(){var bs=document.querySelectorAll('button');for(var i=0;i<bs.length;i++){if((bs[i].innerText||'').trim()==='登录'){bs[i].click();return 'clicked login btn idx='+i;}}return 'login btn not found';})()
EOF
sleep 3
$AB get url 2>&1 | sed 's/^/AFTER_LOGIN_URL: /'
echo "=== STEP3 dashboard tokens + AI card ==="
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/DASH: /'
JSON.stringify({url:location.href,aiCard:document.querySelectorAll('.ai-insight-card,.ai-insight').length,aiText:(document.querySelector('.ai-insight-card,.ai-insight')?.innerText||'').slice(0,220),css:(function(){var s=getComputedStyle(document.documentElement);return {bg:s.getPropertyValue('--lyp-bg').trim()||s.getPropertyValue('--bg').trim(),accent:s.getPropertyValue('--lyp-accent').trim()||s.getPropertyValue('--fire').trim(),glass:s.getPropertyValue('--lyp-glass-panel').trim()};})(),probe:window.__LYP_PROBE__?{pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length}:null})
EOF
$AB screenshot "$DIR/screenshots/r2-admin-dashboard.png" 2>&1 | sed 's/^/SHOT: /'
echo "=== STEP4 command palette AI section ==="
$AB press Control+k >/dev/null 2>&1
sleep 1
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/PALETTE: /'
JSON.stringify({panel:document.querySelectorAll('.cmd-panel').length,aiGroup:document.querySelectorAll('.cmd-ai-group-label').length,aiOptions:document.querySelectorAll('.cmd-ai-option').length,navOptions:document.querySelectorAll('.cmd-option').length,ariaModal:document.querySelector('.cmd-panel')?.getAttribute('aria-modal'),aiLabel:(document.querySelector('.cmd-ai-group-label')?.innerText||'').trim(),firstAi:(document.querySelector('.cmd-ai-option')?.innerText||'').trim()})
EOF
$AB screenshot "$DIR/screenshots/r2-admin-command-ai.png" 2>&1 | sed 's/^/SHOT: /'
echo "=== STEP5 close palette ==="
$AB press Escape >/dev/null 2>&1
echo "=== DONE ==="
