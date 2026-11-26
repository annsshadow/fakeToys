#!/bin/bash
DIR="docs/laiyipao/audits/round6-after"; S=lyp-r6-h5; AB="agent-browser --session $S"
$AB open "http://127.0.0.1:5174/#/pages/index/index" >/dev/null 2>&1
sleep 4
$AB eval --stdin 2>&1 <<'EOF' | sed "s/^/IDX: /"
(function(){var b=document.body.innerText||"";var p=window.__LYP_PROBE__||{};var honest=b.indexOf("元素侧")>=0||b.indexOf("占比受限")>=0||b.indexOf("权重上限")>=0;var falseClaim=/攻击力.{0,6}主力|全靠攻击力|攻击力.{0,4}唯一/.test(b);return JSON.stringify({url:location.href,honest:honest,falseClaim:falseClaim,bodyLen:b.length,snippet:b.slice(0,500),pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length})})()
EOF
$AB screenshot "$DIR/screenshots/r6-h5-index-honest.png" >/dev/null 2>&1
echo R1R5_DONE