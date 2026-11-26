#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y/data"; S=lyp-r61-pf-me
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
AB "$S" open --init-script "$D/auth-init.js" "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "TEXT_BEFORE=$(AB "$S" get text body 2>&1 | grep -o "游客[^，。]*" | head -3)"
echo "CLICK_UPG=$(AB "$S" click "button:has-text(\"设置账号密码\")" 2>&1)"
sleep 1
echo "AFTER_UPG=$(AB "$S" snapshot -i 2>&1 | head -30)"
AB "$S" close >/dev/null 2>&1 || true
echo PF_ME_FINISH