#!/bin/bash
DA="docs/laiyipao/audits/round7-bugfix"
AB="C:/Users/user/AppData/Local/Microsoft/WinGet/Packages/OpenJS.NodeJS.LTS_Microsoft.Winget.Source_8wekyb3d8bbwe/node-v24.20.0-win-x64/agent-browser.cmd"
GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c 'select guest_token from users where id=10;' | tr -d ' ')
TOK=$(curl -s -X POST http://127.0.0.1:8080/api/v1/auth/guest -H "Content-Type: application/json" -d "{\"guest_token\":\"$GT\"}" | node -e "let s='';process.stdin.on('data',d=>s+=d).on('end',()=>console.log(JSON.parse(s).access_token))")
echo TOKLEN=${#TOK}
sed "s/__TOKEN__/$TOK/" "$DA/set-token.js.tpl" > "$DA/set-token.js"
echo TPLLEN=$(wc -c < "$DA/set-token.js")
$AB --session lyp-r7fix eval --stdin < "$DA/set-token.js" 2>&1 | head -2
$AB --session lyp-r7fix open "http://127.0.0.1:5174/#/pages/skills/skills" >/dev/null 2>&1; sleep 6
$AB --session lyp-r7fix eval --stdin < "$DA/diag-token.js" 2>&1 | head -2
