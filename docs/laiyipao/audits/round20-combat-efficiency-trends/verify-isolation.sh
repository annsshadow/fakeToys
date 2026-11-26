#!/bin/bash
D=docs/laiyipao/audits/round20-combat-efficiency-trends
OUT=$D/data
mkdir -p "$OUT"
for U in 10 11 1; do
  GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT guest_token FROM users WHERE id=$U;" 2>/dev/null | tr -d '\r')
  RESP=$(curl -s --max-time 10 -X POST 'http://127.0.0.1:8080/api/v1/auth/guest' -H 'Content-Type: application/json' -d "{\"guest_token\":\"$GT\"}")
  TOKEN=$(printf %s "$RESP" | node -e 'var s="";process.stdin.on("data",function(d){s+=d});process.stdin.on("end",function(){try{var j=JSON.parse(s);process.stdout.write((j.data&&(j.data.access_token||j.data.token))||j.access_token||"")}catch(e){}})')
  echo "=== UID=$U TOKEN_LEN=${#TOKEN} ==="
  curl -s --max-time 15 -X GET 'http://127.0.0.1:8080/api/v1/me/battle-efficiency?window_days=30' -H "Authorization: Bearer $TOKEN" -o "$OUT/http-efficiency-$U.json" -w "HTTP_STATUS=%{http_code}\n"
  node -e 'var fs=require("fs");var j=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));var d=j.data||j;console.log(JSON.stringify({window_days:d.window_days,window_from:d.window_from,window_to:d.window_to,battles:d.battles,shots:d.shots,hits:d.hits,accuracy_pct:d.accuracy_pct,avg_heat_max:d.avg_heat_max,leaked_per_battle:d.leaked_per_battle,avg_duration_ms:d.avg_duration_ms,buckets:d.buckets,ai:d.ai_insight}))' "$OUT/http-efficiency-$U.json"
done
echo "=== BAD_PARAM_31 ==="
curl -s --max-time 10 -X GET 'http://127.0.0.1:8080/api/v1/me/battle-efficiency?window_days=31' -H "Authorization: Bearer $TOKEN" -w " HTTP_STATUS=%{http_code}\n" | head -c 300
echo ""
echo "=== SQL_USER10_AGG_30D ==="
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT to_char(date_trunc('day',created_at AT TIME ZONE 'UTC'),'YYYY-MM-DD') bucket,count(*) battles,coalesce(sum(shots),0) shots,coalesce(sum(hits),0) hits,coalesce(sum(heat_max),0) heat,coalesce(sum(leaked),0) leaked,coalesce(sum(duration_ms),0) dur FROM battle_records WHERE user_id=10 AND created_at >= date_trunc('day',now() at time zone 'UTC') - interval '30 days' AND created_at < date_trunc('day',now() at time zone 'UTC') + interval '1 day' GROUP BY bucket ORDER BY bucket;"
echo "=== SQL_USER1_11_COUNTS ==="
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT user_id,count(*),coalesce(sum(shots),0),coalesce(sum(hits),0) FROM battle_records WHERE user_id IN (1,11) AND created_at >= date_trunc('day',now() at time zone 'UTC') - interval '30 days' GROUP BY user_id ORDER BY user_id;"
