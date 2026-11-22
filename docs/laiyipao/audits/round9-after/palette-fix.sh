#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r9-admin; D='docs/laiyipao/audits/round9-after/'; SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round9-after/screenshots'
run(){ "$AB" --session "$S" "$@" 2>&1; }
echo '=== OPEN DASHBOARD ==='
run open 'http://127.0.0.1:5175/laiyipao-admin/#/admin/dashboard' >/dev/null 2>&1; sleep 4
echo "TRIGGER: $(run eval "document.querySelectorAll('.command-trigger').length" 2>&1|head -1)"
echo '=== CLICK TRIGGER ==='
run click '.command-trigger' >/dev/null 2>&1; sleep 2
echo "PALETTE: $("$AB" --session "$S" eval --stdin < "$D/palette-probe.js" 2>&1|head -1)"
run screenshot "$SHOT/r9-admin-palette-fixed.png" >/dev/null 2>&1; echo SHOT_DONE