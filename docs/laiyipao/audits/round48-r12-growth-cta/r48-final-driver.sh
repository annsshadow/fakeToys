#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round48-r12-growth-cta"

run_scene() {
  local NAME="$1" BODYFILE="$2" DEEPLINK="$3" PREP="$4"
  local S="lyp-r48-$NAME"
  local AB="agent-browser --session $S"
  $AB close >/dev/null 2>&1 || true
  $AB open >/dev/null 2>&1
  local BODY; BODY=$(cat "$D/data/$BODYFILE")
  $AB open --init-script "$D/data/init-element.js" "$DEEPLINK" >/dev/null 2>&1
  $AB set viewport 390 844 >/dev/null 2>&1
  sleep 3
  $AB network unroute >/dev/null 2>&1 || true
  $AB network route "http://127.0.0.1:8080/api/v1/me" --body "$BODY" --allow-origin http://127.0.0.1:5201 > "$D/data/$NAME-route.txt" 2>&1
  echo "${NAME}_ROUTE_RC=$?"
  $AB reload > "$D/data/$NAME-reload.txt" 2>&1
  sleep 5
  $AB wait .r12-priority-cta >/dev/null 2>&1 || true
  cat "$D/r48-probe.js" | $AB eval --stdin > "$D/data/$NAME-before.json" 2>&1
  if [ -n "$PREP" ]; then $AB click "$PREP" > "$D/data/$NAME-prep.txt" 2>&1; sleep 1; fi
  $AB screenshot "$D/screenshots/r48-$NAME-before.png" >/dev/null 2>&1
  local CX CY
  CX=$(node "$D/data/getnum.js" "$D/data/$NAME-before.json" ctaRect.cx)
  CY=$(node "$D/data/getnum.js" "$D/data/$NAME-before.json" ctaRect.cy)
  echo "${NAME}_CTA=$CX,$CY"
  $AB mouse move "$CX" "$CY" > "$D/data/$NAME-move.txt" 2>&1
  $AB mouse down left > "$D/data/$NAME-down.txt" 2>&1
  $AB mouse up left > "$D/data/$NAME-up.txt" 2>&1
  sleep 2
  cat "$D/r48-probe.js" | $AB eval --stdin > "$D/data/$NAME-after.json" 2>&1
  $AB screenshot "$D/screenshots/r48-$NAME-after.png" >/dev/null 2>&1
  echo "${NAME}_DONE=YES"
}

run_scene element me-element.json "http://127.0.0.1:5201/#/pages/bag/bag" ".bag-tabs .chip:nth-child(2)"
run_scene equipment me-equipment.json "http://127.0.0.1:5201/#/pages/equipment/equipment" ".r3-filter .r3-chip:nth-child(2)"
run_scene reaction me-reaction.json "http://127.0.0.1:5201/#/pages/bag/bag" ""
node "$D/r48-scene-summary.js" > "$D/data/scene-summary.txt" 2>&1
cat "$D/data/scene-summary.txt"
echo R48_FINAL_DRIVER_DONE=YES
