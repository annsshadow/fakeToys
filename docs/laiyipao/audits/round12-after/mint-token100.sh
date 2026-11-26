#!/bin/bash
D='D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248/docs/laiyipao/audits/round12-after'
GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c 'select guest_token from users where id=100' 2>/dev/null | tr -d '\r')
RESP=$(curl -s --max-time 10 -X POST http://127.0.0.1:8080/api/v1/auth/guest -H 'Content-Type: application/json' -d "{\"guest_token\":\"$GT\"}" 2>/dev/null)
TOKEN=$(printf %s "$RESP" | node -e 'var s="";process.stdin.on("data",function(d){s+=d});process.stdin.on("end",function(){try{var j=JSON.parse(s);process.stdout.write((j.data&&(j.data.access_token||j.data.token))||j.access_token||"")}catch(e){}})')
echo TOKEN_LEN=${#TOKEN}
[ ${#TOKEN} -lt 100 ] && { echo MINT_FAIL; exit 1; }
sed "s|__LYP_TOKEN__|$TOKEN|g" "$D/auth-init.tpl.js" > "$D/auth100.js"
echo READY