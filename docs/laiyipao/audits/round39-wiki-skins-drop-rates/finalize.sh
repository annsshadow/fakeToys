#!/bin/bash
# R39 finalize: remove temp auth artifacts and restore the QA database baseline.
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round39-wiki-skins-drop-rates
rm -f "$D/data/admin-auth-init.js" "$D/data/login.json" "$D/admin-auth-init.js"
echo AUTH_DELETED=YES
cp "$D/data/qa-before.dump" /tmp/r39-qa-before.dump
docker exec lyp-qa-pg psql -U postgres -d laiyipao -v ON_ERROR_STOP=1 -c "TRUNCATE TABLE battle_records RESTART IDENTITY CASCADE;" >/dev/null
docker exec -i lyp-qa-pg pg_restore -U postgres -d laiyipao --clean --if-exists < /tmp/r39-qa-before.dump >/dev/null 2>&1
echo "DB_CHECK=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select count(*) from battle_records')_$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select coalesce(max(id),0) from battle_records')"
echo FINALIZE_DONE=YES
