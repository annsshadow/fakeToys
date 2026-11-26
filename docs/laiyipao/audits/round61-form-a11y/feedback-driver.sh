#!/usr/bin/env bash
set -u
ROOT=$(git rev-parse --show-toplevel); D="$ROOT/docs/laiyipao/audits/round61-form-a11y"; S=lyp-r61-fb
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
for P in login verify signin; do
 AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/$P/$P" >/dev/null 2>&1
 AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
 echo "TRIG-$P: $(AB "$S" eval --stdin < "$D/trig-$P.js" 2>&1)"
 sleep 1
 AB "$S" eval --stdin < "$D/r61-feedback-probe.js" > "$D/data/feedback-$P.json" 2>"$D/data/feedback-$P.err"
 AB "$S" snapshot > "$D/data/feedback-$P.snapshot.txt" 2>&1
 echo "FB-$P bytes=$(wc -c < "$D/data/feedback-$P.json") err=$(wc -c < "$D/data/feedback-$P.err")"
done
AB "$S" close >/dev/null 2>&1 || true; echo FB_FINISH
