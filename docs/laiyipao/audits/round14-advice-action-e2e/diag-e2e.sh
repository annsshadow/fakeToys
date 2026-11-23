#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round14-advice-action-e2e
S=lyp-r14-e2e
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
echo '=== URL ==='
"$AB" --session $S get url 2>&1 | tail -1
echo '=== DIAG ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => { 
  const p = window.__LYP_PROBE__ || {};
  const tk = (localStorage.getItem('lyp_token') || '').length;
  return JSON.stringify({ vw: innerWidth, vh: innerHeight, tokenLen: tk, url: location.href, adv: (p.adviceResponses||[]).slice(0,8), pe:(p.pageErrors||[]).length, ce:(p.consoleErrors||[]).length, af:(p.apiFailures||[]).length });
})()
EOF
