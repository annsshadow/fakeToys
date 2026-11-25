#!/bin/bash
# R39 QA: mint a temporary admin token. Credentials come ONLY from env vars
# QA_ADMIN_USER / QA_ADMIN_PASS - no literal secrets in this repo.
# The generated data/admin-auth-init.js and data/login.json are deleted by finalize.sh.
set -e
cd "$(dirname "$0")/../../../.." || exit 1
D=docs/laiyipao/audits/round39-wiki-skins-drop-rates
mkdir -p "$D/data" "$D/screenshots"
if [ -z "$QA_ADMIN_USER" ] || [ -z "$QA_ADMIN_PASS" ]; then
  echo "ERR_MISSING_ENV: QA_ADMIN_USER and QA_ADMIN_PASS must be exported"
  exit 1
fi
curl -sS -X POST http://127.0.0.1:8080/api/v1/admin/login -H "Content-Type: application/json" \
  --data "{\"username\":\"$QA_ADMIN_USER\",\"password\":\"$QA_ADMIN_PASS\"}" --max-time 20 \
  -o "$D/data/login.json" || { echo ERR_LOGIN_HTTP; exit 1; }
python - "$D/data/login.json" "$D/data/admin-auth-init.js" <<'PYEOF'
import json, sys
login_path, out_path = sys.argv[1], sys.argv[2]
data = json.load(open(login_path, encoding="utf-8"))
tok = data.get("access_token") or data.get("token")
if not tok and isinstance(data.get("data"), dict):
    tok = data["data"].get("access_token") or data["data"].get("token")
if not tok:
    raise SystemExit("NO_TOKEN_IN_LOGIN_RESPONSE")
open(out_path, "w", encoding="utf-8").write(
    "localStorage.setItem(" + json.dumps("lyp_admin_token") + ", " + json.dumps(tok) + ");\n"
)
print("TOKEN_MINTED len=%d" % len(tok))
PYEOF
echo MINT_DONE=YES
