#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round33-wiki-enemy-mechanics
ADMIN_USER="${QA_ADMIN_USER:?需要 QA_ADMIN_USER 环境变量}"
ADMIN_PASS="${QA_ADMIN_PASS:?需要 QA_ADMIN_PASS 环境变量}"
LOGIN=$(QA_ADMIN_USER="$ADMIN_USER" QA_ADMIN_PASS="$ADMIN_PASS" python -c 'import json,os; print(json.dumps({"username":os.environ["QA_ADMIN_USER"],"password":os.environ["QA_ADMIN_PASS"]}))')
TOKEN=$(curl -s -X POST http://127.0.0.1:8080/api/v1/admin/login -H 'Content-Type: application/json' -d "$LOGIN" | python -c "import sys,json; print(json.load(sys.stdin)['access_token'])")
[ -n "$TOKEN" ] || exit 1
printf '%s\n' '(() => {' '  try {' '    localStorage.setItem("lyp_admin_token", "__LYP_ADMIN_TOKEN__");' '  } catch (e) {}' '})();' | sed "s/__LYP_ADMIN_TOKEN__/$TOKEN/" > "$D/data/admin-auth-init.js"
echo TOKEN_LEN=${#TOKEN}
echo MINTED=YES