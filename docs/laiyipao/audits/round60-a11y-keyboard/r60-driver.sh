#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round60-a11y-keyboard"
SESS=lyp-r60-key
BASE="http://127.0.0.1:5201/#/pages"
AB(){ agent-browser --session "$1" "${@:2}"; }
AB "$SESS" open >/dev/null
for SPEC in "390 844 announcements announcements" "768 1024 announcements announcements" "1280 800 announcements announcements" "390 844 reactions reactions" "768 1024 reactions reactions" "1280 800 reactions reactions" "390 844 equipment equipment" "768 1024 equipment equipment" "1280 800 equipment equipment" "390 844 skills skills" "768 1024 skills skills" "1280 800 skills skills" "390 844 shop shop" "768 1024 shop shop" "1280 800 shop shop" "390 844 mastery mastery" "768 1024 mastery mastery" "1280 800 mastery mastery"; do
  read -r W H PAGE OUT <<<"$SPEC"
  NAME="$W-$PAGE"
  AB "$SESS" open --init-script "$D/data/auth-init.js" "$BASE/$PAGE/$PAGE" >/dev/null
  AB "$SESS" set viewport "$W" "$H" >/dev/null
  sleep 2
  AB "$SESS" eval --stdin < "$D/r60-probe.js" > "$D/data/probe-$NAME.json" 2>"$D/data/probe-$NAME.err" || echo "PROBE_ERR $NAME"
  echo "PROBE $NAME rc=$?"
  AB "$SESS" screenshot "$D/screenshots/r60-$NAME.png" >/dev/null
  echo "SHOT $NAME"
  # Tab sequence: up to 25 tabs, record activeElement after each
  : > "$D/data/tabs-$NAME.txt"
  for i in $(seq 1 25); do
    AB "$SESS" press Tab >/dev/null
    AB "$SESS" eval --stdin < "$D/r60-focus.js" >> "$D/data/tabs-$NAME.txt" 2>>"$D/data/tabs-$NAME.err" || echo "TAB_ERR $NAME $i"
  done
  echo "TABS $NAME lines=$(wc -l < "$D/data/tabs-$NAME.txt")"
done
AB "$SESS" close >/dev/null 2>&1 || true
echo WALK_FINISH
