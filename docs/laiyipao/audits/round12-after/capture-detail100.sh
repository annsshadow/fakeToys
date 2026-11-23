#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round12-after'
S=lyp-r12-detail100
for i in $(seq 1 30); do [ "$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:8080/healthz 2>/dev/null)" = 200 ] && break; sleep 2; done
"$AB" --session $S open 'about:blank' >/dev/null 2>&1; sleep 2
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; sleep 1
"$AB" --session $S --init-script "$D/auth100.js" open 'http://127.0.0.1:5174/#/pages/battles/battles' >/dev/null 2>&1; sleep 6
echo '=== BATTLES LIST ==='; "$AB" --session $S get url 2>&1|head -1
"$AB" --session $S get title 2>&1|head -1
echo LIST=$("$AB" --session $S eval 'JSON.stringify({len:(document.body.innerText||"").length,lose:(document.body.innerText||"").indexOf("失败")>=0})' 2>&1|head -1)
"$AB" --session $S screenshot "$D/screenshots/r12-detail100-list.png" 2>&1|tail -1
echo DETAIL_DONE