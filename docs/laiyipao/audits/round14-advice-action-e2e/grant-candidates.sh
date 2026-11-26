#!/bin/bash
# QA-only data prep (local loopback): grant then restore; never prod
echo '=== BEFORE SKILLS ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "select skill_id from user_skills where user_id=10 order by skill_id" 2>/dev/null|tr -d '\r'|paste -sd, -
echo '=== BEFORE SLOTS ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "select slot||':'||skill_id from user_skill_slots where user_id=10 order by slot" 2>/dev/null|tr -d '\r'|paste -sd, -
echo '=== GRANT fire3 ice6 lightning8 corrosion18 ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c "insert into user_skills(user_id,skill_id,level) values (10,3,1),(10,6,1),(10,8,1),(10,18,1) on conflict do nothing" 2>&1|tr -d '\r'
echo '=== AFTER SKILLS ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "select skill_id from user_skills where user_id=10 order by skill_id" 2>/dev/null|tr -d '\r'|paste -sd, -