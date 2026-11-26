#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round14-advice-action-e2e
S=lyp-r14-rxbattle
SH="$PWD/$D/screenshots"
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 2
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open http://127.0.0.1:5174/#/pages/battles/battles >/dev/null 2>&1
sleep 3
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S reload >/dev/null 2>&1
sleep 7
"$AB" --session $S click '.battle-row.tap' >/dev/null 2>&1
sleep 3
echo '=== BATTLES NONREG DOM ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => { const q=(s)=>document.querySelectorAll(s).length; const p=window.__LYP_PROBE__||{};
  return JSON.stringify({url:location.hash,title:document.title,r14Advice:q('.r14-advice'),r14Cta:q('#r14-advice-action-btn'),
    r14Empty:q('.r14-advice-empty'),learning:q('.reaction-learning'),
    pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length,
    domainInBody:/https?:\/\//.test(document.body.innerText||'')});
})()
EOF
"$AB" --session $S screenshot "$SH/r14-rxfix-05-battles-nonreg.png" >/dev/null 2>&1
echo SHOT5=$?