#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round48-r12-growth-cta"
S="lyp-r48-route-test"
AB="agent-browser --session $S"
$AB close >/dev/null 2>&1 || true
$AB open >"$D/data/rt-open0.txt" 2>&1
BODY=$(cat "$D/data/me-element.json")
$AB network route 'http://127.0.0.1:8080/api/v1/me' --body "$BODY" --allow-origin http://127.0.0.1:8080 >"$D/data/rt-route.txt" 2>&1
echo ROUTE_RC=$?
$AB open --init-script "$D/data/init-element.js" 'http://127.0.0.1:5201/#/pages/bag/bag' >"$D/data/rt-open.txt" 2>&1
$AB set viewport 390 844 >"$D/data/rt-vp.txt" 2>&1
sleep 4
cat "$D/r48-probe.js" | $AB eval --stdin >"$D/data/rt-probe.json" 2>&1
$AB network requests >"$D/data/rt-net.txt" 2>&1
echo RT_DONE
