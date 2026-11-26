#!/usr/bin/env bash
set -uo pipefail
ROOT="$(pwd)"
PD="docs/laiyipao/audits/round66-gem-cache-guard-postdeploy"
QA="docs/laiyipao/audits/round66-gem-cache-guard"
D="$PD/data"; S="$PD/screenshots"; QD="$QA/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
run_case(){
  local st="$1" vw="$2" w="$3" h="$4"
  local sess="r66pd-${st}-${vw}"
  AB "$sess" open --init-script "$D/auth-init.js" --init-script "$QD/r66-init.js" --init-script "$QD/state-${st}.js" >/dev/null 2>&1
  AB "$sess" set viewport "$w" "$h" >/dev/null 2>&1
  AB "$sess" navigate "http://127.0.0.1:5201/#/pages/gems/gems" >/dev/null 2>&1
  sleep 5
  AB "$sess" eval --stdin < "$PD/pd-probe.js" > "$D/pd-probe-${st}-${vw}.txt" 2>&1
  AB "$sess" screenshot "$S/pd-${st}-${vw}.png" >/dev/null 2>&1 || echo "SHOT_FAIL ${st}-${vw}"
  AB "$sess" close >/dev/null 2>&1 || true
  echo "CASE ${st}-${vw} done"
}
run_case broken 414 414 896
run_case broken 820 820 1180
run_case broken 1440 1440 900
run_case valid 414 414 896
run_case valid 820 820 1180
run_case valid 1440 1440 900
run_case empty 414 414 896
run_case empty 820 820 1180
run_case empty 1440 1440 900
echo PD_DONE_ALL
