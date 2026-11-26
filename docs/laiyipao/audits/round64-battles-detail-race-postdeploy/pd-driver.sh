#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round64-battles-detail-race"
PD="docs/laiyipao/audits/round64-battles-detail-race-postdeploy"
D="$PD/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
cp "$P/data/auth-init.js" "$D/auth-init.js"

run_view(){
  local W="$1" H="$2" V="$3"; local S="r64pd$V"
  AB "$S" open --init-script "$D/auth-init.js" --init-script "$P/race-init2.js" >/dev/null 2>&1
  AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$S" navigate "http://127.0.0.1:5201/#/pages/battles/battles" >/dev/null 2>&1
  sleep 4
  AB "$S" eval --stdin < "$P/race-set2.js" >/dev/null 2>&1
  AB "$S" eval --stdin < "$P/r64-clickB.js" > "$D/pd-clickB-$V.txt" 2>&1
  sleep 0.4
  AB "$S" eval --stdin < "$P/r64-clickI4.js" > "$D/pd-clickI4-$V.txt" 2>&1
  sleep 4.2
  AB "$S" eval --stdin < "$P/r64-probe.js" > "$D/pd-race-$V.txt" 2>&1
  AB "$S" screenshot "$PD/screenshots/pd-race-$V.png" >/dev/null 2>&1 || echo "SHOT_FAIL_$V"
  echo "PD_VIEW_$V DONE"
}
run_view 414 896 414
run_view 1440 900 1440
run_view 820 1180 820
echo PD_RACE_DONE
