#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round53-home-pct-rounding"
AB(){ agent-browser --session "$1" "${@:2}"; }
run_case(){
  local S="$1" OUT="$2" BODYNAME="$3" W="$4" H="$5"
  local BODY CFGBODY
  BODY=$(cat "$D/data/me-$BODYNAME.json")
  CFGBODY=$(cat "$D/data/config-$BODYNAME.json")
  AB "$S" close >/dev/null 2>&1 || true
  AB "$S" open >/dev/null 2>&1
  AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/index/index" >/dev/null 2>&1
  AB "$S" set viewport "$W" "$H" >/dev/null 2>&1
  sleep 3
  AB "$S" network unroute >/dev/null 2>&1 || true
  AB "$S" network route "http://127.0.0.1:8080/api/v1/me" --body "$BODY" --allow-origin http://127.0.0.1:5201 > "$D/data/$OUT-route.txt" 2>&1
  AB "$S" network route "http://127.0.0.1:8080/api/v1/config" --body "$CFGBODY" --allow-origin http://127.0.0.1:5201 > "$D/data/$OUT-cfgroute.txt" 2>&1
  AB "$S" reload > "$D/data/$OUT-reload.txt" 2>&1
  sleep 5
  cat "$D/r53-probe.js" | AB "$S" eval --stdin > "$D/data/$OUT-probe.json" 2>&1
  AB "$S" screenshot "$D/screenshots/r53-$OUT.png" >/dev/null 2>&1
  echo "${OUT}_PROBE_DONE vp=${W}x${H}"
}
run_case lyp-r53-c1 case1 case1 390 844
run_case lyp-r53-c2 case2 case1 768 1024
run_case lyp-r53-c3 case3 case1 1280 800
run_case lyp-r53-empty empty empty 390 844
node "$D/data/summary.js" > "$D/data/summary.txt" 2>&1
cat "$D/data/summary.txt"
echo R53_DRIVER_DONE=YES
