#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-posth5b"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"; }
echo "=== install post-listener ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/f2-diag-install.js 2>&1 | mask
echo "=== navigate to battle fresh (triggers sfx) ==="
$AB open "https://$DOMAIN/laiyipao-h5/#/pages/index/index" >/dev/null 2>&1; sleep 2
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/f2-diag-install.js >/dev/null 2>&1
$AB open "https://$DOMAIN/laiyipao-h5/#/pages/battle/battle" >/dev/null 2>&1; sleep 2
echo "=== reinstall after nav (init-script may have reset) ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/f2-diag-install.js 2>&1 | mask
echo "=== play 15s ==="
for n in 1 2 3 4 5; do $AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/e2e-battle-check.js >/dev/null 2>&1; sleep 3; done
echo "=== F2 diag result ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/f2-diag-read.js 2>&1 | mask
echo done