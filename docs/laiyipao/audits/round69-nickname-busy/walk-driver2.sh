#!/usr/bin/env bash
set -uo pipefail
P="docs/laiyipao/audits/round69-nickname-busy"; D="$P/data"; S="$P/screenshots"
AB(){ agent-browser --session "$1" "${@:2}"; }

run_one(){
  local W="$1" H="$2" V="$3" MODE="$4"
  local SS="r69b-${MODE}-${V}"
  AB "$SS" open --init-script "$D/auth-init.js" --init-script "$D/r69-init.js" >/dev/null 2>&1
  AB "$SS" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$SS" navigate "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1
  sleep 5
  AB "$SS" eval --stdin < "$P/r69-open-edit.js" > "$D/b-${MODE}-${V}-edit.txt" 2>&1
  AB "$SS" eval --stdin < "$P/r69-fill.js" > "$D/b-${MODE}-${V}-fill.txt" 2>&1
  if [ "$MODE" = "slow" ]; then
    AB "$SS" eval "window.__qaNet.puts=[]; window.__qaNet.slow=true; 'ON'" >/dev/null 2>&1
    AB "$SS" eval --stdin < "$P/r69-click.js" > "$D/b-${MODE}-${V}-click.txt" 2>&1
    sleep 1
    AB "$SS" eval --stdin < "$P/r69-probe.js" > "$D/b-${MODE}-${V}-probe-1s.txt" 2>&1
    AB "$SS" screenshot "$S/r69b-${MODE}-${V}.png" > "$D/b-${MODE}-${V}-shot.txt" 2>&1
    sleep 3
    AB "$SS" eval --stdin < "$P/r69-probe.js" > "$D/b-${MODE}-${V}-probe-after.txt" 2>&1
  elif [ "$MODE" = "double" ]; then
    AB "$SS" eval "window.__qaNet.puts=[]; 'ON'" >/dev/null 2>&1
    AB "$SS" eval --stdin < "$P/r69-click.js" > "$D/b-${MODE}-${V}-click1.txt" 2>&1
    AB "$SS" eval --stdin < "$P/r69-click.js" > "$D/b-${MODE}-${V}-click2.txt" 2>&1
    sleep 3
    AB "$SS" eval --stdin < "$P/r69-probe.js" > "$D/b-${MODE}-${V}-probe.txt" 2>&1
    AB "$SS" screenshot "$S/r69b-${MODE}-${V}.png" > "$D/b-${MODE}-${V}-shot.txt" 2>&1
  else
    AB "$SS" eval "window.__qaNet.puts=[]; 'ON'" >/dev/null 2>&1
    AB "$SS" network route "**/me/nickname" --abort >/dev/null 2>&1
    AB "$SS" eval --stdin < "$P/r69-click.js" > "$D/b-${MODE}-${V}-click.txt" 2>&1
    sleep 1
    AB "$SS" eval --stdin < "$P/r69-probe.js" > "$D/b-${MODE}-${V}-probe.txt" 2>&1
    AB "$SS" screenshot "$S/r69b-${MODE}-${V}.png" > "$D/b-${MODE}-${V}-shot.txt" 2>&1
  fi
  AB "$SS" close >/dev/null 2>&1 || true
  echo "DONE ${MODE} ${V}"
}

for V in 390 768 1280; do
  case "$V" in 390) W=390; H=844;; 768) W=768; H=1024;; 1280) W=1280; H=800;; esac
  for M in slow double fail; do run_one "$W" "$H" "$V" "$M"; done
done
echo ALL_DONE
