#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round17-verify-reaction-diff
S=lyp-r17-verify
SH="$PWD/$D/screenshots"
mkdir -p "$SH"
run(){ "$AB" --session "$S" "$@" 2>&1; }
echo '=== RESET ==='
run close >/dev/null 2>&1
run --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/verify/verify" >/dev/null 2>&1
sleep 5
run set viewport 390 844 >/dev/null 2>&1
echo '=== INIT STATE ==='
run eval --stdin <<'EOF'
(()=>{const p=window.__LYP_PROBE__||{};return JSON.stringify({url:location.hash,token:!!localStorage.getItem('lyp_token'),probe:!!window.__LYP_PROBE__,body:(document.body.innerText||'').slice(0,200)})})()
EOF
echo '=== FILL ID 378 ==='
run fill "input.inp" "378" >/dev/null 2>&1
sleep 1
echo '=== FETCH REPLAY ==='
run eval --stdin <<'EOF'
(()=>{var b=[].slice.call(document.querySelectorAll('.btn')).find(function(x){return (x.innerText||'').indexOf('取复现信息')>=0});if(b){b.click();return 'CLICKED'}return 'NOBTN'})()
EOF
sleep 4
echo '=== AFTER FETCH ==='
run eval --stdin <<'EOF'
(()=>{const p=window.__LYP_PROBE__||{};const b=document.body.innerText||'';return JSON.stringify({hasHash:!!document.querySelector('.hash'),hash:(document.querySelector('.hash')||{}).textContent||'',replay:p.replayResponses||[],af:(p.apiFailures||[]).slice(-3),pe:p.pageErrors.length})})()
EOF
echo '=== DO REPLAY ==='
run eval --stdin <<'EOF'
(()=>{var b=document.querySelector('#r5-replay-btn')||[].slice.call(document.querySelectorAll('.btn')).find(function(x){return (x.innerText||'').indexOf('重放')>=0});if(b){b.click();return 'CLICKED_REPLAY'}return 'NOBTN'})()
EOF
sleep 5
echo '=== R17 DIFF DOM ==='
run eval --stdin <<'EOF'
(()=>{const box=document.querySelector('.r17-reaction-diff');const rows=[].slice.call(document.querySelectorAll('.r17-diff-row'));const p=window.__LYP_PROBE__||{};return JSON.stringify({box:!!box,summary:(document.querySelector('.r17-diff-summary')||{}).textContent||'',firstDiff:(document.querySelector('.r17-first-diff')||{}).textContent||'',empty:(document.querySelector('.r17-diff-empty')||{}).textContent||'',trunc:(document.querySelector('.r17-truncated')||{}).textContent||'',rows:rows.length,firstRow:rows[0]?rows[0].textContent.trim().slice(0,160):'',idx:rows.map(function(r){return r.getAttribute('data-r17-index')}).slice(0,6),verify:p.verifyResponses||[],af:(p.apiFailures||[]).slice(-3),pe:p.pageErrors.length,ce:p.consoleErrors.length})})()
EOF
echo '=== SCREENSHOT TOP ==='
run screenshot "$SH/r17-01-verify-top.png" >/dev/null 2>&1
run eval "document.querySelector('.r17-reaction-diff')&&document.querySelector('.r17-reaction-diff').scrollIntoView()" >/dev/null 2>&1
sleep 1
run screenshot "$SH/r17-02-diff-card.png" >/dev/null 2>&1
echo '=== SHA ==='
sha256sum "$SH"/r17-0*.png
echo '=== DONE ==='
