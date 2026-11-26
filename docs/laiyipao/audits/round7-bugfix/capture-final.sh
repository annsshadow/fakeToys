#!/bin/bash
# t75 最终闭环：绝对路径截图 + 原生click 打开弹层与确认
DA="docs/laiyipao/audits/round7-bugfix"
SH="D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round7-bugfix/screenshots"
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7fix; BASE="http://127.0.0.1:5174/#"
run(){ $AB --session "$S" "$@" 2>&1 | head -1; }
j(){ $AB --session "$S" eval --stdin < "$DA/$1" 2>&1 | head -1 | sed 's/^"//;s/"$//'; }
GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c 'select guest_token from users where id=10;' | tr -d ' ')
TOK=$(curl -s -X POST http://127.0.0.1:8080/api/v1/auth/guest -H "Content-Type: application/json" -d "{\"guest_token\":\"$GT\"}" | node -e "let s='';process.stdin.on('data',d=>s+=d).on('end',()=>console.log(JSON.parse(s).access_token))")
sed "s/__TOKEN__/$TOK/" "$DA/set-token2.js.tpl" > "$DA/set-token2.js"
echo "CTAP: $(j cta-params.js)"
for P in skills equipment; do
  echo "=== PAGE $P ==="
  run open "$BASE/pages/$P/$P" >/dev/null; sleep 4
  run eval --stdin < "$DA/set-token2.js" >/dev/null; run reload >/dev/null; sleep 9
  echo "AUTH: $(j diag-token.js)"
  echo "LOADED: $(j state.js)"
  run screenshot "$SH/r7fix-$P-loaded-final.png" >/dev/null 2>&1
  echo "ME_BEFORE: $(j me-build.js)"
  run click ".r7-btn.primary" >/dev/null; sleep 2
  echo "MODAL: $(j modal-state.js)"
  run screenshot "$SH/r7fix-$P-modal-final.png" >/dev/null 2>&1
  echo "CONFIRM_CLICK: $(run click ".r7-compare .r7-btn.primary")"; sleep 4
  echo "STATE_AFTER: $(j state.js)"
  echo "ME_AFTER: $(j me-build.js)"
  run screenshot "$SH/r7fix-$P-applied-final.png" >/dev/null 2>&1
done
echo "=== DB VERIFY ==="
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -c "select skill_id from user_skill_slots where user_id=10 order by slot;" 2>&1 | head -8
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -c "select equipment_id from user_equipment where user_id=10 and equipped order by equipment_id;" 2>&1 | head -8
echo FINAL_DONE
