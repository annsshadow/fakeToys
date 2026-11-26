#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y"; S=lyp-r61-bag
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/bag/bag" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "FILL=$(AB "$S" eval "(function(){var i=document.querySelector('input.preset-input');if(!i)return 'NO_INPUT';var setter=Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype,'value').set;setter.call(i,'QA预设C'+Date.now());i.dispatchEvent(new Event('input',{bubbles:true}));return i.value;})()" 2>&1)"
echo "CLICK=$(AB "$S" eval --stdin < "$D/trig-bag-save.js" 2>&1)"
for n in 1 2 3 4 5 6 7 8 9 10; do
 R=$(AB "$S" eval "(function(){var n=document.querySelectorAll('[role=status],[role=alert]');var o=[];for(var k=0;k<n.length;k++){var t=String(n[k].innerText||'').trim();if(t)o.push(n[k].getAttribute('role')+':'+t.slice(0,40));}return o.join('|');})()" 2>&1)
 echo "POLL$n=$R"
 sleep 0.2
done
AB "$S" close >/dev/null 2>&1 || true; echo BAG_INFLIGHT_FINISH
