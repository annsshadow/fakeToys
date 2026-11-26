#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round48-r12-growth-cta"
S="lyp-r48-route-test3"
AB="agent-browser --session $S"
$AB close >/dev/null 2>&1 || true
$AB open >/dev/null 2>&1
BODY=$(cat "$D/data/me-element.json")
$AB network route "**/api/v1/me" --body "$BODY" --allow-origin http://127.0.0.1:8080 > "$D/data/rt3-route.txt" 2>&1
echo RT3_ROUTE_RC=$?
$AB open --init-script "$D/data/init-element.js" "http://127.0.0.1:5201/#/pages/bag/bag" >/dev/null 2>&1
$AB set viewport 390 844 >/dev/null 2>&1
sleep 5
cat "$D/r48-probe.js" | $AB eval --stdin > "$D/data/rt3-probe.json" 2>&1
$AB network requests --filter "api/v1/me" > "$D/data/rt3-net.txt" 2>&1
echo RT3_DONE
