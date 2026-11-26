#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round60-postdeploy"
SESS=lyp-r60-pd
BASE="http://127.0.0.1:5201/#/pages"
AB(){ agent-browser --session "$SESS" "$@"; }
AB open >/dev/null 2>&1
printf "%s\n" "414 896" "1440 900" | while read -r W H; do
  for P in announcements reactions equipment skills shop mastery; do
    NAME="$W-$P"
    AB open --init-script "$D/data/auth-init.js" "$BASE/$P/$P" >/dev/null 2>&1
    AB set viewport "$W" "$H" >/dev/null 2>&1
    sleep 2
    AB eval --stdin < "$D/pd-probe.js" > "$D/data/pd-$NAME.json" 2>"$D/data/pd-$NAME.err" || echo "PROBE_ERR $NAME"
    AB screenshot "$D/screenshots/pd-$NAME.png" >/dev/null 2>&1
    echo "SHOT $NAME rc=$?"
  done
done
for P in announcements reactions equipment skills shop mastery; do
  AB open --init-script "$D/data/auth-init.js" "$BASE/$P/$P" >/dev/null 2>&1
  AB set viewport 768 1024 >/dev/null 2>&1
  sleep 2
  : > "$D/data/pd-tabs-$P.txt"
  for i in $(seq 1 12); do
    AB press Tab >/dev/null 2>&1
    AB eval --stdin < "$D/pd-focus.js" >> "$D/data/pd-tabs-$P.txt" 2>/dev/null
  done
  AB eval --stdin < "$D/pd-focus.js" > "$D/data/pd-focuslast-$P.json" 2>/dev/null
  AB eval --stdin < "$D/pd-state.js" > "$D/data/pd-state-$P-before.json" 2>/dev/null
  AB press Enter >/dev/null 2>&1
  sleep 1
  AB eval --stdin < "$D/pd-state.js" > "$D/data/pd-state-$P-after.json" 2>/dev/null
  echo "TABENTER $P done"
done
AB close >/dev/null 2>&1 || true
echo PD_FINISH