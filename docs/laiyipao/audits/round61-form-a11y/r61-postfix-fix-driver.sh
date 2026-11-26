#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y/data"; S=lyp-r61-pf-fix
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
AB "$S" open --init-script "$D/auth-init.js" "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "ME_SNAP_BEFORE=$(AB "$S" snapshot -i 2>&1 | head -20)"
echo "ME_CLICK=$(AB "$S" click "text=设置账号密码" 2>&1)"; sleep 1
echo "ME_SNAP_AFTER=$(AB "$S" snapshot -i 2>&1 | head -35)"
AB "$S" snapshot --json > "$D/pf-me-upgrade-ax.json" 2>&1
echo "ME_FILL1=$(AB "$S" fill "input" "qa_pf_user" 2>&1)"
echo "ME_FILL2=$(AB "$S" fill "input:nth-of-type(2)" "QaPfPass123456" 2>&1)"
echo "ME_SUBMIT=$(AB "$S" click "text=确认升级" 2>&1)"; sleep 3
echo "ME_MSG=$(AB "$S" eval "(function(){var n=document.querySelectorAll('[role=status],[role=alert]');var o=[];for(var k=0;k<n.length;k++){var t=String(n[k].innerText||'').trim();if(t)o.push(n[k].getAttribute('role')+':'+t.slice(0,60));}return o.join('|');})()" 2>&1)"
AB "$S" snapshot --json > "$D/pf-fb-ax-me-upgrade.json" 2>&1
AB "$S" open --init-script "$D/auth-init.js" "http://127.0.0.1:5201/#/pages/bag/bag" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "BAG_INPUT=$(AB "$S" get count input 2>&1)"
echo "BAG_HTML=$(AB "$S" eval "(function(){var i=document.querySelector('input');return i?i.outerHTML:'NONE';})()" 2>&1)"
echo "BAG_SNAP=$(AB "$S" snapshot -i 2>&1 | head -45)"
echo "BAG_SAVE_CLICK=$(AB "$S" click "text=保存当前构筑" 2>&1)"; sleep 1
echo "BAG_STATUS=$(AB "$S" eval "(function(){var n=document.querySelectorAll('[role=status],[role=alert]');var o=[];for(var k=0;k<n.length;k++){var t=String(n[k].innerText||'').trim();if(t)o.push(n[k].getAttribute('role')+':'+t.slice(0,60));}return o.join('|');})()" 2>&1)"
AB "$S" snapshot --json > "$D/pf-fb-ax-bag-live.json" 2>&1
AB "$S" close >/dev/null 2>&1 || true
echo PF_FIX_FINISH