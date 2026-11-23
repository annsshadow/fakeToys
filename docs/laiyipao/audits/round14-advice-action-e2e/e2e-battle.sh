#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round14-advice-action-e2e
S=lyp-r14-e2e
SH="$PWD/$D/screenshots"
echo '=== BEFORE DB ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "select slot||':'||skill_id from user_skill_slots where user_id=10 order by slot" 2>/dev/null|tr -d '\r'|paste -sd, -
"$AB" --session $S screenshot "$SH/r14-battle-01-cta-ready.png" >/dev/null 2>&1
echo SHOT1=$?
"$AB" --session $S click '#r14-advice-action-btn' >/dev/null 2>&1
sleep 2
echo '=== MODAL ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => { const m=document.querySelector('.r7-compare-mask'); const t=document.querySelector('.r7-compare-title'); const rows=document.querySelectorAll('.r7-vs-row'); const btns=document.querySelectorAll('.r7-btn'); const p=window.__LYP_PROBE__||{}; return JSON.stringify({mask:!!m, title:t?t.textContent.trim():'', rows:rows.length, rowText:rows.length?rows[0].textContent.trim().slice(0,120):'', btns:Array.from(btns).map(b=>b.textContent.trim()), pe:(p.pageErrors||[]).length}); })()
EOF
"$AB" --session $S screenshot "$SH/r14-battle-02-compare.png" >/dev/null 2>&1
echo SHOT2=$?
"$AB" --session $S click '.r7-btn.primary' >/dev/null 2>&1
sleep 4
echo '=== AFTER APPLY DOM ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => { const done=document.querySelectorAll('.apply-done'); const err=document.querySelector('.r7-error'); const p=window.__LYP_PROBE__||{}; return JSON.stringify({done:Array.from(done).map(d=>d.textContent.trim()), err:err?err.textContent.trim():'', adv:(p.adviceResponses||[]).slice(-2), pe:(p.pageErrors||[]).length, ce:(p.consoleErrors||[]).length}); })()
EOF
"$AB" --session $S screenshot "$SH/r14-battle-03-applied.png" >/dev/null 2>&1
echo SHOT3=$?
echo '=== AFTER DB ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "select slot||':'||skill_id from user_skill_slots where user_id=10 order by slot" 2>/dev/null|tr -d '\r'|paste -sd, -