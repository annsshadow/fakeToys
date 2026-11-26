#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-posth5b"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"; }
sleep 30
echo "=== probe final ==="
cat <<EOF | $AB eval --stdin 2>&1 | mask
JSON.stringify(window.__LYP_PROBE__ ? {pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length,fr:window.__LYP_PROBE__.failedRequests.length} : null)
EOF
echo "=== battle records ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/probe-battles.js 2>&1 | mask
echo done