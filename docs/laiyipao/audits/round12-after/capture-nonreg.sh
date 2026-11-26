#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round12-after'
S=lyp-r12-nr
# 游客态新会话：清token后重进首页，验首屏引导卡
"$AB" --session $S open 'about:blank' >/dev/null 2>&1; sleep 2
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; sleep 1
"$AB" --session $S --init-script "$D/probe-init.js" open 'http://127.0.0.1:5174/#/pages/index/index' >/dev/null 2>&1; sleep 5
echo VW=$("$AB" --session $S eval 'window.innerWidth' 2>&1|head -1)
echo '=== HOME VISITOR ==='; "$AB" --session $S get url 2>&1|head -1
"$AB" --session $S eval --stdin < "$D/nonreg-probe.js" 2>&1|head -1
"$AB" --session $S screenshot "$D/screenshots/r12-nr-home-visitor.png" 2>&1|tail -1
echo '=== ERRORS ==='; "$AB" --session $S errors --json 2>&1|head -c 300
echo NR_DONE