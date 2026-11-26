#!/bin/bash
D=docs/laiyipao/audits/round14-advice-action-e2e
mkdir -p "$D/data"
GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT guest_token FROM users WHERE id=10;" 2>/dev/null | tr -d '\r')
RESP=$(curl -s --max-time 10 -X POST 'http://127.0.0.1:8080/api/v1/auth/guest' -H 'Content-Type: application/json' -d "{\"guest_token\":\"$GT\"}")
TOKEN=$(printf %s "$RESP" | node -e 'var s="";process.stdin.on("data",function(d){s+=d});process.stdin.on("end",function(){try{var j=JSON.parse(s);process.stdout.write((j.data&&(j.data.access_token||j.data.token))||j.access_token||"")}catch(e){}})')
echo "TOKEN_LEN=${#TOKEN}"
printf %s "$TOKEN" | node -e 'var s="";process.stdin.on("data",function(d){s+=d});process.stdin.on("end",function(){try{var p=s.split(".")[1];var j=JSON.parse(Buffer.from(p,"base64").toString());process.stdout.write("PAYLOAD="+JSON.stringify(j))}catch(e){process.stdout.write("ERR")}})'; echo ""
for el in fire ice lightning corrosion kinetic; do
  printf "EL=%s " "$el"
  curl -s --max-time 10 -H "Authorization: Bearer $TOKEN" "http://127.0.0.1:8080/api/v1/me/advice/action?element=$el&source=battle" | node -e 'var s="";process.stdin.on("data",function(d){s+=d});process.stdin.on("end",function(){try{var j=JSON.parse(s);var e=j.empty_reason||{};process.stdout.write("code="+(e.code||"READY")+" kind="+(j.kind||"-")+" cand="+(j.candidate_id||0)+" repl="+(j.replaced_id||0)+" cn="+(j.candidate_name||"-")+" rn="+(j.replaced_name||"-"))}catch(err){process.stdout.write("ERR")}})'; echo ""
done