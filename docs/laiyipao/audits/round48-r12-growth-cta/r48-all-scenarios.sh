#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round48-r12-growth-cta"
BASE="http://127.0.0.1:5201"
run_scene(){
  local SCENE="$1" PAGE="$2" PREP="$3"
  local S="lyp-r48-$SCENE"
  local AB="agent-browser --session $S"
  local P="$D/data/$SCENE"
  $AB close >/dev/null 2>&1 || true
  $AB open >"$P-open0.txt" 2>&1
  local BODY
  BODY=$(cat "$D/data/me-$SCENE.json")
  $AB network route "**/api/v1/me" --body "$BODY" --allow-origin http://127.0.0.1:8080 >"$P-route.txt" 2>&1
  echo "$SCENE ROUTE_RC=$?"
  $AB open --init-script "$D/data/init-element.js" "$BASE/#$PAGE" >"$P-open.txt" 2>&1
  echo "$SCENE OPEN_URL=$($AB get url 2>/dev/null || true)"
  $AB set viewport 390 844 >"$P-vp.txt" 2>&1
  $AB wait '.r12-priority-cta' >"$P-wait.txt" 2>&1
  sleep 3
  cat "$D/r48-probe.js" | $AB eval --stdin >"$P-before.json" 2>&1
  $AB screenshot "$D/screenshots/r48-$SCENE-before.png" >"$P-ss1.txt" 2>&1
  local RECTSRC="$P-before.json"
  if [ "$PREP" != "none" ]; then
    $AB click "$PREP" >"$P-prep.txt" 2>&1
    sleep 1
    cat "$D/r48-probe.js" | $AB eval --stdin >"$P-prepared.json" 2>&1
    RECTSRC="$P-prepared.json"
  fi
  local CX CY
  CX=$(node "$D/data/getnum.js" "$RECTSRC" ctaRect.cx)
  CY=$(node "$D/data/getnum.js" "$RECTSRC" ctaRect.cy)
  echo "$SCENE CTA_CLICK=$CX,$CY"
  $AB mouse move "$CX" "$CY" >"$P-move.txt" 2>&1
  $AB mouse down left >"$P-down.txt" 2>&1
  $AB mouse up left >"$P-up.txt" 2>&1
  sleep 2
  cat "$D/r48-probe.js" | $AB eval --stdin >"$P-after.json" 2>&1
  $AB screenshot "$D/screenshots/r48-$SCENE-after.png" >"$P-ss2.txt" 2>&1
  echo "$SCENE AFTER_URL=$($AB get url 2>/dev/null || true)"
}
run_scene element "/pages/bag/bag" ".bag-tabs .chip:nth-child(2)"
run_scene equipment "/pages/equipment/equipment" ".r3-filter .r3-chip:nth-child(2)"
run_scene reaction "/pages/bag/bag" "none"
echo ALL_SCENARIOS_DONE
