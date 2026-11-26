#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round71-verify-time-format"
D="$P/data"; S="$P/screenshots"
AB(){ agent-browser --session "$1" "${@:2}"; }
run_one(){
  local W="$1" H="$2" V="$3" MODE="$4"
  local SS="r71-${MODE}-${V}"
  AB "$SS" close >/dev/null 2>&1 || true
  if [ "$MODE" = "bad" ]; then
    AB "$SS" open --init-script "$D/auth-init.js" --init-script "$D/r71-created-bad.js" --init-script "$D/r71-init.js" >/dev/null 2>&1
  else
    AB "$SS" open --init-script "$D/auth-init.js" --init-script "$D/r71-init.js" >/dev/null 2>&1
  fi
  AB "$SS" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$SS" navigate "http://127.0.0.1:5201/#/pages/verify/verify" >/dev/null 2>&1
  sleep 5
  if [ "$MODE" = "double" ]; then
    AB "$SS" eval --stdin < "$D/r71-click.js" >/dev/null 2>&1
    AB "$SS" eval --stdin < "$D/r71-click.js" >/dev/null 2>&1
  elif [ "$MODE" = "fail" ]; then
    AB "$SS" eval "window.__qaCfg.fail=true; 'FAIL_ON'" >/dev/null 2>&1
    AB "$SS" eval --stdin < "$D/r71-click.js" >/dev/null 2>&1
  else
    AB "$SS" eval --stdin < "$D/r71-click.js" >/dev/null 2>&1
  fi
  sleep 2
  AB "$SS" eval --stdin < "$D/r71-probe.js" > "$D/${MODE}-${V}-probe.txt" 2>&1
  AB "$SS" screenshot "$S/r71-${MODE}-${V}.png" > "$D/${MODE}-${V}-shot.txt" 2>&1
  AB "$SS" close >/dev/null 2>&1 || true
  echo "DONE ${MODE} ${V}"
}
for V in 390 768 1280; do
  case "$V" in 390) W=390; H=844;; 768) W=768; H=1024;; 1280) W=1280; H=800;; esac
  for M in normal bad double fail; do run_one "$W" "$H" "$V" "$M"; done
done
echo ALL_DONE