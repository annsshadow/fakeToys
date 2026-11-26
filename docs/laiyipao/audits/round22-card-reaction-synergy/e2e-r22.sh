#!/bin/bash
AB=agent-browser
D=docs/laiyipao/audits/round22-card-reaction-synergy
S=r22-e2e
SH="$PWD/$D/screenshots"
mkdir -p "$SH" "$D/data"
echo '=== RESET SESSION ==='
"$AB" --session "$S" close >/dev/null 2>&1
"$AB" --session "$S" open about:blank >/dev/null 2>&1
sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$PWD/$D/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" >/dev/null 2>&1
sleep 5
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" reload >/dev/null 2>&1
sleep 5
echo '=== PUSH TO HAND ==='
for i in $(seq 1 50); do
  ST=$("$AB" --session "$S" eval --stdin 2>&1 <<'EOF'
(() => { const hc=document.querySelectorAll('.hand-card'); const r=document.querySelector('.result-title'); const skip=[...document.querySelectorAll('.btn')].find(b=>b.textContent.includes('全部不要了')); let act=''; if(hc.length>0) act='HAND'; else if(r) act='SETTLED'; else if(skip){skip.click();act='SKIP'} return JSON.stringify({act:act,hands:hc.length,result:r?r.textContent.trim():''}); })()
EOF
)
  echo "[$i] $ST"
  if echo "$ST" | grep -q '"act":"HAND"'; then echo HAND_REACHED; break; fi
  if echo "$ST" | grep -q SETTLED; then echo UNEXPECTED_SETTLED; break; fi
  sleep 1.2
done
echo '=== PROBE HAND ==='
"$AB" --session "$S" eval --stdin < "$PWD/$D/probe.js" > "$D/data/probe-hand.json" 2>&1
head -c 2500 "$D/data/probe-hand.json"
echo
"$AB" --session "$S" screenshot "$SH/r22-01-hand-synergy.png" >/dev/null 2>&1
echo SHOT1_DONE
echo '=== SCROLL SYNERGY CARD ==='
"$AB" --session "$S" eval --stdin 2>&1 <<'EOF'
(() => { const c=document.querySelector('.r22-synergy-card'); if(c){c.scrollIntoView({block:'center'});return 'SCROLLED'} return 'NO_CARD' })()
EOF
sleep 1
"$AB" --session "$S" screenshot "$SH/r22-02-synergy-card.png" >/dev/null 2>&1
echo SHOT2_DONE
echo '=== SCROLL AI NOTE ==='
"$AB" --session "$S" eval --stdin 2>&1 <<'EOF'
(() => { const n=document.querySelector('.r22-ai-note'); if(n){n.scrollIntoView({block:'center'});return 'SCROLLED'} return 'NO_NOTE' })()
EOF
sleep 1
"$AB" --session "$S" screenshot "$SH/r22-03-ai-note.png" >/dev/null 2>&1
echo SHOT3_DONE
echo '=== CLICK ONE CARD ==='
"$AB" --session "$S" eval --stdin 2>&1 <<'EOF'
(() => { const c=document.querySelector('.hand-card'); if(c){c.click();return 'CLICKED'} return 'NO_CARD' })()
EOF
sleep 2
"$AB" --session "$S" eval --stdin < "$PWD/$D/probe.js" > "$D/data/probe-after-card.json" 2>&1
echo AFTER_CARD_DONE
"$AB" --session "$S" screenshot "$SH/r22-04-after-card.png" >/dev/null 2>&1
echo SHOT4_DONE
echo '=== DONE ==='
