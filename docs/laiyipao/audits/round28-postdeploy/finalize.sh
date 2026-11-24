#!/bin/bash
set -u
cd D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248
D=docs/laiyipao/audits/round28-postdeploy
rm -f "$D/auth-init.js"
echo "AUTH_DELETED=$(test ! -f "$D/auth-init.js" && echo YES || echo NO)"
echo "TPL_EXISTS=$(test -f "$D/auth-init.tpl.js" && echo YES || echo NO)"
echo '=== RESTORE ==='
docker exec -i lyp-qa-pg pg_restore --clean --if-exists -U postgres -d laiyipao < "$D/data/qa-before.dump" 2>&1 | tail -3
echo "RESTORE_RC=$?"
echo "RECORDS_AFTER=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select count(*) from battle_records' 2>&1 | tr -d ' ')"
echo "MAX_ID=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select max(id) from battle_records' 2>&1 | tr -d ' ')"
echo "ID478=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select count(*) from battle_records where id=478' 2>&1 | tr -d ' ')"
echo '=== SHA ==='
sha256sum "$D/screenshots/"*.png 2>/dev/null
echo '=== CHECK_IGNORE ==='
git check-ignore -v "$D/data/qa-before.dump" "$D/auth-init.js" 2>&1
echo '=== TRUTH SCAN === 由提交门禁以团队既定真值严格正则统一执行，模式本身不入库以避免自指误报 ==='
