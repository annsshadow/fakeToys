#!/bin/bash
AB=agent-browser
S=lyp-r15-final
D=docs/laiyipao/audits/round15-reaction-timeline
SH="$PWD/$D/screenshots"
mkdir -p "$SH"
"$AB" --session $S close >/dev/null 2>&1
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 5
echo '=== BATTLE LOOP ==='
SETTLED=0
for i in $(seq 1 180); do
  ACT=$("$AB" --session $S eval --stdin 2>/dev/null <<'EOF'
(() => { const r=document.querySelector('.result-title'); if(r) return 'SETTLED'; const c=document.querySelector('.hand-card'); if(c){c.click();return 'CARD'} const s=[...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('全部不要了')); if(s){s.click();return 'SKIP'} return 'WAIT'; })()
EOF
)
  ACT=$(echo "$ACT" | tr -d '"\r')
  if [ "$ACT" = "SETTLED" ]; then echo "[$i] SETTLED"; SETTLED=1; break; fi
  if [ $((i % 10)) -eq 0 ]; then echo "[$i] $ACT"; fi
  sleep 1
done
echo "SETTLED_FLAG=$SETTLED"
echo '=== SETTLE STATE ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => { const p=window.__LYP_PROBE__||{}; const r=document.querySelector('.result-title'); const rb=[...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('查看本局战报')); return JSON.stringify({result:r?r.textContent.trim():'',hasReportBtn:!!rb,shim:p.qaShimApplied||0,settle:(p.settleResponses||[]).slice(-2),af:(p.apiFailures||[]).slice(-3),pe:(p.pageErrors||[]).slice(-3)}); })()
EOF
"$AB" --session $S screenshot "$SH/r15-final-01-settle.png" >/dev/null 2>&1
echo SHOT1
echo '=== CLICK REPORT ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => { const rb=[...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('查看本局战报')); if(rb){rb.click();return 'CLICKED'} return 'NO_BTN'; })()
EOF
sleep 5
echo '=== TIMELINE DOM ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => { const p=window.__LYP_PROBE__||{}; const tl=document.querySelector('.r15-reaction-timeline'); const rows=[...document.querySelectorAll('.r15-timeline-row')]; const empty=document.querySelector('.r15-timeline-empty'); return JSON.stringify({url:location.hash,hasTimeline:!!tl,rows:rows.length,first:rows.length?rows[0].textContent.trim().slice(0,90):'',last:rows.length?rows[rows.length-1].textContent.trim().slice(0,90):'',waves:rows.length?[...new Set(rows.map(r=>r.getAttribute('data-r15-wave')))].slice(0,6):[],empty:empty?empty.textContent.trim():'',trunc:!!document.querySelector('.r15-truncated-note'),detail:(p.battleDetailResponses||[]).slice(-1)}); })()
EOF
"$AB" --session $S screenshot "$SH/r15-final-02-timeline-top.png" >/dev/null 2>&1
echo SHOT2
"$AB" --session $S scroll down 700 >/dev/null 2>&1
sleep 1
"$AB" --session $S screenshot "$SH/r15-final-03-timeline-scroll.png" >/dev/null 2>&1
echo SHOT3
echo '=== SHA256 ==='
sha256sum "$SH"/r15-final-0*.png 2>/dev/null
echo '=== E2E DONE ==='
