#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round22-card-reaction-synergy
S=r22-e2e
SH="$PWD/$D/screenshots"
echo === PUSH_TO_SETTLE ===
for i in $(seq 1 150); do
  ST=$("$AB" --session "$S" eval --stdin 2>&1 <<'EOF'
(() => { const hc=document.querySelectorAll('.hand-card'); const r=document.querySelector('.result-title'); const skip=[...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('全部不要了')); let act=''; if(r) act='SETTLED'; else if(hc.length) {hc[0].click();act='CARD'} else if(skip){skip.click();act='SKIP'} return JSON.stringify({act:act,hands:hc.length,result:r?r.textContent.trim():''}); })()
EOF
)
  if echo "$ST" | grep -q SETTLED; then echo "[$i] SETTLED $ST"; break; fi
  sleep 0.8
done
echo === HASH_PROBE ===
"$AB" --session "$S" eval --stdin 2>&1 <<'EOF'
(() => { const hn=document.querySelector('.hash-note'); const rt=document.querySelector('.result-title'); const body=document.body.textContent||''; const m=body.match(/[0-9a-f]{16}/g)||[]; return JSON.stringify({result:rt?rt.textContent.trim():'',hashNote:hn?hn.textContent.trim():'',hashMatches:m.slice(0,5)}); })()
EOF
"$AB" --session "$S" eval --stdin < "$PWD/$D/probe.js" > "$D/data/probe-settle.json" 2>&1
echo SETTLE_PROBE_DONE
"$AB" --session "$S" screenshot "$SH/r22-07-settle-hash.png" >/dev/null 2>&1
echo SHOT7_DONE