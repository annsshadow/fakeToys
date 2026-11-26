#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y/data"; P="$ROOT/docs/laiyipao/audits/round61-form-a11y"; S=lyp-r61-me-signout
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
AB "$S" open --init-script "$D/auth-init.js" "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "STEP1=$(AB "$S" click "uni-view.generic >> nth=1" 2>&1)"; sleep 1
echo "SNAP=$(AB "$S" snapshot -i 2>&1 | head -20)"
echo "FILL1=$(AB "$S" fill "input >> nth=0" "" 2>&1)"
echo "FILL2=$(AB "$S" fill "input >> nth=1" "" 2>&1)"
echo "SUBMIT=$(AB "$S" click "uni-view.btn.btn-primary" 2>&1)"; sleep 2
AB "$S" eval --stdin < "$P/pf-capture.js" > "$D/pf-fb-me-pwd-signout.json" 2>&1
echo "FB=$(cat "$D/pf-fb-me-pwd-signout.json" | head -c 600)"
echo "SIGNOUT=$(AB "$S" click "uni-view.generic >> nth=2" 2>&1)"; sleep 2
echo "URL=$(AB "$S" get url 2>&1)"
AB "$S" close >/dev/null 2>&1 || true
echo PF_ME_SIGNOUT_FINISH