#!/usr/bin/env bash
set -uo pipefail
AB(){ agent-browser --session "$1" "${@:2}"; }
S=lyp-r55-pd3
O=docs/laiyipao/audits/round55-postdeploy
INIT="$O/data/auth-init.js"
AB "$S" open >/dev/null 2>&1
for W in 414 820 1440; do
  case $W in 414) H=896;; 820) H=1180;; 1440) H=900;; esac
  for pg in reactions bag; do
    AB "$S" open --init-script "$INIT" "http://127.0.0.1:5201/#/pages/$pg/$pg" >/dev/null 2>&1
    AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
    sleep 2
    AB "$S" eval --stdin < "$O/light-probe.js" > "$O/data/light-$W-$pg.json" 2>/dev/null
    AB "$S" screenshot "$O/screenshots/pd-$W-$pg.png" >/dev/null 2>&1
    echo EXTRA_DONE $W $pg
  done
done
echo EXTRA_FINISH
