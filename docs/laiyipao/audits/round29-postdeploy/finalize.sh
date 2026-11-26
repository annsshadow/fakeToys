#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
E=docs/laiyipao/audits/round29-postdeploy
rm -f "$E/auth-init.js" "$E/data/samples.jsonl.tmp"
echo "AUTH_DELETED=$(test ! -f "$E/auth-init.js" && echo YES || echo NO)"
echo "=== RESTORE ==="
docker exec -i lyp-qa-pg pg_restore --clean --if-exists -U postgres -d laiyipao < "$E/data/qa-before.dump" 2>&1 | tail -3
echo "RECORDS_AFTER=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select count(*) from battle_records' 2>&1 | tr -d ' ')"
echo "MAX_ID=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select max(id) from battle_records' 2>&1 | tr -d ' ')"
echo "ID478=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select count(*) from battle_records where id=478' 2>&1 | tr -d ' ')"
echo "=== CHECK_IGNORE ==="
git check-ignore "$E/data/qa-before.dump" "$E/auth-init.js"
echo "=== END ==="
