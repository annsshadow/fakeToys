#!/usr/bin/env bash
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"
PD="$ROOT/docs/laiyipao/audits/round93-recompute-skeleton-postdeploy"
QA="$ROOT/docs/laiyipao/audits/round93-recompute-skeleton"
D="$PD/data"
S="$PD/screenshots"
AB(){ agent-browser --session "$1" "${@:2}"; }
for V in 414 820 1440; do
  case $V in 414) H=896;; 820) H=1180;; 1440) H=900;; esac
  Sess="r93pd-$V"
  AB "$Sess" close --all >/dev/null 2>&1 || true
  AB "$Sess" open --init-script "$D/r93-init.js" >/dev/null 2>&1
  AB "$Sess" open --init-script "$D/r93-init.js" >/dev/null 2>&1
  AB "$Sess" set viewport $V $H >/dev/null 2>&1
  AB "$Sess" navigate "http://127.0.0.1:5175/laiyipao-admin/#/admin/recomputes" >/dev/null 2>&1
  sleep 1.5
  AB "$Sess" eval --stdin < "$PD/pd-probe.js" > "$D/pd-$V-load.txt" 2>&1
  AB "$Sess" screenshot "$S/pd-r93-$V-load.png" >/dev/null 2>&1
  sleep 6.5
  AB "$Sess" eval --stdin < "$PD/pd-probe.js" > "$D/pd-$V-arrive.txt" 2>&1
  AB "$Sess" screenshot "$S/pd-r93-$V-arrive.png" >/dev/null 2>&1
  AB "$Sess" eval --stdin < "$QA/data/r93-click-refresh.js" >/dev/null 2>&1
  sleep 0.3
  AB "$Sess" eval --stdin < "$QA/data/r93-refresh-probe.js" > "$D/pd-$V-refresh-mid.txt" 2>&1
  sleep 6.5
  AB "$Sess" eval --stdin < "$QA/data/r93-refresh-probe.js" > "$D/pd-$V-refresh-after.txt" 2>&1
  AB "$Sess" close --all >/dev/null 2>&1 || true
  echo "CASE_DONE $V"
done
echo PD_DONE_ALL
