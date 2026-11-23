#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round12-after'
S=lyp-r12-admin2
PASS=$(grep -oP 'BOOTSTRAP_ADMIN_PASS", "\K[^"]+' 'D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/laiyipao/server/internal/config/config.go' 2>/dev/null|head -1)
echo PASS_LEN=${#PASS}
[ ${#PASS} -ne 10 ] && { echo PASS_LEN_BAD; exit 1; }
sed "s|__ADMIN_PASS__|$PASS|g" "$D/admin-fill2.tpl.js" > "$D/admin-fill2-run.js"
"$AB" --session $S open 'about:blank' >/dev/null 2>&1; sleep 2
"$AB" --session $S set viewport 1258 588 >/dev/null 2>&1; sleep 1
"$AB" --session $S open 'http://127.0.0.1:5175/laiyipao-admin/' >/dev/null 2>&1; sleep 3
"$AB" --session $S eval 'localStorage.clear()' >/dev/null 2>&1
"$AB" --session $S open 'http://127.0.0.1:5175/laiyipao-admin/' >/dev/null 2>&1; sleep 3
echo FILL=$("$AB" --session $S eval --stdin < "$D/admin-fill2-run.js" 2>&1|head -1)
echo CLICK=$("$AB" --session $S eval --stdin < "$D/admin-click.js" 2>&1|head -1)
sleep 3
echo URL=$("$AB" --session $S get url 2>&1|head -1)
echo TOPBAR=$("$AB" --session $S eval --stdin < "$D/nr-admin-probe.js" 2>&1|head -1)
echo POPEN=$("$AB" --session $S click '.command-trigger' 2>&1|head -1)
sleep 1
echo PALETTE=$("$AB" --session $S eval 'JSON.stringify({panel:document.querySelectorAll(".cmd-panel").length,aiGroup:document.querySelectorAll(".cmd-ai-group-label").length,aiOpts:document.querySelectorAll(".cmd-ai-option").length,aria:document.querySelector(".cmd-panel")?document.querySelector(".cmd-panel").getAttribute("aria-modal"):null})' 2>&1|head -1)
echo NR2_DONE