#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
D=docs/laiyipao/audits/round37-wiki-mastery-nodes
U="${QA_ADMIN_USER:?need QA_ADMIN_USER}"
P="${QA_ADMIN_PASS:?need QA_ADMIN_PASS}"
python - "$U" "$P" "$D/data/admin-auth-init.js" <<'PY'
import base64,json,sys
u,p,out=sys.argv[1],sys.argv[2],sys.argv[3]
token=base64.b64encode(json.dumps({"user":u,"pass":p}).encode()).decode()
with open(out,"w") as f: f.write("localStorage.setItem("+json.dumps("lyp_admin_token")+", "+json.dumps(token)+");\n")
print("TOKEN_LEN=%d MINTED=YES" % len(token))
PY
echo AUTH_READY=YES