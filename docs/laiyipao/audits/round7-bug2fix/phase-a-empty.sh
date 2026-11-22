#!/bin/bash
# t78 阶段A：user10 未拥有 skill2/equipment11 → 不下发 impact CTA 诚实空态
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
D="D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round7-bug2fix"
S=lyp-r7bug2fix; BASE="http://127.0.0.1:5174/#"
run(){ $AB --session "$S" "$@" 2>&1 | head -1; }
j(){ $AB --session "$S" eval --stdin < "$D/$1" 2>&1 | head -1 | sed 's/^"//;s/"$//'; }
GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c 'select guest_token from users where id=10;' | tr -d ' ')
TOK=$(curl -s -X POST http://127.0.0.1:8080/api/v1/auth/guest -H "Content-Type: application/json" -d "{\"guest_token\":\"$GT\"}" | node -e "let s='';process.stdin.on('data',d=>s+=d).on('end',()=>console.log(JSON.parse(s).access_token))")
sed "s/__TOKEN__/$TOK/" "$D/set-token2.js.tpl" > "$D/set-token2.js"
run set viewport 390 844 >/dev/null 2>&1
echo "=== PHASE A: skills (未拥有候选) ==="
$AB --session "$S" --init-script "$D/probe-init.js" open "$BASE/pages/skills/skills" 2>&1 | head -1
sleep 4
run eval --stdin < "$D/set-token2.js" >/dev/null 2>&1
run reload >/dev/null 2>&1; sleep 9
echo "AUTH_SKILLS: $(j diag-token.js)"
echo "CTA_A_SKILLS: $(j cta-params.js)"
echo "STATE_A_SKILLS: $(j state.js)"
run screenshot "$D/screenshots/honest-empty-skills.png" >/dev/null 2>&1
echo "=== PHASE A: equipment (未拥有候选) ==="
$AB --session "$S" --init-script "$D/probe-init.js" open "$BASE/pages/equipment/equipment" 2>&1 | head -1
sleep 4; run reload >/dev/null 2>&1; sleep 8
echo "CTA_A_EQUIP: $(j cta-params.js)"
echo "STATE_A_EQUIP: $(j state.js)"
run screenshot "$D/screenshots/honest-empty-equipment.png" >/dev/null 2>&1
echo PHASE_A_DONE
