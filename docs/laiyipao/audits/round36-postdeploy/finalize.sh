#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round36-postdeploy
cp "$D/data/qa-before.dump" /tmp/r36-qa-before.dump
rm -rf "$D/data" "$D/admin-auth-init.js" admin-auth-init.js
echo AUTH_DELETED=YES
docker exec lyp-qa-pg psql -U postgres -d laiyipao -v ON_ERROR_STOP=1 -c "TRUNCATE TABLE battle_records RESTART IDENTITY CASCADE;" >/dev/null
docker exec -i lyp-qa-pg pg_restore -U postgres -d laiyipao --clean --if-exists < /tmp/r36-qa-before.dump >/dev/null 2>&1 || true
echo "DB=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select count(*) from battle_records' | tr -d ' ')|$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select coalesce(max(id),0) from battle_records' | tr -d ' ')"
git check-ignore -v "$D/data/config.json" 2>&1 || true
echo FINALIZE_DONE=YES
