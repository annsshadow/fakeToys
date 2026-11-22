#!/bin/bash
DA="docs/laiyipao/audits/round7-bugfix"
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
S=lyp-r7fix; BASE="http://127.0.0.1:5174/#"
run(){ $AB --session "$S" "$@" 2>&1 | head -1; }
j(){ $AB --session "$S" eval --stdin < "$DA/$1" 2>&1 | head -1 | sed 's/^"//;s/"$//'; }
GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c 'select guest_token from users where id=10;' | tr -d ' ')
TOK=$(curl -s -X POST http://127.0.0.1:8080/api/v1/auth/guest -H "Content-Type: application/json" -d "{\"guest_token\":\"$GT\"}" | node -e "let s='';process.stdin.on('data',d=>s+=d).on('end',()=>console.log(JSON.parse(s).access_token))")
sed "s/__TOKEN__/$TOK/" "$DA/set-token2.js.tpl" > "$DA/set-token2.js"
run open "$BASE/pages/skills/skills" >/dev/null; sleep 4
echo "INJECT: $(run eval --stdin < "$DA/set-token2.js")"
echo "BEFORE_RELOAD: $(j state.js)"
run reload >/dev/null; sleep 9
echo "AUTH: $(j diag-token.js)"
echo "STATE: $(j state.js)"
echo "MODAL_CTA: $(j open-modal.js)"
