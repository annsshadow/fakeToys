#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round66-gem-cache-guard"; D="$P/data"; S="$P/screenshots"
AB(){ agent-browser --session "$1" "${@:2}"; }
run_case(){
  local st="$1" vw="$2" w="$3" h="$4"
  local sess="r66-${st}-${vw}"
  AB "$sess" open --init-script "$D/auth-init.js" --init-script "$D/r66-init.js" --init-script "$D/state-${st}.js" >/dev/null 2>&1
  AB "$sess" set viewport "$w" "$h" >/dev/null 2>&1
  AB "$sess" navigate "http://127.0.0.1:5201/#/pages/gems/gems" >/dev/null 2>&1
  sleep 5
  AB "$sess" eval --stdin < "$P/r66-probe.js" > "$D/probe-${st}-${vw}.txt" 2>&1
  AB "$sess" screenshot "$S/r66-${st}-${vw}.png" >/dev/null 2>&1 || echo "SHOT_FAIL ${st}-${vw}"
  AB "$sess" close >/dev/null 2>&1 || true
  echo "CASE ${st}-${vw} done"
}
run_case broken 390 390 844
run_case broken 768 768 1024
run_case broken 1280 1280 800
run_case valid 390 390 844
run_case valid 768 768 1024
run_case valid 1280 1280 800
run_case empty 390 390 844
run_case empty 768 768 1024
run_case empty 1280 1280 800
echo DONE_ALL
