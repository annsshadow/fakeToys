#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round12-after'
S=lyp-r12-defeat
"$AB" --session $S open 'about:blank' >/dev/null 2>&1; sleep 1
"$AB" --session $S set viewport 390 844 >/dev/null 2>&1; sleep 1
"$AB" --session $S --init-script "$D/auth-init.js" open 'http://127.0.0.1:5174/?r12=7#/pages/battle/battle?level=40' >/dev/null 2>&1
sleep 5
echo '=== START ==='; "$AB" --session $S get url 2>&1 | head -1
"$AB" --session $S screenshot "$D/screenshots/r12-repro-01-start.png" 2>&1 | tail -1
# 逐波真实点击「全部不要了，直接下一波」8 次
for i in 1 2 3 4 5 6 7 8; do
  sleep 2
  R=$("$AB" --session $S click "xpath=//*[contains(text(),'全部不要了')]" 2>&1 | head -1)
  echo "CLICK$i=$R"
done
sleep 4
echo '=== SETTLE ==='; "$AB" --session $S get url 2>&1 | head -1
"$AB" --session $S eval --stdin < "$D/defeat-probe.js" 2>&1 | head -1
"$AB" --session $S screenshot "$D/screenshots/r12-repro-02-settle.png" 2>&1 | tail -1
echo '=== ERRORS ==='; "$AB" --session $S errors --json 2>&1 | head -c 500
echo DEFEAT_DONE