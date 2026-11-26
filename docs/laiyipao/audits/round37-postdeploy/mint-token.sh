#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round37-postdeploy
mkdir -p "$D/data" "$D/screenshots"
curl -sS -X POST http://127.0.0.1:8080/api/v1/admin/login -H "Content-Type: application/json" --data '{"username":"admin","password":"admin12345"}' --max-time 20 -o "$D/data/login.json" || exit 1
PY=python
"$PY" - "$D/data/login.json" "$D/data/admin-auth-init.js" <<'PYEOF'
import json,sys
login,out=sys.argv[1],sys.argv[2]
data=json.load(open(login,encoding='utf-8'))
tok=data.get('access_token') or data.get('token') or data.get('data',{}).get('access_token')
if not tok: raise SystemExit('NO_TOKEN_IN_LOGIN')
open(out,'w',encoding='utf-8').write('localStorage.setItem('+json.dumps('lyp_admin_token')+', '+json.dumps(tok)+');\n')
print('TOKEN_LEN=%d MINTED=YES' % len(tok))
PYEOF
curl -sS http://127.0.0.1:8080/api/v1/config --max-time 30 -o "$D/data/config.json" || exit 1
echo "CONFIG_BYTES=$(wc -c < "$D/data/config.json")"
