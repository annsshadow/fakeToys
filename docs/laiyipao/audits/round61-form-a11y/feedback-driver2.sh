#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y"; S=lyp-r61-fb2
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
echo "=== signin invalid ==="
AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/signin/signin" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "TRIG=$(AB "$S" eval --stdin < "$D/trig-signin-invalid.js" 2>&1)"; sleep 2
AB "$S" eval --stdin < "$D/r61-feedback-probe.js" > "$D/data/feedback-signin-invalid.json" 2>&1
echo "FB=$(cat "$D/data/feedback-signin-invalid.json")"
echo "=== me upgrade + submit ==="
AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "UPG=$(AB "$S" eval --stdin < "$D/trig-me-upgrade.js" 2>&1)"; sleep 1
AB "$S" eval --stdin < "$D/r61-feedback-probe.js" > "$D/data/ax-me-upgrade-fb.json" 2>&1; echo "MEFB=$(cat "$D/data/ax-me-upgrade-fb.json")"
echo "SUBMIT=$(AB "$S" eval --stdin < "$D/trig-me-submit.js" 2>&1)"; sleep 2
AB "$S" eval --stdin < "$D/r61-feedback-probe.js" > "$D/data/feedback-me.json" 2>&1; echo "ME=$(cat "$D/data/feedback-me.json")"
echo "=== bag preset ==="
AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/bag/bag" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "SAVE=$(AB "$S" eval --stdin < "$D/trig-bag-save.js" 2>&1)"; sleep 1
AB "$S" eval --stdin < "$D/r61-feedback-probe.js" > "$D/data/feedback-bag.json" 2>&1; echo "BAG=$(cat "$D/data/feedback-bag.json")"
AB "$S" close >/dev/null 2>&1 || true; echo FB2_FINISH
