#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round48-postdeploy"
BASE="http://127.0.0.1:5201"
API="http://127.0.0.1:8080/api/v1/me"

run_scene() {
  local NAME="$1" BODYFILE="$2" DEEPLINK="$3" PREP="$4"
  local S="lyp-r48pd-$NAME"
  local AB="agent-browser --session $S"
  local BODY; BODY=$(cat "$D/data/$BODYFILE")
  echo "START=$NAME"
  $AB close >/dev/null 2>&1 || true
  $AB open >/dev/null 2>&1
  $AB open --init-script "$D/data/auth-init.js" "$BASE/#$DEEPLINK" > "$D/data/$NAME-open.txt" 2>&1
  $AB set viewport 390 844 > "$D/data/$NAME-vp.txt" 2>&1
  sleep 2
  $AB network unroute > "$D/data/$NAME-unroute.txt" 2>&1 || true
  $AB network route "$API" --body "$BODY" --allow-origin "$BASE" > "$D/data/$NAME-route.txt" 2>&1
  echo "${NAME}_ROUTE_RC=$?"
  $AB reload > "$D/data/$NAME-reload.txt" 2>&1
  sleep 5
  $AB wait .r12-priority-cta > "$D/data/$NAME-wait.txt" 2>&1 || true
  if [ -n "$PREP" ]; then $AB click "$PREP" > "$D/data/$NAME-prep.txt" 2>&1; sleep 1; fi
  cat "$D/r48-probe.js" | $AB eval --stdin > "$D/data/$NAME-before.json" 2>&1
  $AB screenshot "$D/screenshots/pd-$NAME-before.png" > "$D/data/$NAME-ss1.txt" 2>&1
  local CX CY
  CX=$(node "$D/getnum.js" "$D/data/$NAME-before.json" ctaRect.cx)
  CY=$(node "$D/getnum.js" "$D/data/$NAME-before.json" ctaRect.cy)
  echo "${NAME}_CTA=$CX,$CY"
  $AB mouse move "$CX" "$CY" > "$D/data/$NAME-move.txt" 2>&1
  $AB mouse down left > "$D/data/$NAME-down.txt" 2>&1
  $AB mouse up left > "$D/data/$NAME-up.txt" 2>&1
  sleep 2
  cat "$D/r48-probe.js" | $AB eval --stdin > "$D/data/$NAME-after.json" 2>&1
  $AB screenshot "$D/screenshots/pd-$NAME-after.png" > "$D/data/$NAME-ss2.txt" 2>&1
  echo "${NAME}_AFTER_URL=$($AB get url 2>/dev/null || true)"
  echo "END=$NAME"
}

run_scene element me-element.json "/pages/bag/bag" ".bag-tabs .chip:nth-child(2)"
run_scene equipment me-equipment.json "/pages/equipment/equipment" ".r3-filter .r3-chip:nth-child(2)"
run_scene reaction me-reaction.json "/pages/bag/bag" ""
echo R48_POSTDEPLOY_E2E_DONE
