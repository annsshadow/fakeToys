#!/bin/bash
D=docs/laiyipao/audits/round21-player-level-coach
API=http://127.0.0.1:8080/api/v1
extract() { node -e 'var s="";process.stdin.on("data",function(d){s+=d});process.stdin.on("end",function(){try{var j=JSON.parse(s);process.stdout.write((j.data&&(j.data.access_token||j.data.token))||j.access_token||"")}catch(e){}})'; }
for Q_UID in 10 100 11; do
  GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT guest_token FROM users WHERE id=$Q_UID;" 2>/dev/null | tr -d '\r')
  RESP=$(curl -s --max-time 10 -X POST "$API/auth/guest" -H 'Content-Type: application/json' -d '{"guest_token":"'"$GT"'"}')
  TOKEN=$(printf %s "$RESP" | extract)
  ST=$(curl -s --max-time 10 -o "$D/data/http-coach-$Q_UID.json" -w '%{http_code}' "$API/me/level-coach?window_days=30" -H "Authorization: Bearer $TOKEN")
  echo "UID=$Q_UID TOKEN_LEN=${#TOKEN} HTTP=$ST"
done
GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT guest_token FROM users WHERE id=10;" | tr -d '\r')
RESP=$(curl -s -X POST "$API/auth/guest" -H 'Content-Type: application/json' -d '{"guest_token":"'"$GT"'"}')
TOKEN=$(printf %s "$RESP" | extract)
echo "WINDOW7=$(curl -s -o /dev/null -w '%{http_code}' "$API/me/level-coach?window_days=7" -H "Authorization: Bearer $TOKEN")"
echo "DUP=$(curl -s -o /dev/null -w '%{http_code}' "$API/me/level-coach?window_days=30&window_days=30" -H "Authorization: Bearer $TOKEN")"
echo "NOAUTH=$(curl -s -o /dev/null -w '%{http_code}' "$API/me/level-coach?window_days=30")"
