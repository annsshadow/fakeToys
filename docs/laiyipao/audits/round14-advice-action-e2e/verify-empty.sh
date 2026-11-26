#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round14-advice-action-e2e
S=lyp-r14-empty
SH="$PWD/$D/screenshots"
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 2
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open http://127.0.0.1:5174/#/pages/battles/battles >/dev/null 2>&1
sleep 6
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S reload >/dev/null 2>&1
sleep 6
"$AB" --session $S click '.battle-row.tap' >/dev/null 2>&1
sleep 3
echo '=== EMPTY STATE DOM ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => { const p=window.__LYP_PROBE__||{}; const empty=document.querySelector('.r14-advice-empty'); const cta=document.querySelector('#r14-advice-action-btn'); const adv=document.querySelector('.r14-advice'); return JSON.stringify({hasAdvice:!!adv,hasEmpty:!!empty,emptyMsg:empty?empty.textContent.trim().slice(0,100):'',hasCta:!!cta,adv:(p.adviceResponses||[]).slice(0,3),pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length}); })()
EOF
"$AB" --session $S screenshot "$SH/r14-battle-05-empty-no-cta.png" >/dev/null 2>&1