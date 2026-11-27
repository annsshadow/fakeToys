#!/bin/bash
set -u
ROOT="$(git rev-parse --show-toplevel)"
D="$ROOT/docs/laiyipao/audits/round88-trend-legend"
AUTH="$D/data/auth-init.js"
INIT="$D/data/r88-init.js"
PROBE="$D/data/r88-probe.js"
AB(){ agent-browser --session "$1" "${@:2}"; }
for V in 390 768 1280; do
  case $V in 390) H=844;; 768) H=1024;; 1280) H=800;; esac
  S="r88-$V"
  AB "$S" close --all >/dev/null 2>&1 || true
  AB "$S" open --init-script "$AUTH" --init-script "$INIT" >/dev/null 2>&1
  AB "$S" set viewport $V $H >/dev/null 2>&1
  AB "$S" navigate "http://127.0.0.1:5201/#/pages/battles/battles" >/dev/null 2>&1
  sleep 6
  AB "$S" eval --stdin < "$PROBE" > "$D/data/probe-$V.txt" 2>&1
  AB "$S" screenshot "$D/screenshots/r88-trend-$V.png" >/dev/null 2>&1
  echo "DONE $V"
done
echo WALK88_ALL_DONE