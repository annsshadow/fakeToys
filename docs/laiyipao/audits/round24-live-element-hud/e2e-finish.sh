#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round24-live-element-hud
S=lyp-r24-e2e
SH=$PWD/$D/screenshots
echo '=== CONTINUE BATTLE TO SETTLE ==='
for i in $(seq 1 70); do
  ST=$("$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => {
  const r=document.querySelector('.result-title');
  if(r) return JSON.stringify({act:'SETTLED',result:r.textContent.trim()});
  const card=document.querySelector('.hand-card');
  const skip=[...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('全部不要了'));
  if(card){card.click();return JSON.stringify({act:'CARD'});}
  if(skip){skip.click();return JSON.stringify({act:'SKIP'});}
  return JSON.stringify({act:'WAIT'});
})()
EOF
)
  echo "[$i] $ST"
  if echo "$ST" | grep -q SETTLED; then echo BATTLE_DONE; break; fi
  sleep 1
done
echo '=== SETTLE PROBE ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => {
  const p=window.__LYP_PROBE__||{};
  const r=document.querySelector('.result-title');
  const hn=document.querySelector('.hash-note');
  const raw=(p.pageErrors||[]);
  const noise=x=>/play\(\) failed|goo\.gl\/xX8pDD|Autoplay/i.test(x);
  return JSON.stringify({result:r?r.textContent.trim():'',hash:hn?hn.textContent.trim().slice(0,120):'',
    settle:(p.settleResponses||[]).slice(-2),
    peRaw:raw.length,peReal:raw.filter(x=>!noise(x)).length,
    peKinds:[...new Set(raw.map(x=>noise(x)?'AUTOPLAY_NOISE':String(x).slice(0,90)))].slice(0,8),
    ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length});
})()
EOF
"$AB" --session $S screenshot "$SH/r24-04-settled.png" >/dev/null 2>&1
echo SHOT_SETTLED
echo '=== DONE ==='