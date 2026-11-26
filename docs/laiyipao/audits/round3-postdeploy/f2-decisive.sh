#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-posth5b"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"; }
echo "=== install post-listener (after page load / initSfx) ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/f2-diag-install.js 2>&1 | mask
echo "=== reset diag counters ==="
cat <<EOF | $AB eval --stdin 2>&1 | mask
window.__F2_DIAG__={interrupted:0,preventedTrue:0,preventedFalse:0};"reset"
EOF
echo "=== restart battle via CDP nav (reuse token) ==="
$AB open "https://$DOMAIN/laiyipao-h5/#/pages/battle/battle" >/dev/null 2>&1; sleep 3
echo "=== reinstall listener after nav (initSfx re-runs) ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/f2-diag-install.js 2>&1 | mask
cat <<EOF | $AB eval --stdin 2>&1 | mask
window.__F2_DIAG__={interrupted:0,preventedTrue:0,preventedFalse:0};"reset2"
EOF
echo "=== play 18s ==="
for n in 1 2 3 4 5 6; do $AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/e2e-battle-check.js >/dev/null 2>&1; sleep 3; done
echo "=== RESULT ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/f2-final-read.js 2>&1 | mask
echo done