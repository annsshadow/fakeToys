#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round61-form-a11y-postdeploy"
S=lyp-r61-pd
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$S" open >/dev/null 2>&1
PAGES="announcements login verify signin me bag equipment enemies gems reactions skills skins"
for W in 390 768 1280; do
  H=844; [ "$W" = 768 ] && H=1024; [ "$W" = 1280 ] && H=800
  for P in $PAGES; do
    N="$W-$P"
    AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/$P/$P" >/dev/null 2>&1
    AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
    sleep 2
    AB "$S" eval --stdin < "$D/pd-probe.js" > "$D/data/pd-empty-$N.json" 2>"$D/data/pd-empty-$N.err"
    AB "$S" eval --stdin < "$D/pd-fill.js" > "$D/data/pd-fill-$N.json" 2>"$D/data/pd-fill-$N.err"
    sleep 1
    AB "$S" eval --stdin < "$D/pd-probe.js" > "$D/data/pd-filled-$N.json" 2>"$D/data/pd-filled-$N.err"
    echo "PROBE $N e=$(wc -c < "$D/data/pd-empty-$N.json") f=$(wc -c < "$D/data/pd-filled-$N.json")"
  done
done
for W in 414 1440; do
  H=896; [ "$W" = 1440 ] && H=900
  for P in announcements login verify me bag reactions; do
    AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/$P/$P" >/dev/null 2>&1
    AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
    sleep 2
    AB "$S" eval --stdin < "$D/pd-fill.js" >/dev/null 2>&1 || true
    sleep 1
    AB "$S" screenshot "$D/screenshots/pd-$W-$P.png" >/dev/null 2>&1
    echo "SHOT $W-$P rc=$?"
  done
done
AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1
AB "$S" set viewport 768 1024 >/dev/null 2>&1; sleep 2
echo "UPG=$(AB "$S" eval --stdin < "$ROOT/docs/laiyipao/audits/round61-form-a11y/trig-me-upgrade.js" 2>&1)"; sleep 1
echo "PWD=$(AB "$S" eval --stdin < "$ROOT/docs/laiyipao/audits/round61-form-a11y/trig-me-pwd.js" 2>&1)"; sleep 1
AB "$S" eval --stdin < "$D/pd-probe.js" > "$D/data/pd-empty-768-me-expanded.json" 2>"$D/data/pd-empty-768-me-expanded.err"
AB "$S" eval --stdin < "$D/pd-fill.js" > "$D/data/pd-fill-768-me-expanded.json" 2>&1
sleep 1
AB "$S" eval --stdin < "$D/pd-probe.js" > "$D/data/pd-filled-768-me-expanded.json" 2>"$D/data/pd-filled-768-me-expanded.err"
echo "PROBE me-expanded e=$(wc -c < "$D/data/pd-empty-768-me-expanded.json") f=$(wc -c < "$D/data/pd-filled-768-me-expanded.json")"
AB "$S" close >/dev/null 2>&1 || true
echo PD_FINISH
