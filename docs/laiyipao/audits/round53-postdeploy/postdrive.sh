#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
PD="$ROOT/docs/laiyipao/audits/round53-postdeploy"
AB(){ agent-browser --session "$1" "${@:2}"; }
run_case(){
  local S="$1" OUT="$2" BODY="$3" W="$4" H="$5"
  local BODYJSON CFGBODY
  BODYJSON=$(cat "$PD/data/$BODY.json")
  CFGBODY=$(cat "$PD/data/config-case1.json")
  AB "$S" close >/dev/null 2>&1 || true
  AB "$S" open >/dev/null 2>&1
  AB "$S" open --init-script "$PD/data/auth-init.js" "http://127.0.0.1:5201/#/pages/index/index" >/dev/null 2>&1
  AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
  sleep 3
  AB "$S" network unroute >/dev/null 2>&1 || true
  AB "$S" network route "http://127.0.0.1:8080/api/v1/me" --body "$BODYJSON" --allow-origin http://127.0.0.1:5201 > "$PD/data/$OUT-route.txt" 2>&1
  AB "$S" network route "http://127.0.0.1:8080/api/v1/config" --body "$CFGBODY" --allow-origin http://127.0.0.1:5201 > "$PD/data/$OUT-cfgroute.txt" 2>&1
  AB "$S" reload > "$PD/data/$OUT-reload.txt" 2>&1
  sleep 5
  cat "$PD/r53-probe.js" | AB "$S" eval --stdin > "$PD/data/pd-$OUT.json" 2>&1
  AB "$S" screenshot "$PD/screenshots/pd-$OUT.png" >/dev/null 2>&1
  echo "$OUT DONE"
}
run_case lyp-r53pd1 case1 me-case1 390 844
run_case lyp-r53pd2 empty me-empty 768 1024
run_case lyp-r53pd3 case2 me-case2 1280 800
echo POSTDRIVE_DONE=YES
