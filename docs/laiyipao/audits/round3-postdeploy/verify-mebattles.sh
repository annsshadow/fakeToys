#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-posth5b"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"; }
echo "=== /me/battles ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/probe-mebattles.js 2>&1 | mask
echo done