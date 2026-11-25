#!/bin/bash
# R41 部署后 finalize：删临时 auth 真值 + 只读核对 QA 库 104/477
cd "$(dirname "$0")/../../../.." || exit 1
D=docs/laiyipao/audits/round41-postdeploy
rm -f "$D/data/admin-auth-init.js" "$D/admin-auth-init.js" "$D/data/login.json"
echo AUTH_DELETED=YES
C=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select count(*) from battle_records' 2>/dev/null | tr -d '[:space:]')
M=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -tAc 'select coalesce(max(id),0) from battle_records' 2>/dev/null | tr -d '[:space:]')
echo "DB_CHECK=${C}_${M}"
echo FINALIZE_DONE=YES