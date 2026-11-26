#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round51-me-radar-growth"
AB(){ agent-browser --session "$1" "${@:2}"; }
setup() {
  local S="$1" PAGE="$2"
  AB "$S" close >/dev/null 2>&1 || true
  AB "$S" open >/dev/null 2>&1
  AB "$S" open --init-script "$D/data/auth-init.js" "http://127.0.0.1:5201/#/pages/$PAGE" >/dev/null 2>&1
  AB "$S" set viewport 390 844 >/dev/null 2>&1
  sleep 3
  AB "$S" network unroute >/dev/null 2>&1 || true
  local BODY; BODY=$(cat "$D/data/me-case1.json")
  local CFGBODY; CFGBODY=$(cat "$D/data/config-case1.json")
  AB "$S" network route "http://127.0.0.1:8080/api/v1/me" --body "$BODY" --allow-origin http://127.0.0.1:5201 >/dev/null 2>&1
  AB "$S" network route "http://127.0.0.1:8080/api/v1/config" --body "$CFGBODY" --allow-origin http://127.0.0.1:5201 >/dev/null 2>&1
  AB "$S" reload >/dev/null 2>&1
  sleep 5
}
S=lyp-r51-click
setup "$S" me/me
cat "$D/r51-probe.js" | AB "$S" eval --stdin > "$D/data/click-before.json" 2>&1
AB "$S" screenshot "$D/screenshots/r51-click-before.png" >/dev/null 2>&1
CX=$(node "$D/data/getnum.js" "$D/data/click-before.json" ctaRect.cx)
CY=$(node "$D/data/getnum.js" "$D/data/click-before.json" ctaRect.cy)
echo CTA_CLICK=$CX,$CY
AB "$S" mouse move "$CX" "$CY" > "$D/data/click-move.txt" 2>&1
AB "$S" mouse down left > "$D/data/click-down.txt" 2>&1
AB "$S" mouse up left > "$D/data/click-up.txt" 2>&1
sleep 3
cat "$D/r51-probe.js" | AB "$S" eval --stdin > "$D/data/click-after.json" 2>&1
AB "$S" screenshot "$D/screenshots/r51-click-after.png" >/dev/null 2>&1
echo CLICK_DONE
S2=lyp-r51-index
setup "$S2" index/index
cat "$D/index-probe.js" | AB "$S2" eval --stdin > "$D/data/index-probe.json" 2>&1
AB "$S2" screenshot "$D/screenshots/r51-index.png" >/dev/null 2>&1
echo INDEX_DONE
AB "$S" close >/dev/null 2>&1 || true
AB "$S2" close >/dev/null 2>&1 || true
node "$D/data/summary2.js" > "$D/data/summary2.txt" 2>&1
cat "$D/data/summary2.txt"
echo R51_CLICK_COMPARE_DONE=YES
