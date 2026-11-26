#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round14-advice-action-e2e
S=lyp-r14-rxscroll
SH="$PWD/$D/screenshots"
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 2
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open http://127.0.0.1:5174/#/pages/reactions/reactions >/dev/null 2>&1
sleep 3
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S reload >/dev/null 2>&1
sleep 8
echo '=== SCROLL TO R14 CARD ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => { const el=document.querySelector('.r14-advice'); if(!el) return 'NO_CARD'; el.scrollIntoView({block:'center'}); return JSON.stringify({top:Math.round(el.getBoundingClientRect().top),cta:document.querySelectorAll('#r14-advice-action-btn').length}); })()
EOF
sleep 2
"$AB" --session $S screenshot "$SH/r14-rxfix-06-card-visible.png" >/dev/null 2>&1
echo SHOT6=$?