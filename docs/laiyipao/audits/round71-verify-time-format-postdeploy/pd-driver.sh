#!/usr/bin/env bash
set -uo pipefail
cd /d/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248
PD="docs/laiyipao/audits/round71-verify-time-format-postdeploy"
QA="docs/laiyipao/audits/round71-verify-time-format"
D="$PD/data"; S="$PD/screenshots"; QD="$QA/data"
AB(){ agent-browser --session "$1" "${@:2}"; }
run_one(){
  local W="$1" H="$2" V="$3" M="$4"
  local SS="r71pd-${M}-${V}"
  AB "$SS" close >/dev/null 2>&1 || true
  if [ "$M" = "bad" ]; then
    AB "$SS" open --init-script "$D/auth-init.js" --init-script "$QD/r71-created-bad.js" --init-script "$QD/r71-init.js" >/dev/null 2>&1
  else
    AB "$SS" open --init-script "$D/auth-init.js" --init-script "$QD/r71-init.js" >/dev/null 2>&1
  fi
  AB "$SS" set viewport "$W" "$H" >/dev/null 2>&1
  AB "$SS" navigate "http://127.0.0.1:5201/#/pages/verify/verify" >/dev/null 2>&1
  sleep 5
  if [ "$M" = "double" ]; then
    AB "$SS" eval --stdin < "$QD/r71-click.js" >/dev/null 2>&1
    AB "$SS" eval --stdin < "$QD/r71-click.js" >/dev/null 2>&1
  elif [ "$M" = "fail" ]; then
    AB "$SS" eval "window.__qaCfg.fail=true; 'FAIL_ON'" >/dev/null 2>&1
    AB "$SS" eval --stdin < "$QD/r71-click.js" >/dev/null 2>&1
  else
    AB "$SS" eval --stdin < "$QD/r71-click.js" >/dev/null 2>&1
  fi
  sleep 2
  AB "$SS" eval --stdin < "$PD/pd-probe.js" > "$D/pd-${M}-${V}-probe.txt" 2>&1
  AB "$SS" screenshot "$S/pd-${M}-${V}.png" > "$D/pd-${M}-${V}-shot.txt" 2>&1
  AB "$SS" close >/dev/null 2>&1 || true
  echo "PD_DONE ${M} ${V}"
}
for V in 414 820 1440; do
  case "$V" in 414) W=414; H=896;; 820) W=820; H=1180;; 1440) W=1440; H=900;; esac
  for M in normal bad double fail; do run_one "$W" "$H" "$V" "$M"; done
done
echo PD_DONE_ALL
