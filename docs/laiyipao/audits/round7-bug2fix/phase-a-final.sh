#!/bin/bash
D="D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round7-bug2fix"
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7b2empty; BASE="http://127.0.0.1:5174/#"
run(){ $AB --session "$S" "$@" 2>&1 | head -1; }
j(){ $AB --session "$S" eval --stdin < "$D/$1" 2>&1 | head -1 | sed 's/^"//;s/"$//'; }
run set viewport 390 844 >/dev/null 2>&1
echo "=== PHASE A: equipment 未拥有候选11 → 诚实空态 ==="
$AB --session "$S" --init-script "$D/auth-init.js" open "$BASE/pages/equipment/equipment" 2>&1 | head -1
sleep 10
echo "A_AUTH: $(j diag-token.js)"
echo "A_CTA: $(j cta-params.js)"
echo "A_STATE: $(j state.js)"
run screenshot "$D/screenshots/honest-empty-equipment.png" >/dev/null 2>&1
echo "=== PHASE A: skills 未拥有更优候选场景(用新号33) ==="
GT33=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "select guest_token from users where id=33;" | tr -d " ")
TOK33=$(curl -s -X POST http://127.0.0.1:8080/api/v1/auth/guest -H "Content-Type: application/json" -d "{\"guest_token\":\"$GT33\"}" | node -e "let s='';process.stdin.on('data',d=>s+=d).on('end',()=>console.log(JSON.parse(s).access_token))")
sed "s/__TOKEN__/$TOK33/" "$D/auth-init.js.tpl" > "$D/auth-init-33.js"
S=lyp-r7b2empty33
$AB --session "$S" --init-script "$D/auth-init-33.js" open "$BASE/pages/skills/skills" 2>&1 | head -1
sleep 10
echo "A33_AUTH: $(j diag-token.js)"
echo "A33_CTA: $(j cta-params.js)"
echo "A33_STATE: $(j state.js)"
run screenshot "$D/screenshots/honest-empty-skills-newuser.png" >/dev/null 2>&1
echo PHASE_A_DONE
