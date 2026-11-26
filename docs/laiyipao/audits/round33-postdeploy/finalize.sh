#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round33-postdeploy
rm -f "$D/data/admin-auth-init.js" "$D/admin-auth-init.js"; [ -f "$D/data/admin-auth-init.js" ] || echo AUTH_DELETED=YES
docker exec lyp-qa-pg psql -U postgres -d laiyipao -c 'TRUNCATE battle_records RESTART IDENTITY CASCADE' >/dev/null 2>&1
docker exec -i lyp-qa-pg pg_restore --clean --if-exists -U postgres -d laiyipao < "$D/data/qa-before.dump" >/dev/null 2>&1
echo DB=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -Atc 'select count(*),coalesce(max(id),0) from battle_records')
git check-ignore -v "$D/data/admin-auth-init.js" "$D/data/config.json" "$D/admin-auth-init.js"
echo FINALIZE_DONE=YES
