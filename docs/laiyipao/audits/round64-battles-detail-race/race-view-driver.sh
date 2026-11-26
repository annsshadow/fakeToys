#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round64-battles-detail-race"; D="$P/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
run_view(){ local W="$1" H="$2" V="$3"; local S="r64v$V"
  AB "$S" open --init-script "$D/auth-init.js" --init-script "$P/race-init2.js" >/dev/null 2>&1
  AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$S" navigate "http://127.0.0.1:5201/#/pages/battles/battles" >/dev/null 2>&1
  sleep 4
  AB "$S" eval --stdin < "$P/race-set2.js" >/dev/null 2>&1
  AB "$S" eval --stdin < "$P/r64-clickB.js" >/dev/null 2>&1
  sleep 0.4
  AB "$S" eval --stdin < "$P/r64-clickI4.js" >/dev/null 2>&1
  sleep 4.2
  AB "$S" eval --stdin < "$P/r64-probe.js" > "$D/race-view-$V.txt" 2>&1
  AB "$S" screenshot "$P/screenshots/race-view-$V.png" >/dev/null 2>&1 || echo "SHOT_FAIL_$V"
  echo "VIEW_$V DONE"
}
run_view 390 844 390
run_view 768 1024 768
run_view 1280 800 1280
echo ALL_VIEWS_DONE