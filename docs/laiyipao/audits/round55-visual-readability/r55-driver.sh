#!/usr/bin/env bash
set -uo pipefail
AB(){ agent-browser --session "$1" "${@:2}"; }
S=lyp-r55-vis
O=docs/laiyipao/audits/round55-visual-readability
INIT="$O/data/auth-init.js"
VW=(390 768 1280); VH=(844 1024 800)
echo R55_WALK_START=$(date +%s)
AB "$S" open >/dev/null 2>&1
for i in 0 1 2; do
  W=${VW[$i]}; H=${VH[$i]}
  for pg in reactions/reactions bag/bag; do
    nm=${pg/\//-}
    AB "$S" open --init-script "$INIT" "http://127.0.0.1:5201/#/pages/$pg" >/dev/null 2>&1
    AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
    sleep 3
    AB "$S" eval --stdin < "$O/r55-probe.js" > "$O/data/probe-$W-$nm.json" 2>"$O/data/evalerr-$W-$nm.txt"
    AB "$S" screenshot "$O/screenshots/r55-$W-$nm.png" >/dev/null 2>&1
    echo DONE $W $nm
  done
done
echo R55_WALK_FINISH=$(date +%s)
