#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y"; S=lyp-r61-bag2
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/bag/bag" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "FILL=$(AB "$S" fill "input.preset-input" "QA预设D" 2>&1)"
echo "VAL=$(AB "$S" get value "input.preset-input" 2>&1)"
echo "CLICK=$(AB "$S" click "text=保存当前构筑" 2>&1)"
for n in 1 2 3 4 5 6; do
 R=$(AB "$S" eval "(function(){var n=document.querySelectorAll('[role=status],[role=alert]');var o=[];for(var k=0;k<n.length;k++){var t=String(n[k].innerText||'').trim();if(t)o.push(n[k].getAttribute('role')+':'+t.slice(0,40));}return o.join('|');})()" 2>&1)
 echo "POLL$n=$R"; sleep 0.15
done
AB "$S" close >/dev/null 2>&1 || true; echo BAG2_FINISH
