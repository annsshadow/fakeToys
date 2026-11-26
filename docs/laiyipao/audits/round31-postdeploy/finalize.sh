#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round31-postdeploy
rm -f "$D/admin-auth-init.js"
[ ! -f "$D/admin-auth-init.js" ] && echo AUTH_DELETED=YES
docker exec -i lyp-qa-pg pg_restore --clean --if-exists -U postgres -d laiyipao < "$D/data/qa-before.dump"
echo "DB=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select count(*),max(id) from battle_records')"
echo "ID478=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select count(*) from battle_records where id=478')"
git check-ignore -v "$D/data/config.json" "$D/admin-auth-init.js" || echo CHECK_IGNORE_FAIL
