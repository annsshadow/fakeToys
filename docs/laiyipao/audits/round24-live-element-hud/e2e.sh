#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round24-live-element-hud
S=lyp-r24-e2e
SH="$PWD/$D/screenshots"
DATA="$PWD/$D/data"
echo '=== RESET SESSION ==='
"$AB" --session $S close >/dev/null 2>&1
"$AB" --session $S open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 5
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1
"$AB" --session $S reload >/dev/null 2>&1
sleep 5
echo '=== CAPTURE LIVE HUD SAMPLES ==='
: > "$DATA/hud-samples.jsonl"
for i in $(seq 1 40); do
  ST=$("$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => {
  const hud=document.querySelector('.r24-element-hud');
  const rows=[...document.querySelectorAll('.r24-enemy-row')];
  const hints=[...document.querySelectorAll('.r24-reaction-hint')];
  const empty=document.querySelector('.r24-hud-empty');
  const concl=document.querySelector('.r24-ai-conclusion');
  const basis=document.querySelector('.r24-ai-basis');
  const honesty=document.querySelector('.r24-ai-honesty');
  const cta=document.querySelector('.r24-ai-cta');
  const r=document.querySelector('.result-title');
  const card=document.querySelector('.hand-card');
  const skip=[...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('全部不要了'));
  let act='';
  if(r) act='SETTLED';
  else if(card){card.click();act='CARD'}
  else if(skip){skip.click();act='SKIP'}
  return JSON.stringify({hud:!!hud,rows:rows.length,
    uids:rows.map(x=>x.getAttribute('data-r24-uid')),
    stacks:rows.map(x=>{const s=x.querySelector('.r24-enemy-stacks');return s?s.textContent.trim():''}),
    hints:hints.length,firstHint:hints.length?hints[0].textContent.trim():'',
    empty:empty?empty.textContent.trim():'',
    aiConcl:concl?concl.textContent.trim():'',
    aiTitle:document.querySelector('.r24-ai-title')?document.querySelector('.r24-ai-title').textContent.trim():'',
    basis:!!basis?basis.textContent.trim():'',honesty:!!honesty?honesty.textContent.trim():'',cta:cta?cta.textContent.trim():'',
    hasUndefined:document.body.innerText.includes('undefined'),hasNaN:document.body.innerText.includes('NaN'),
    act:act,result:r?r.textContent.trim():''});
})()
EOF
)
  echo "$ST" >> "$DATA/hud-samples.jsonl"
  echo "[$i] $(echo "$ST" | head -c 260)"
  if [ "$i" = "4" ]; then "$AB" --session $S screenshot "$SH/r24-01-hud-live-a.png" >/dev/null 2>&1; echo SHOT_A; fi
  if [ "$i" = "9" ]; then "$AB" --session $S screenshot "$SH/r24-02-hud-live-b.png" >/dev/null 2>&1; echo SHOT_B; fi
  if echo "$ST" | grep -q SETTLED; then echo BATTLE_DONE; break; fi
  sleep 1.2
done
echo '=== SETTLE STATE ==='
"$AB" --session $S eval --stdin 2>&1 <<'EOF'
(() => { const p=window.__LYP_PROBE__||{}; const r=document.querySelector('.result-title'); const hn=document.querySelector('.hash-note'); return JSON.stringify({result:r?r.textContent.trim():'',hash:hn?hn.textContent.trim().slice(0,80):'',settle:(p.settleResponses||[]).slice(-2),pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length,peList:(p.pageErrors||[]).slice(0,3),ceList:(p.consoleErrors||[]).slice(0,3)}); })()
EOF
"$AB" --session $S screenshot "$SH/r24-03-settle.png" >/dev/null 2>&1
echo SHOT_SETTLE
echo '=== DONE ==='