#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round12-after'
PASS=$(grep -oP 'BOOTSTRAP_ADMIN_PASS", "\K[^"]+' laiyipao/server/internal/config/config.go 2>/dev/null | head -1)
if [ ${#PASS} -ne 10 ]; then echo PASS_LEN_BAD=${#PASS}; fi
S=lyp-r12-admin
"$AB" --session $S open 'about:blank' >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 1258 588 >/dev/null 2>&1; sleep 1
"$AB" --session $S --init-script "$D/probe-init.js" open "http://127.0.0.1:5175/laiyipao-admin/#/admin/login" >/dev/null 2>&1; sleep 2
"$AB" --session $S eval 'localStorage.clear()' >/dev/null 2>&1
"$AB" --session $S fill "input[placeholder='admin']" 'admin' >/dev/null 2>&1
"$AB" --session $S fill "input[placeholder='\u81f3\u5c11 10 \u4f4d']" "$PASS" >/dev/null 2>&1
"$AB" --session $S eval '[].slice.call(document.querySelectorAll("button")).find(function(b){return(b.innerText||"").indexOf("\u767b\u5f55")>=0})&&[].slice.call(document.querySelectorAll("button")).find(function(b){return(b.innerText||"").indexOf("\u767b\u5f55")>=0}).click()' >/dev/null 2>&1
sleep 3
echo "=== LOGIN URL ==="; "$AB" --session $S get url 2>&1 | head -1
for V in dashboard skills users battles; do
  echo "=== $V ==="
  "$AB" --session $S open "http://127.0.0.1:5175/laiyipao-admin/#/admin/$V" >/dev/null 2>&1
  sleep 3
  "$AB" --session $S get url 2>&1 | head -1
  "$AB" --session $S get title 2>&1 | head -1
  "$AB" --session $S eval --stdin < "$D/admin-probe.js" 2>&1 | head -1
  "$AB" --session $S screenshot "$D/screenshots/r12-admin-$V.png" 2>&1 | tail -1
done
echo ADMIN_DONE