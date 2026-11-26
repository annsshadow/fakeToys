#!/usr/bin/env bash
set -uo pipefail
AB(){ agent-browser --session "$1" "${@:2}"; }
S=lyp-r57-vis
OUT=docs/laiyipao/audits/round57-visual-truncation
INIT="$OUT/data/auth-init.js"
BASE="http://127.0.0.1:5201/#/pages"
PROBE="$OUT/r57-probe.js"
PAGES="announcements/announcements enemies/enemies"
VW=(390 768 1280); VH=(844 1024 800)
echo WALK_START=$(date +%s)
AB "$S" open >/dev/null 2>&1
for i in 0 1 2; do
  W=${VW[$i]}; H=${VH[$i]}
  for pg in $PAGES; do
    name=${pg/\//-}
    AB "$S" open --init-script "$INIT" "$BASE/$pg" >/dev/null 2>&1
    AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
    sleep 2
    AB "$S" eval --stdin < "$PROBE" > "$OUT/data/probe-$W-$name.json" 2>"$OUT/data/evalerr-$W-$name.txt"
    AB "$S" screenshot "$OUT/screenshots/r57-$W-$name.png" >/dev/null 2>&1
    echo "DONE $W $name"
  done
done
echo WALK_FINISH=$(date +%s)