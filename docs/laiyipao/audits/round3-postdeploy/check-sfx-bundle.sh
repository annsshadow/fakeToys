#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-posth5b"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"; }
$AB open "https://$DOMAIN/laiyipao-h5/" >/dev/null 2>&1; sleep 3
echo "=== sfx filter in deployed bundle ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/check-sfx-bundle.js 2>&1 | mask
echo done