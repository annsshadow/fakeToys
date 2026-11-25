#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round51-me-radar-growth"
AB="agent-browser --session lyp-r51"
run_case() {
  local NAME="$1" VP="$2" W="$3" H="$4"
  local BODY; BODY=$(cat "$D/data/me-$NAME.json")
  local CFGBODY; CFGBODY=$(cat "$D/data/config-$NAME.json")
  $AB close >/dev/null 2>&1 || true
  $AB open >/dev/null 2>&1
  $AB open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1
  $AB set viewport "$W" "$H" >/dev/null 2>&1
  sleep 3
  $AB network unroute >/dev/null 2>&1 || true
  $AB network route "http://127.0.0.1:8080/api/v1/me" --body "$BODY" --allow-origin http://127.0.0.1:5201 > "$D/data/$NAME-route.txt" 2>&1
  $AB network route "http://127.0.0.1:8080/api/v1/config" --body "$CFGBODY" --allow-origin http://127.0.0.1:5201 > "$D/data/$NAME-cfgroute.txt" 2>&1
  $AB reload > "$D/data/$NAME-reload.txt" 2>&1
  sleep 5
  cat "$D/r51-probe.js" | $AB eval --stdin > "$D/data/$NAME-probe.json" 2>&1
  $AB screenshot "$D/screenshots/r51-$NAME.png" >/dev/null 2>&1
  echo "${NAME}_PROBE_DONE vp=${W}x${H}"
}
run_case case1 390x844 390 844
run_case case2 768x1024 768 1024
run_case case3 1280x800 1280 800
$AB close >/dev/null 2>&1 || true
$AB open >/dev/null 2>&1
$AB open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/me/me" >/dev/null 2>&1
$AB set viewport 390 844 >/dev/null 2>&1
sleep 3
$AB network unroute >/dev/null 2>&1 || true
$AB network route "http://127.0.0.1:8080/api/v1/me" --body '{"user_id":10,"account":{"id":10,"nickname":"QA"},"build":null,"build_rating":null,"power":1,"max_stage":60}' --allow-origin http://127.0.0.1:5201 >/dev/null 2>&1
$AB reload >/dev/null 2>&1
sleep 5
cat "$D/r51-probe.js" | $AB eval --stdin > "$D/data/empty-probe.json" 2>&1
$AB screenshot "$D/screenshots/r51-empty.png" >/dev/null 2>&1
echo EMPTY_PROBE_DONE
$AB close >/dev/null 2>&1 || true
node "$D/data/summarize.js" > "$D/data/summary.txt" 2>&1
cat "$D/data/summary.txt"
echo R51_DRIVER_DONE=YES
