#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round42-postdeploy
AUTH="$D/data/admin-auth-init.js"
if [ -f "$AUTH" ]; then rm -f "$AUTH"; echo AUTH_DELETED=YES; else echo AUTH_DELETED=ABSENT; fi
echo "AUTH_GLOB=$(git ls-files --others --exclude-standard "$D" | grep -c admin-auth-init.js || true)"
echo "DATA_IGNORED=$(git check-ignore -v "$D/data/config.json" || true)"
echo "AUTH_IGNORED=$(git check-ignore -v "$D/admin-auth-init.js" || true)"
DB=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc "select count(*)||'_'||coalesce(max(id),0) from battle_records" 2>/dev/null | tr -d " ")
echo "DB_CHECK=$DB"
echo FINALIZE_DONE=YES