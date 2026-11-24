#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round20-combat-efficiency-trends
S=lyp-r20-e2e
SH="$PWD/$D/screenshots"
echo '=== RESET ==='
"$AB" --session $S close >/dev/null 2>&1
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open 'http://127.0.0.1:5174/#/pages/battles/battles' >/dev/null 2>&1
sleep 6
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S reload >/dev/null 2>&1
sleep 6
echo '=== H5 EFFICIENCY DOM ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => { var p=window.__R20_PROBE__||{}; var card=document.querySelector('.r20-efficiency-trend'); var row=card&&card.querySelector('.r20-efficiency-row'); var bars=[].slice.call(document.querySelectorAll('.r20-trend-bar')); var ai=card&&card.querySelector('.r20-ai-note'); var empty=card&&card.querySelector('.r20-efficiency-empty'); return JSON.stringify({url:location.hash,title:document.title,hasCard:!!card,role:card&&card.getAttribute('role'),windowNote:card&&card.querySelector('.r20-eff-window')?card.querySelector('.r20-eff-window').textContent.trim():'',row:row?row.textContent.trim().replace(/\s+/g,' '):'',bars:bars.map(function(b){return {bucket:b.getAttribute('data-r20-bucket'),text:b.textContent.trim().replace(/\s+/g,' ')}}),aiTitle:ai&&ai.querySelector('.r20-ai-title')?ai.querySelector('.r20-ai-title').textContent.trim():'',aiConclusion:ai&&ai.querySelector('.r20-ai-conclusion')?ai.querySelector('.r20-ai-conclusion').textContent.trim():'',aiBasis:ai&&ai.querySelector('.r20-ai-basis')?ai.querySelector('.r20-ai-basis').textContent.trim():'',aiCta:ai&&ai.querySelector('[data-r20-cta]')?ai.querySelector('[data-r20-cta]').textContent.trim():'',honesty:card&&card.querySelector('.r20-honest-note')?card.querySelector('.r20-honest-note').textContent.trim():'',empty:empty?empty.textContent.trim().replace(/\s+/g,' '):'',responses:(p.efficiencyResponses||[]).slice(-3),pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length,peDetail:(p.pageErrors||[]).slice(0,2),ceDetail:(p.consoleErrors||[]).slice(0,2)}); })()
EOF
"$AB" --session $S screenshot "$SH/r20-01-h5-efficiency.png" >/dev/null 2>&1
echo SHOT1_DONE
"$AB" --session $S scroll down 400 >/dev/null 2>&1
sleep 1
"$AB" --session $S screenshot "$SH/r20-02-h5-scroll.png" >/dev/null 2>&1
echo SHOT2_DONE
echo '=== DONE ==='
