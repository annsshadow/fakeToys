#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round48-r12-growth-cta"
rm -f "$D/data/auth-init.js" "$D/data/login.json" "$D/data/admin-auth-init.js" "$D/data/init-element.js" "$D/data/init-equipment.js" "$D/data/init-reaction.js"
for f in auth-init.js login.json admin-auth-init.js; do if [ -e "$D/data/$f" ]; then echo "AUTH_$f=PRESENT"; else echo "AUTH_$f=ABSENT"; fi; done
git check-ignore -v "$D/data/auth-init.js" "$D/data/login.json" "$D/data/admin-auth-init.js" | sed "s#$ROOT/##"
docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc "SELECT count(*), max(id) FROM battle_records" | tr -d ' '
echo DB_CHECK_DONE
node "$D/secret-scan.js"
echo R48_CLEANUP_DONE=YES