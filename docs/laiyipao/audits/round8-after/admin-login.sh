#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r8-admin; D='docs/laiyipao/audits/round8-after/'
run(){ "$AB" --session "$S" "$@" 2>&1; }
run eval "localStorage.clear()" >/dev/null
run open 'http://127.0.0.1:5175/laiyipao-admin/#/admin/login' >/dev/null; sleep 3
run fill "input[placeholder='admin']" 'admin' >/dev/null
PW=$(grep -oE 'BOOTSTRAP_ADMIN_PASS" *, *"[^"]+' laiyipao/server/internal/config/config.go | head -1 | sed -E 's/.*"([^"]+)$/\1/'); [ ${#PW} -eq 10 ] || PW='admin12345'
run fill "input[placeholder='至少 10 位']" "$PW" >/dev/null
run eval "(function(){var b=[].slice.call(document.querySelectorAll('button')).filter(function(e){return (e.innerText||'').trim().indexOf('登录')>=0});if(b.length){b[0].click();return 'CLICKED'}return 'NOBTN'})()" >/dev/null
sleep 5
echo "URL: $(run get url | head -2 | tail -1)"
echo "ADMIN_LOGGED: $(run eval --stdin < "$D/admin-probe.js" 2>&1 | head -1)"