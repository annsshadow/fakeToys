#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y/data"; P="$ROOT/docs/laiyipao/audits/round61-form-a11y/pf-capture.js"; S=lyp-r61-pf-login
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
AB "$S" open "http://127.0.0.1:5201/#/pages/login/login" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "BEFORE=$(AB "$S" get value "input >> nth=0" 2>&1)"
echo "FILL_PWD=$(AB "$S" fill "input >> nth=1" "admin12345" 2>&1)"
echo "LOGIN_CLICK=$(AB "$S" click "text=登录" 2>&1)"; sleep 3
echo "AFTER_URL=$(AB "$S" get url 2>&1)"
echo "TOKENS=$(AB "$S" eval "(function(){return Object.keys(localStorage).filter(function(k){return k.indexOf("lyp_")===0;}).join(",");})()" 2>&1)"
AB "$S" storage local > "$D/pf-login-storage.txt" 2>&1; echo "STORAGE_BYTES=$(wc -c < "$D/pf-login-storage.txt")"
AB "$S" open "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "ME_BEFORE=$(AB "$S" snapshot -i 2>&1 | head -15)"
echo "ME_CLICK=$(AB "$S" click "text=修改密码" 2>&1)"; sleep 1
echo "ME_AFTER=$(AB "$S" snapshot -i 2>&1 | head -25)"
echo "ME_SUBMIT=$(AB "$S" click "text=确认修改" 2>&1)"; sleep 2
AB "$S" eval --stdin < "$P" > "$D/pf-fb-me-pwd-final.json" 2>&1; echo "ME_PWD_FB=$(cat "$D/pf-fb-me-pwd-final.json" | head -c 400)"
AB "$S" snapshot --json > "$D/pf-fb-ax-me-pwd-final.json" 2>&1
AB "$S" close >/dev/null 2>&1 || true
echo PF_LOGIN_FINISH