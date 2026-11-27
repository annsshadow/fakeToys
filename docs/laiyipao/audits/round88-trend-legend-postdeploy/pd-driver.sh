#!/usr/bin/env bash
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"
PD="$ROOT/docs/laiyipao/audits/round88-trend-legend-postdeploy"
QA="$ROOT/docs/laiyipao/audits/round88-trend-legend"
D="$PD/data"
S="$PD/screenshots"
QD="$QA/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
for V in 414 820 1440; do
  case $V in 414) H=896;; 820) H=1180;; 1440) H=900;; esac
  Sess="r88pd-$V"
  AB "$Sess" close --all >/dev/null 2>&1 || true
  AB "$Sess" open --init-script "$D/auth-init.js" --init-script "$QD/r88-init.js" >/dev/null 2>&1
  AB "$Sess" set viewport $V $H >/dev/null 2>&1
  AB "$Sess" navigate "http://127.0.0.1:5201/#/pages/battles/battles" >/dev/null 2>&1
  sleep 6
  AB "$Sess" eval --stdin < "$PD/pd-probe.js" > "$D/pd-probe-$V.txt" 2>&1
  AB "$Sess" screenshot "$S/pd-r88-trend-$V.png" >/dev/null 2>&1
  AB "$Sess" close --all >/dev/null 2>&1 || true
  echo "CASE_DONE $V"
done
echo PD_DONE_ALL
