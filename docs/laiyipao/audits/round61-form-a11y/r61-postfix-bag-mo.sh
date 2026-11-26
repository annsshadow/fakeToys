#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y/data"; P="$ROOT/docs/laiyipao/audits/round61-form-a11y"; S=lyp-r61-pf-mo
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
AB "$S" open --init-script "$D/auth-init.js" "http://127.0.0.1:5201/#/pages/bag/bag" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "TRIG=$(AB "$S" eval --stdin < "$P/pf-mo-install.js" 2>&1)"; sleep 2
AB "$S" eval --stdin < "$P/pf-mo-read.js" > "$D/pf-fb-bag-mo.json" 2>&1
echo "CAP=$(cat "$D/pf-fb-bag-mo.json" | head -c 900)"
AB "$S" close >/dev/null 2>&1 || true
echo PF_MO_FINISH