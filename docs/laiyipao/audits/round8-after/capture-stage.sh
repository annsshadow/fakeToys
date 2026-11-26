#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
S=lyp-r8-h5; BASE='http://127.0.0.1:5174/#'; D='docs/laiyipao/audits/round8-after/'; SHOT='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round8-after/screenshots'
run(){ "$AB" --session "$S" "$@" 2>&1; }
j(){ "$AB" --session "$S" eval --stdin < "$D/$1" 2>&1 | head -1; }
echo '=== L98 SCREENSHOT ==='
run screenshot "$SHOT/r8-L98-star-rule-milestones.png" >/dev/null; echo L98_SHOT_DONE
echo '=== L1 ==='
run open "$BASE/pages/stage/stage?level=1" >/dev/null; sleep 4
run screenshot "$SHOT/r8-L1-stage.png" >/dev/null
echo "L1: $(j stage-probe.js)"
echo '=== BOSS LEVELS (milestone coverage) ==='
for L in 56 60 91 95 96 100; do
  run open "$BASE/pages/stage/stage?level=$L" >/dev/null; sleep 3
  echo "L$L: $(j stage-probe.js)"
done
echo STAGE_DONE