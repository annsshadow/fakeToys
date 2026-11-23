#!/bin/bash
AB=agent-browser
S=lyp-r16-admin
D=docs/laiyipao/audits/round16-admin-reaction-timeline
SH="$PWD/$D/screenshots"
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
run(){ "$AB" --session "$S" "$@" 2>&1; }
j(){ run eval --stdin < "$D/$1" 2>&1 | head -1; }
echo "=== LOGIN ==="
run close >/dev/null 2>&1
run --init-script "$D/init.js" open "$BASE/admin/login" >/dev/null 2>&1; sleep 3
run set viewport 1440 900 >/dev/null 2>&1
run eval "localStorage.removeItem('lyp_admin_token')" >/dev/null 2>&1
run open "$BASE/admin/login" >/dev/null 2>&1; sleep 2
run fill "input[placeholder='admin']" 'admin' >/dev/null 2>&1
run fill "input[placeholder='至少 10 位']" 'admin12345' >/dev/null 2>&1
run eval "(function(){var b=[].slice.call(document.querySelectorAll('button')).filter(function(e){return (e.innerText||'').trim().indexOf('登录')>=0});if(b.length){b[0].click();return 'CLICKED'}return 'NOBTN'})()" >/dev/null 2>&1
sleep 5
echo "URL: $(run get url | head -2 | tail -1)"
echo "=== BATTLES 378 (HAS TIMELINE) ==="
run open "$BASE/admin/battles" >/dev/null 2>&1; sleep 4
run screenshot "$SH/r16-01-battles-list.png" >/dev/null 2>&1
echo "FIND378: $(run eval --stdin < "$D/find-row.js" 2>&1 | head -1)"
echo "CLICK378: $(run click '[data-battle-id="378"] .el-button' 2>&1 | head -2 | tail -1)"
sleep 4
echo "DOM378: $(j probe.js)"
run screenshot "$SH/r16-02-timeline-top.png" >/dev/null 2>&1
run eval "document.querySelector('.el-drawer__body')?.scrollTop=600" >/dev/null 2>&1; sleep 1
echo "DOM378SCROLL: $(j probe.js)"
run screenshot "$SH/r16-03-timeline-scroll.png" >/dev/null 2>&1
echo "=== OLD BATTLE 41 (EMPTY) ==="
run eval "document.querySelector('.el-drawer__close-btn')?.click()" >/dev/null 2>&1; sleep 1
run eval --stdin < "$D/find-old.js" >/dev/null 2>&1
echo "CLICK41: $(run click '[data-battle-id="41"] .el-button' 2>&1 | head -2 | tail -1)"
sleep 4
echo "DOM41: $(j probe.js)"
run screenshot "$SH/r16-04-old-empty.png" >/dev/null 2>&1
echo E2E_DONE
