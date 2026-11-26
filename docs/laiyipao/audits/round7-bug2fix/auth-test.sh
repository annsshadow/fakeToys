#!/bin/bash
D="D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round7-bug2fix"
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7b2fresh; BASE="http://127.0.0.1:5174/#"
run(){ $AB --session "$S" "$@" 2>&1 | head -1; }
j(){ $AB --session "$S" eval --stdin < "$D/$1" 2>&1 | head -1 | sed 's/^"//;s/"$//'; }
run set viewport 390 844 >/dev/null 2>&1
echo "=== open skills with document-start auth ==="
$AB --session "$S" --init-script "$D/auth-init.js" open "$BASE/pages/skills/skills" 2>&1 | head -1
sleep 10
echo "AUTH: $(j diag-token.js)"
echo "STATE: $(j state.js)"
echo "CTA: $(j cta-params.js)"
run screenshot "$D/screenshots/auth-test.png" >/dev/null 2>&1
echo AUTH_TEST_DONE
