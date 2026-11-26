#!/bin/bash
echo '=== RESTORE DB ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "UPDATE user_skill_slots SET skill_id=4 WHERE user_id=10 AND slot=1 AND skill_id=10;" 2>/dev/null
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT COALESCE(string_agg(skill_id||':'||slot,','),'none') FROM user_skill_slots WHERE user_id=10;" 2>/dev/null
echo '=== SANITIZE TOKEN FILE ==='
sed "s/'lyp_token','[^']*'/'lyp_token','<YOUR_TEST_TOKEN>'/g" docs/laiyipao/audits/round10-after/auth-init.js > docs/laiyipao/audits/round10-after/auth-init.js.san
mv docs/laiyipao/audits/round10-after/auth-init.js.san docs/laiyipao/audits/round10-after/auth-init.js
echo '=== SCAN JWT/TOKEN in dir ==='
C=$(grep -rlE 'eyJ[A-Za-z0-9_-]{20,}|[A-Za-z0-9_-]{100,}\.[A-Za-z0-9_-]{20,}' docs/laiyipao/audits/round10-after/ 2>/dev/null | wc -l)
echo "JWT_HIT_FILES=$C"