#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round48-r12-growth-cta"
NAME="$1" BODYFILE="$2" DEEPLINK="$3" PREP="$4"
S="lyp-r48-$NAME"
AB="agent-browser --session $S"
BODY=$(cat "$D/data/$BODYFILE")
echo "START=$NAME $(date +%T)"
$AB close >/dev/null 2>&1 || true
echo "01_OPEN_BLANK"
$AB open >/dev/null 2>&1
echo "02_OPEN_DEEP"
$AB open --init-script "$D/data/init-element.js" "$DEEPLINK" >/dev/null 2>&1
$AB set viewport 390 844 >/dev/null 2>&1
sleep 3
$AB network unroute >/dev/null 2>&1 || true
echo "03_ROUTE"
$AB network route "http://127.0.0.1:8080/api/v1/me" --body "$BODY" --allow-origin http://127.0.0.1:5201 > "$D/data/$NAME-route.txt" 2>&1
echo "ROUTE_RC=$?"
$AB reload > "$D/data/$NAME-reload.txt" 2>&1
sleep 5
echo "04_PROBE_BEFORE"
cat "$D/r48-probe.js" | $AB eval --stdin > "$D/data/$NAME-before.json" 2>&1
node "$D/r48-scene-summary.js" one "$NAME"
PRIO=$(node "$D/data/getnum.js" "$D/data/$NAME-before.json" priorityText)
case "$PRIO" in
  *元素覆盖*) echo "SCENE_OK=element";;
  *装备契合*) echo "SCENE_OK=equipment";;
  *反应链*) echo "SCENE_OK=reaction";;
  *) echo "SCENE_UNEXPECTED=$PRIO"; exit 2;;
esac
if [ -n "$PREP" ]; then
  echo "05_PREP"
  $AB click "$PREP" > "$D/data/$NAME-prep.txt" 2>&1
  sleep 1
  cat "$D/r48-probe.js" | $AB eval --stdin > "$D/data/$NAME-before.json" 2>&1
fi
$AB screenshot "$D/screenshots/r48-$NAME-before.png" >/dev/null 2>&1
CX=$(node "$D/data/getnum.js" "$D/data/$NAME-before.json" ctaRect.cx)
CY=$(node "$D/data/getnum.js" "$D/data/$NAME-before.json" ctaRect.cy)
echo "06_CLICK=$CX,$CY"
$AB mouse move "$CX" "$CY" > "$D/data/$NAME-move.txt" 2>&1
$AB mouse down left > "$D/data/$NAME-down.txt" 2>&1
$AB mouse up left > "$D/data/$NAME-up.txt" 2>&1
sleep 2
cat "$D/r48-probe.js" | $AB eval --stdin > "$D/data/$NAME-after.json" 2>&1
$AB screenshot "$D/screenshots/r48-$NAME-after.png" >/dev/null 2>&1
node "$D/r48-scene-summary.js" one "$NAME"
echo "END=$NAME $(date +%T)"
