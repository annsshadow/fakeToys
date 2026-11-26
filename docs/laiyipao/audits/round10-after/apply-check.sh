#!/bin/bash
AB='C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd'
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round10-after'
echo '=== DB before ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT COALESCE(string_agg(skill_id||':'||slot,','),'none') FROM user_skill_slots WHERE user_id=10;" 2>/dev/null
echo '=== CLICK CONFIRM ==='
"$AB" --session lyp-r10x-skills click '.r7-compare .r7-btn.primary' 2>&1 | head -1
sleep 2
"$AB" --session lyp-r10x-skills eval --stdin < "$D/js-state.js" 2>&1 | head -1
"$AB" --session lyp-r10x-skills screenshot "$D/screenshots/r10-skills-applied.png" 2>&1 | head -1
echo '=== DB after ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT COALESCE(string_agg(skill_id||':'||slot,','),'none') FROM user_skill_slots WHERE user_id=10;" 2>/dev/null