#!/usr/bin/env bash
set -uo pipefail
P=docs/laiyipao/audits/round69-nickname-busy-postdeploy
QA=docs/laiyipao/audits/round69-nickname-busy
D="$P/data"
S="$P/screenshots"
QD="$QA/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
run_case(){
  local W="$1" H="$2" V="$3" MODE="$4"
  local SS="r69pd-$MODE-$V"
  AB "$SS" open --init-script "$D/auth-init.js" --init-script "$QD/r69-init.js" --init-script "$QD/r69-err-init.js" >/dev/null 2>&1
  AB "$SS" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$SS" navigate "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1
  sleep 5
  AB "$SS" eval --stdin < "$QA/r69-open-edit.js" > "$D/pd-$MODE-$V-edit.txt" 2>&1
  AB "$SS" eval --stdin < "$QA/r69-fill.js" > "$D/pd-$MODE-$V-fill.txt" 2>&1
  if [ "$MODE" = "slow" ]; then
    AB "$SS" eval "window.__qaNet.puts=[]; window.__qaNet.slow=true; 'ON'" >/dev/null 2>&1
    AB "$SS" eval --stdin < "$QA/r69-click.js" > "$D/pd-$MODE-$V-click.txt" 2>&1
    sleep 1
    AB "$SS" eval --stdin < "$P/pd-probe.js" > "$D/pd-$MODE-$V-probe-1s.txt" 2>&1
    AB "$SS" screenshot "$S/pd-$MODE-$V.png" > "$D/pd-$MODE-$V-shot.txt" 2>&1
    sleep 3
    AB "$SS" eval --stdin < "$P/pd-probe.js" > "$D/pd-$MODE-$V-probe-after.txt" 2>&1
  elif [ "$MODE" = "double" ]; then
    AB "$SS" eval "window.__qaNet.puts=[]; 'ON'" >/dev/null 2>&1
    AB "$SS" eval --stdin < "$QA/r69-click.js" > "$D/pd-$MODE-$V-click1.txt" 2>&1
    AB "$SS" eval --stdin < "$QA/r69-click.js" > "$D/pd-$MODE-$V-click2.txt" 2>&1
    sleep 3
    AB "$SS" eval --stdin < "$P/pd-probe.js" > "$D/pd-$MODE-$V-probe.txt" 2>&1
    AB "$SS" screenshot "$S/pd-$MODE-$V.png" > "$D/pd-$MODE-$V-shot.txt" 2>&1
  else
    AB "$SS" eval "window.__qaNet.puts=[]; 'ON'" >/dev/null 2>&1
    AB "$SS" network route "**/me/nickname" --abort >/dev/null 2>&1
    AB "$SS" eval --stdin < "$QA/r69-click.js" > "$D/pd-$MODE-$V-click.txt" 2>&1
    sleep 1
    AB "$SS" eval --stdin < "$P/pd-probe.js" > "$D/pd-$MODE-$V-probe.txt" 2>&1
    AB "$SS" screenshot "$S/pd-$MODE-$V.png" > "$D/pd-$MODE-$V-shot.txt" 2>&1
  fi
  AB "$SS" close >/dev/null 2>&1 || true
  echo "CASE_DONE $MODE $V"
}
for V in 414 820 1440; do
  case "$V" in 414) W=414; H=896;; 820) W=820; H=1180;; 1440) W=1440; H=900;; esac
  for M in slow double fail; do run_case "$W" "$H" "$V" "$M"; done
done
echo PD_DONE_ALL
