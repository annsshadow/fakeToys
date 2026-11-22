#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round10-after'
PASS=$(grep -oP 'BOOTSTRAP_ADMIN_PASS",\s*"\K[^"]+' laiyipao/server/internal/config/config.go | head -1 | tr -d '\r')
echo "PASS_LEN=${#PASS}"
if [ ${#PASS} -ne 10 ]; then echo 'PASS_BAD'; exit 1; fi
echo '=== ADMIN ==='
"$AB" --session lyp-r10-admin --init-script "$D/probe-init.js" open 'http://127.0.0.1:5175/laiyipao-admin/' >/dev/null 2>&1
sleep 2
"$AB" --session lyp-r10-admin eval "localStorage.clear()" >/dev/null 2>&1
"$AB" --session lyp-r10-admin open 'http://127.0.0.1:5175/laiyipao-admin/#/admin/login' >/dev/null 2>&1
sleep 2
"$AB" --session lyp-r10-admin fill "input[placeholder='admin']" "admin" >/dev/null 2>&1
"$AB" --session lyp-r10-admin fill "input[placeholder*='10']" "$PASS" >/dev/null 2>&1
"$AB" --session lyp-r10-admin eval "(function(){var bs=document.querySelectorAll('button');for(var i=0;i<bs.length;i++){if((bs[i].innerText||'').trim()==='登录'){bs[i].click();return 'clicked'}}return 'no-btn'})()" 2>&1 | head -1
sleep 3
"$AB" --session lyp-r10-admin get url 2>&1 | head -1
"$AB" --session lyp-r10-admin eval --stdin < "$D/js-admin.js" 2>&1 | head -1
"$AB" --session lyp-r10-admin screenshot "$D/screenshots/r10-admin-dashboard.png" 2>&1 | head -1
echo '=== INDEX ==='
"$AB" --session lyp-r10-index --init-script "$D/auth-init.js" open 'http://127.0.0.1:5174/#/pages/index/index' >/dev/null 2>&1
sleep 4
"$AB" --session lyp-r10-index eval --stdin < "$D/js-index.js" 2>&1 | head -1
echo '=== BATTLE LEGEND ==='
"$AB" --session lyp-r10-legend --init-script "$D/auth-init.js" open 'http://127.0.0.1:5174/#/pages/battle/battle?level=1' >/dev/null 2>&1
sleep 4
"$AB" --session lyp-r10-legend eval --stdin < "$D/js-aoe.js" 2>&1 | head -1