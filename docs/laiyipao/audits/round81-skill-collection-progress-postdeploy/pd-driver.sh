#!/usr/bin/env bash
set -uo pipefail
ROOT=$(pwd)
PD="$ROOT/docs/laiyipao/audits/round81-skill-collection-progress-postdeploy"
AB(){ agent-browser --session "$1" "${@:2}"; }
for V in 414 820 1440; do
  case $V in
    414) H=896;;
    820) H=1180;;
    1440) H=900;;
  esac
  S="r81pd-$V"
  AB "$S" close --all >/dev/null 2>&1 || true
  AB "$S" open --init-script "$PD/data/auth-init.js" >/dev/null 2>&1
  AB "$S" set viewport "$V" "$H" >/dev/null 2>&1
  AB "$S" navigate "http://127.0.0.1:5201/#/pages/skills/skills" >/dev/null 2>&1
  sleep 6
  AB "$S" eval --stdin < "$PD/pd-probe.js" > "$PD/data/pd-probe-$V.txt" 2>&1
  AB "$S" screenshot "$PD/screenshots/pd-skills-$V.png" >/dev/null 2>&1
  echo "CASE $V done"
  AB "$S" close --all >/dev/null 2>&1 || true
done
echo PD_DONE_ALL
