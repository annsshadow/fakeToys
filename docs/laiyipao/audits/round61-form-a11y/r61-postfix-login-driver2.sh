#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y/data"; P="$ROOT/docs/laiyipao/audits/round61-form-a11y/pf-capture.js"; S=lyp-r61-pf-login2
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
AB "$S" open "http://127.0.0.1:5201/#/pages/login/login" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "USER=$(AB "$S" get value "input >> nth=0" 2>&1)"
echo "PWD=$(AB "$S" fill "input >> nth=1" "admin12345" 2>&1)"
echo "SNAP=$(AB "$S" snapshot 2>&1 | grep -E "textbox|登录" | head -15)"
echo "CLICK=$(AB "$S" click "uni-view.btn.btn-primary" 2>&1)"; sleep 3
echo "URL=$(AB "$S" get url 2>&1)"
AB "$S" storage local > "$D/pf-login-storage2.txt" 2>&1; echo "STORAGE=$(head -c 200 "$D/pf-login-storage2.txt")"
AB "$S" open "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "ME=$(AB "$S" snapshot -i 2>&1 | head -20)"
echo "ME_CLICK=$(AB "$S" click "uni-button.btn" 2>&1)"; sleep 1
echo "ME_AFTER=$(AB "$S" snapshot -i 2>&1 | head -25)"
echo "ME_SUBMIT=$(AB "$S" click "uni-view.btn.btn-primary" 2>&1)"; sleep 2
AB "$S" eval --stdin < "$P" > "$D/pf-fb-me-pwd-final2.json" 2>&1; echo "FB=$(cat "$D/pf-fb-me-pwd-final2.json" | head -c 400)"
AB "$S" snapshot --json > "$D/pf-fb-ax-me-pwd-final2.json" 2>&1
AB "$S" close >/dev/null 2>&1 || true
echo PF_LOGIN2_FINISH