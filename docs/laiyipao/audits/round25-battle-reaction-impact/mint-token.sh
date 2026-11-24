#!/bin/bash
D=docs/laiyipao/audits/round25-battle-reaction-impact
Q_UID=${1:-10}
GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT guest_token FROM users WHERE id=$Q_UID;" 2>/dev/null | tr -d '\r')
RESP=$(curl -s --max-time 10 -X POST 'http://127.0.0.1:8080/api/v1/auth/guest' -H 'Content-Type: application/json' -d "{\"guest_token\":\"$GT\"}")
TOKEN=$(printf %s "$RESP" | node -e 'var s="";process.stdin.on("data",function(d){s+=d});process.stdin.on("end",function(){try{var j=JSON.parse(s);process.stdout.write((j.data&&(j.data.access_token||j.data.token))||j.access_token||"")}catch(e){}})')
TL=$(printf %s "$TOKEN" | wc -c)
echo "TOKEN_LEN=$TL UID=$Q_UID"
if [ "$TL" -lt 100 ]; then echo MINT_FAILED; exit 1; fi
sed "s|__LYP_TOKEN__|$TOKEN|g" "$D/auth-init.tpl.js" > "$D/auth-init.js"
echo AUTH_INIT_READY