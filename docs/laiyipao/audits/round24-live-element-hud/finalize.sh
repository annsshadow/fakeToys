#!/bin/bash
D=docs/laiyipao/audits/round24-live-element-hud
echo '=== GITIGNORE R24 ==='
grep -n 'round24-live-element-hud' .gitignore || echo NO_GITIGNORE
echo '=== DUMP EXISTS ==='
ls -l "$D/data/qa-before.dump"
echo '=== DELETE TOKEN FILE ==='
rm -f "$D/auth-init.js"
if [ -f "$D/auth-init.js" ]; then echo DELETE_FAILED; else echo AUTH_INIT_DELETED; fi
echo "TPL_STILL_THERE=$(test -f $D/auth-init.tpl.js && echo yes || echo no)"
echo '=== QA RESTORE ==='
docker exec -i lyp-qa-pg pg_restore --clean --if-exists -U postgres -d laiyipao < "$D/data/qa-before.dump" 2>&1 | tail -5
echo "RESTORE_EXIT=${PIPESTATUS[0]}"
echo '=== BATTLE COUNT ==='
docker exec lyp-qa-pg psql -U postgres -d laiyipao -t -A -c "SELECT count(*) FROM battle_records;"
echo '=== DONE ==='