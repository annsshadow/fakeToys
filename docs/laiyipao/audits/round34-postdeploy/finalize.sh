#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round34-postdeploy
DUMP="$D/data/qa-before.dump"
cp "$DUMP" /tmp/r34-qa-before.dump
rm -rf "$D/data" "$D/admin-auth-init.js" admin-auth-init.js
echo AUTH_DELETED=$(test ! -e "$D/data/admin-auth-init.js" -a ! -e admin-auth-init.js && echo YES || echo NO)
docker exec lyp-qa-pg psql -U postgres -d lyp_qa -c 'TRUNCATE TABLE battle_records RESTART IDENTITY CASCADE' >/dev/null 2>&1
docker exec -i lyp-qa-pg pg_restore -U postgres -d lyp_qa --clean --if-exists < /tmp/r34-qa-before.dump >/dev/null 2>&1
DB=$(docker exec lyp-qa-pg psql -U postgres -d lyp_qa -Atc "SELECT count(id),max(id) FROM battle_records")
echo DB=$DB
git check-ignore -v docs/laiyipao/audits/round34-wiki-level-mechanics/data/config.json || echo NO_IGNORE
echo FINALIZE_DONE=YES
