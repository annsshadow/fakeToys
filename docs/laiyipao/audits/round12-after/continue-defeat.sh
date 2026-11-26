#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round12-after'
S=lyp-r12-defeat2
for i in $(seq 1 120); do
  R=$("$AB" --session $S eval --stdin < "$D/battle-tick.js" 2>&1 | head -1)
  echo "TICK$i=$R"
  case "$R" in *SETTLED*) break;; esac
  sleep 2
done
sleep 4
echo '=== SETTLE ==='; "$AB" --session $S get url 2>&1 | head -1
"$AB" --session $S eval --stdin < "$D/defeat-probe.js" 2>&1 | head -1
"$AB" --session $S screenshot "$D/screenshots/r12-repro-02-settle.png" 2>&1 | tail -1
echo CONT_DONE