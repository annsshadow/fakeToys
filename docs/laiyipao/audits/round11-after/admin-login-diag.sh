#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
PASS=$(grep -oP 'BOOTSTRAP_ADMIN_PASS", "\K[^"]+' laiyipao/server/internal/config/config.go 2>/dev/null | head -1)
echo PASS_LEN=${#PASS}
S=lyp-r11-admdiag
"$AB" --session $S close >/dev/null 2>&1
"$AB" --session $S open 'about:blank' >/dev/null 2>&1
sleep 1
"$AB" --session $S set viewport 1258 588 >/dev/null 2>&1
"$AB" --session $S open 'http://127.0.0.1:5175/laiyipao-admin/#/admin/login' >/dev/null 2>&1
sleep 2
"$AB" --session $S eval 'localStorage.clear()' >/dev/null 2>&1
"$AB" --session $S open 'http://127.0.0.1:5175/laiyipao-admin/#/admin/login' >/dev/null 2>&1
sleep 2
echo INPUTS:; "$AB" --session $S eval '[].slice.call(document.querySelectorAll("input")).map(function(i){return i.placeholder})' 2>&1 | tail -1
echo SNAP:; "$AB" --session $S snapshot -i 2>&1 | grep -i -E 'textbox|button|admin|password|至少|登录' | head -12
"$AB" --session $S fill "input[placeholder='admin']" 'admin' 2>&1 | tail -1
echo U:; "$AB" --session $S get value "input[placeholder='admin']" 2>&1 | tail -1
"$AB" --session $S fill "input[placeholder='至少 10 位']" "$PASS" 2>&1 | tail -1
echo P:; "$AB" --session $S get value "input[placeholder='至少 10 位']" 2>&1 | tail -1
echo BTNS:; "$AB" --session $S eval '[].slice.call(document.querySelectorAll("button")).map(function(b){return (b.innerText||"").trim()}).filter(Boolean)' 2>&1 | tail -1
"$AB" --session $S eval 'var b=[].slice.call(document.querySelectorAll("button")).find(function(x){return (x.innerText||"").indexOf("登录")>=0});if(b)b.click();"clicked="+(!!b)' 2>&1 | tail -1
sleep 3
echo URL:; "$AB" --session $S get url 2>&1 | tail -1
echo DIAG_DONE