#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round34-postdeploy
LOGIN='{"username":"admin","password":"admin12345"}'
TOKEN=$(curl -s --max-time 10 -X POST http://127.0.0.1:8080/api/v1/admin/login -H 'Content-Type: application/json' -d "$LOGIN" | python -c "import sys,json; print(json.load(sys.stdin)['access_token'])")
[ -n "$TOKEN" ] || exit 1
printf '%s\n' '(() => {' '  try {' '    localStorage.setItem("lyp_admin_token", "__LYP_ADMIN_TOKEN__");' '  } catch (e) {}' '})();' | sed "s/__LYP_ADMIN_TOKEN__/$TOKEN/" > "$D/data/admin-auth-init.js"
echo TOKEN_LEN=$(printf %s "$TOKEN" | wc -c)
echo MINTED=YES
