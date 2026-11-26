#!/usr/bin/env bash
set -uo pipefail
AB(){ agent-browser --session "$1" "${@:2}"; }
S=lyp-r55-pd
O=docs/laiyipao/audits/round55-postdeploy
INIT="$O/data/auth-init.js"
PROBE="$O/r55-probe.js"
AB "$S" open >/dev/null 2>&1
VW=(390 768 1280); VH=(844 1024 800)
for i in 0 1 2; do
  W=${VW[$i]}; H=${VH[$i]}
  for pg in reactions bag; do
    AB "$S" open --init-script "$INIT" "http://127.0.0.1:5201/#/pages/$pg/$pg" >/dev/null 2>&1
    AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
    sleep 3
    AB "$S" eval --stdin < "$PROBE" > "$O/data/pd-probe-$W-$pg.json" 2>"$O/data/pd-err-$W-$pg.txt"
    AB "$S" screenshot "$O/screenshots/pd-$W-$pg.png" >/dev/null 2>&1
    echo PD_DONE $W $pg
  done
done
echo R55_PD_FINISH
