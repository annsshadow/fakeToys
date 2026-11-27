#!/usr/bin/env bash
set -uo pipefail
ROOT=$(pwd)
D="$ROOT/docs/laiyipao/audits/round81-skill-collection-progress"
AB(){ agent-browser --session "$1" "${@:2}"; }
for V in 390 768 1280; do
  case $V in 390) H=844;; 768) H=1024;; 1280) H=800;; esac
  S="r81-$V"
  AB "$S" close --all >/dev/null 2>&1 || true
  AB "$S" open --init-script "$D/data/auth-init.js" >/dev/null 2>&1
  AB "$S" set viewport "$V" "$H" >/dev/null 2>&1
  AB "$S" navigate "http://127.0.0.1:5201/#/pages/skills/skills" >/dev/null 2>&1
  sleep 6
  AB "$S" eval --stdin < "$D/data/r81-probe.js" > "$D/data/probe-$V.txt" 2>&1
  AB "$S" screenshot "$D/screenshots/r81-skills-$V.png" >/dev/null 2>&1
  AB "$S" navigate "http://127.0.0.1:5201/#/pages/equipment/equipment" >/dev/null 2>&1
  sleep 5
  AB "$S" eval --stdin < "$D/data/r81-probe-eq.js" > "$D/data/probe-eq-$V.txt" 2>&1
  echo "VIEW $V done"
  AB "$S" close --all >/dev/null 2>&1 || true
done
echo WALK_DONE