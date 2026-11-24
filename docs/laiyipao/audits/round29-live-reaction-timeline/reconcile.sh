#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round29-live-reaction-timeline
BID=$(node -e 'var fs=require("fs");var o=JSON.parse(fs.readFileSync(process.argv[1],"utf8").trim());if(typeof o==="string")o=JSON.parse(o);var f=(o.settle||[]).slice(-1)[0]||{};process.stdout.write(String(f.battle_id||0))' "$D/data/final.json")
echo "BATTLE_ID=$BID"
GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT guest_token FROM users WHERE id=10;" 2>/dev/null | tr -d '\r')
RESP=$(curl -s --max-time 10 -X POST 'http://127.0.0.1:8080/api/v1/auth/guest' -H 'Content-Type: application/json' -d "{\"guest_token\":\"$GT\"}")
TOKEN=$(printf %s "$RESP" | node -e 'var s="";process.stdin.on("data",function(d){s+=d});process.stdin.on("end",function(){try{var j=JSON.parse(s);process.stdout.write((j.data&&(j.data.access_token||j.data.token))||j.access_token||"")}catch(e){}})')
echo "DETAIL_TOKEN_LEN=$(printf %s "$TOKEN" | wc -c)"
curl -s --max-time 15 "http://127.0.0.1:8080/api/v1/me/battles/$BID" -H "Authorization: Bearer $TOKEN" > "$D/data/detail.json"
node -e 'var fs=require("fs");var j=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));var d=j.data||j;console.log("DETAIL_KEYS="+Object.keys(d).join(","));var t=d.reaction_timeline||{};var ev=t.events||[];console.log("TIMELINE_TOTAL="+ev.length+" TRUNCATED="+t.truncated+" REPORTED_TOTAL="+t.total);console.log("LAST3="+JSON.stringify(ev.slice(-3)))' "$D/data/detail.json"
