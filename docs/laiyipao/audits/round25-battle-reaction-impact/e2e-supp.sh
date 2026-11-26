#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round25-battle-reaction-impact
S=lyp-r25-e2e
SH="$PWD/$D/screenshots"
echo '=== OLD EMPTY SCROLL battle 339 ==='
"$AB" --session $S close >/dev/null 2>&1
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battles/battles?battle_id=339" >/dev/null 2>&1
sleep 6
"$AB" --session $S eval --stdin <<'EOF'
(() => { var el=document.querySelector('.r25-reaction-impact'); if(el){el.scrollIntoView({block:'center'});return 'SCROLLED_EMPTY'} return 'NO_CARD' })()
EOF
sleep 1
"$AB" --session $S screenshot "$SH/r25-04-old-empty-scroll.png" >/dev/null 2>&1
echo SHOT_04
echo '--- PROBE WITH EXPANDED HASH ---'
"$AB" --session $S eval --stdin <<'EOF'
(() => { var p=window.__LYP_PROBE__||{}; var exp=document.querySelector('.battle-row.expanded'); var hash=exp&&exp.querySelector('.hash')?exp.querySelector('.hash').textContent.trim():''; var card=document.querySelector('.r25-reaction-impact'); return JSON.stringify({expandedHash:hash,empty:card&&card.querySelector('.r25-impact-empty')?card.querySelector('.r25-impact-empty').textContent.trim():'',timelineRows:document.querySelectorAll('.r15-timeline-row').length,allDetail:(p.detailResponses||[]).map(function(x){return {id:x.battle_id,tlLen:x.tlLen}}),pe:(p.pageErrors||[]).filter(function(x){return !/Autoplay|play\(\)/i.test(x)}).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}); })()
EOF
echo '=== DONE ==='
