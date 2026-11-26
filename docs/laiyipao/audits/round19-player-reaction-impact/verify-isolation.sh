#!/bin/bash
D=docs/laiyipao/audits/round19-player-reaction-impact
OUT=$D/data
mkdir -p "$OUT"
for U in 10 11 1; do
  GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT guest_token FROM users WHERE id=$U;" 2>/dev/null | tr -d '\r')
  RESP=$(curl -s --max-time 10 -X POST 'http://127.0.0.1:8080/api/v1/auth/guest' -H 'Content-Type: application/json' -d "{\"guest_token\":\"$GT\"}")
  TOKEN=$(printf %s "$RESP" | node -e 'var s="";process.stdin.on("data",function(d){s+=d});process.stdin.on("end",function(){try{var j=JSON.parse(s);process.stdout.write((j.data&&(j.data.access_token||j.data.token))||j.access_token||"")}catch(e){}})')
  echo "=== UID=$U TOKEN_LEN=$(expr length "$TOKEN") ==="
  curl -s --max-time 15 -X GET 'http://127.0.0.1:8080/api/v1/me/reaction-damage-impact' -H "Authorization: Bearer $TOKEN" -o "$OUT/http-impact-$U.json" -w "HTTP_STATUS=%{http_code}\n"
  node -e 'var fs=require("fs");var j=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));var d=j.data||j;console.log(JSON.stringify({window_days:d.window_days,window_from:d.window_from,window_to:d.window_to,excluded:d.excluded_old_battles,items:d.items}))' "$OUT/http-impact-$U.json"
done
echo "=== SQL_USER10_AUTHORITY_COUNTS ==="
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "SELECT k.key, SUM((k.value)::bigint) AS authority_count FROM battle_records br, jsonb_each(br.reactions_used) k WHERE br.user_id=10 AND br.created_at >= now()-interval '30 days' GROUP BY k.key ORDER BY k.key;"
echo "=== SQL_USER10_BATTLES_EXCLUDED ==="
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "SELECT count(*) AS total, count(*) FILTER (WHERE reaction_timeline IS NULL OR reaction_timeline::text='[]' OR reaction_timeline->>'schema_version' IS DISTINCT FROM 'reaction-timeline-v1') AS excluded FROM battle_records WHERE user_id=10 AND created_at >= now()-interval '30 days';"
