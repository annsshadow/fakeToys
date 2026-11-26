#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r9-admin; BASE='http://127.0.0.1:5175/laiyipao-admin/#'; D='docs/laiyipao/audits/round9-after/'; SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round9-after/screenshots'
run(){ "$AB" --session "$S" "$@" 2>&1; }
j(){ "$AB" --session "$S" eval --stdin < "$D/$1" 2>&1 | head -1; }
echo '=== LOGIN ==='
run --init-script "$D/probe-init.js" open "$BASE/admin/dashboard" >/dev/null 2>&1; sleep 4
run set viewport 1258 588 >/dev/null 2>&1
run eval "localStorage.clear()" >/dev/null 2>&1
run open "$BASE/admin/login" >/dev/null 2>&1; sleep 3
PW=$(grep -oE 'BOOTSTRAP_ADMIN_PASS" *, *"[^"]+' laiyipao/server/internal/config/config.go | head -1 | sed -E 's/.*"([^"]+)$/\1/'); [ ${#PW} -eq 10 ] || PW='admin12345'
run fill "input[placeholder='admin']" 'admin' >/dev/null 2>&1
run fill "input[placeholder='至少 10 位']" "$PW" >/dev/null 2>&1
run eval "(function(){var b=[].slice.call(document.querySelectorAll('button')).filter(function(e){return (e.innerText||'').trim().indexOf('登录')>=0});if(b.length){b[0].click();return 'CLICKED'}return 'NOBTN'})()" >/dev/null 2>&1
sleep 5
echo "URL: $(run get url | head -2 | tail -1)"
echo '=== DASHBOARD ==='
echo "PROBE: $(j admin-probe.js)"
run screenshot "$SHOT/r9-admin-dashboard.png" >/dev/null 2>&1
echo '=== WIKI (t89 route fix) ==='
run open "$BASE/wiki" >/dev/null 2>&1; sleep 5
echo "PROBE: $(j admin-probe.js)"
run screenshot "$SHOT/r9-admin-wiki.png" >/dev/null 2>&1
echo '=== COMMAND PALETTE ==='
run press 'Control+k' >/dev/null 2>&1; sleep 2
echo "PALETTE: $(j palette-probe.js)"
run screenshot "$SHOT/r9-admin-palette.png" >/dev/null 2>&1
echo ADMIN_DONE