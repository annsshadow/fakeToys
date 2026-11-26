#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y/data"; S=lyp-r61-pf-bag
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
AB "$S" open --init-script "$D/auth-init.js" "http://127.0.0.1:5201/#/pages/bag/bag" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "TABS=$(AB "$S" get text ".tab-bar" 2>&1 | head -5)"
echo "CLICK_SKILLS=$(AB "$S" click ".tab-bar >> text=技能" 2>&1)"
sleep 1
echo "INPUT_COUNT=$(AB "$S" get count input 2>&1)"
echo "CLICK_FOCUS=$(AB "$S" click "text=点这里填名字保存一套" 2>&1)"
sleep 1
echo "FILL=$(AB "$S" fill "input.preset-input" "QA预设PF" 2>&1)"
echo "CLICK_SAVE=$(AB "$S" click "text=保存当前构筑" 2>&1)"
for n in 1 2 3 4 5 6 7 8 9 10; do R=$(AB "$S" eval "(function(){var n=document.querySelectorAll('[role=status],[role=alert]');var o=[];for(var k=0;k<n.length;k++){var t=String(n[k].innerText||'').trim();if(t)o.push(n[k].getAttribute('role')+':'+t.slice(0,40));}return o.join('|');})()" 2>&1); echo "POLL$n=$R"; sleep 0.12; done
AB "$S" close >/dev/null 2>&1 || true
echo PF_BAG_FINISH