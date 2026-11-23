#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round14-advice-action-e2e
S=lyp-r14-e2e
"$AB" --session $S eval --stdin >/dev/null 2>&1 <<'EOF'
document.querySelectorAll('.battle-row.tap').length && document.querySelector('.battle-row.tap').scrollIntoView(); true
EOF
"$AB" --session $S click '.battle-row.tap' >/dev/null 2>&1
sleep 4
echo '=== DOM AFTER EXPAND ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => {
  const p=window.__LYP_PROBE__||{};
  const adv=document.querySelector('.r14-advice');
  const cta=document.querySelector('#r14-advice-action-btn');
  const empty=document.querySelector('.r14-advice-empty');
  const learn=document.querySelector('.reaction-learning-text');
  const det=document.querySelector('.battle-detail');
  return JSON.stringify({detailOpen:!!det, hasAdvice:!!adv, cta:!!cta, ctaText:cta?cta.textContent.trim():'', empty:!!empty, emptyMsg:empty?empty.textContent.trim().slice(0,80):'', learn:learn?learn.textContent.trim().slice(0,80):'', adv:(p.adviceResponses||[]).slice(0,4), pe:(p.pageErrors||[]).length, ce:(p.consoleErrors||[]).length});
})()
EOF