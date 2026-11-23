#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round14-advice-action-e2e
S=lyp-r14-rx
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 2
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open http://127.0.0.1:5174/#/pages/reactions/reactions >/dev/null 2>&1
sleep 6
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
sleep 1
"$AB" --session $S reload >/dev/null 2>&1
sleep 6
echo '=== URL ==='
"$AB" --session $S get url 2>&1 | tail -1
echo '=== RX DIAG ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => {
  const p=window.__LYP_PROBE__||{};
  const ctas=document.querySelectorAll('#r14-advice-action-btn');
  const empties=document.querySelectorAll('.r14-advice-empty');
  return JSON.stringify({vw:innerWidth,tokenLen:(localStorage.getItem('lyp_token')||'').length,ctaCount:ctas.length,firstCta:ctas.length?ctas[0].textContent.trim():'',emptyCount:empties.length,adv:(p.adviceResponses||[]).slice(0,8),pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length});
})()
EOF