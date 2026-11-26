#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y/data"; P="$ROOT/docs/laiyipao/audits/round61-form-a11y"; S=lyp-r61-me-upgrade2
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
AB "$S" open --init-script "$D/auth-init.js" "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "OPEN=$(AB "$S" snapshot -i 2>&1 | head -12)"
echo "CLICK=$(AB "$S" click "uni-button.btn" 2>&1)"; sleep 1
echo "AFTER=$(AB "$S" snapshot -i 2>&1 | head -15)"
echo "F1=$(AB "$S" fill "input >> nth=0" "" 2>&1)"; echo "F2=$(AB "$S" fill "input >> nth=1" "" 2>&1)"
echo "SUB=$(AB "$S" click "uni-view.btn.btn-primary" 2>&1)"; sleep 2
AB "$S" eval --stdin < "$P/pf-capture.js" > "$D/pf-fb-me-upgrade.json" 2>&1
echo "FB=$(cat "$D/pf-fb-me-upgrade.json" | head -c 800)"
AB "$S" snapshot --json > "$D/pf-fb-ax-me-upgrade.json" 2>&1
AB "$S" close >/dev/null 2>&1 || true
echo PF_ME_UPGRADE_FINISH