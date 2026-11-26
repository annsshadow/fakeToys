#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y"; S=lyp-r61-pf-fb
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
openp(){ AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/$1/$1" >/dev/null 2>&1; AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2; }
capture(){ AB "$S" eval --stdin < "$D/r61-feedback-probe.js" > "$D/data/pf-fb-$1.json" 2>"$D/data/pf-fb-$1.err"; AB "$S" snapshot --json > "$D/data/pf-fb-ax-$1.json" 2>"$D/data/pf-fb-ax-$1.err"; echo "FB-$1 bytes=$(wc -c < "$D/data/pf-fb-$1.json")"; }
openp login; echo "LOGIN_TRIG=$(AB "$S" eval --stdin < "$D/trig-login.js" 2>&1)"; sleep 1; capture login-err
openp verify; echo "VERIFY_TRIG=$(AB "$S" eval --stdin < "$D/trig-verify.js" 2>&1)"; sleep 1; capture verify-err
openp signin; echo "SIGNIN_TRIG=$(AB "$S" eval --stdin < "$D/trig-signin-invalid.js" 2>&1)"; sleep 2; capture signin-redeem
openp me; echo "UPG=$(AB "$S" eval --stdin < "$D/trig-me-upgrade.js" 2>&1)"; sleep 1; echo "PWD=$(AB "$S" eval --stdin < "$D/trig-me-pwd.js" 2>&1)"; sleep 1; echo "SUBMIT=$(AB "$S" eval --stdin < "$D/trig-me-submit.js" 2>&1)"; sleep 2; capture me-password
openp bag; echo "FILL=$(AB "$S" fill "input.preset-input" "QA预设PF" 2>&1)"; echo "CLICK=$(AB "$S" click "text=保存当前构筑" 2>&1)"; for n in 1 2 3 4 5 6 7 8; do AB "$S" eval --stdin < "$D/r61-feedback-probe.js" > "$D/data/pf-fb-bag-poll$n.json" 2>&1; sleep 0.12; done; capture bag-saving
AB "$S" close >/dev/null 2>&1 || true
echo PF_FB_FINISH