#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round36-postdeploy
mkdir -p "$D/data"
docker exec lyp-qa-pg pg_dump -U postgres -Fc laiyipao > "$D/data/qa-before.dump"
echo "DUMP_BYTES=$(wc -c < "$D/data/qa-before.dump")"
echo "DB_BEFORE=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select count(*) from battle_records' | tr -d ' ')|$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select coalesce(max(id),0) from battle_records' | tr -d ' ')"
