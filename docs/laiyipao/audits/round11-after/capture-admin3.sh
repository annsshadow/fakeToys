#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round11-after'
PASS=$(grep -oP 'BOOTSTRAP_ADMIN_PASS", "\K[^"]+' laiyipao/server/internal/config/config.go 2>/dev/null | head -1)
echo PASS_LEN=${#PASS}
S=lyp-r11-admin-final
"$AB" --session $S close >/dev/null 2>&1
"$AB" --session $S open 'about:blank' >/dev/null 2>&1
sleep 2
"$AB" --session $S set viewport 1258 588 >/dev/null 2>&1
sleep 1
"$AB" --session $S --init-script "$D/probe-init.js" open 'http://127.0.0.1:5175/laiyipao-admin/#/admin/login' >/dev/null 2>&1
sleep 2
"$AB" --session $S eval 'localStorage.clear()' >/dev/null 2>&1
"$AB" --session $S open 'http://127.0.0.1:5175/laiyipao-admin/#/admin/login' >/dev/null 2>&1
sleep 2
"$AB" --session $S fill "input[placeholder='admin']" 'admin' >/dev/null 2>&1
"$AB" --session $S fill "input[placeholder='至少 10 位']" "$PASS" >/dev/null 2>&1
"$AB" --session $S eval 'var b=[].slice.call(document.querySelectorAll("button")).find(function(x){return (x.innerText||"").indexOf("登录")>=0});if(b)b.click();"clicked="+(!!b)' >/dev/null 2>&1
sleep 3
echo -n 'URL: '; "$AB" --session $S get url 2>&1 | tail -1
echo -n 'VP: '; "$AB" --session $S eval 'window.innerWidth+"x"+window.innerHeight' 2>&1 | tail -1
echo -n 'PROBE: '; "$AB" --session $S eval 'typeof window.__LYP_PROBE__' 2>&1 | tail -1
echo '=== ADMIN DASHBOARD ==='
"$AB" --session $S eval --stdin < "$D/admin-probe.js" 2>&1 | head -1
"$AB" --session $S screenshot "$D/screenshots/r11-admin-dashboard.png" 2>&1 | head -1
echo ADMIN_FINAL_DONE