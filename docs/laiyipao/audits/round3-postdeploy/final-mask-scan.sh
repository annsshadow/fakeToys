#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
echo "=== PNG binary domain scan ==="
for f in docs/laiyipao/audits/round3-postdeploy/screenshots/*.png; do if strings "$f" 2>/dev/null | grep -qF "$DOMAIN"; then echo "LEAK: $(basename $f)"; fi; done; echo "PNG_SCAN_DONE"
echo "=== mask verification in live session ==="
S="lyp-postadmin"
AB="agent-browser --session $S"
$AB open "https://$DOMAIN/laiyipao-admin/#/admin/levels" >/dev/null 2>&1; sleep 3
cat <<EOF | $AB eval --stdin 2>&1 | sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"
document.querySelectorAll(".api-tag").forEach(function(e){e.style.display="none"});JSON.stringify({apiTagHidden:[...document.querySelectorAll(".api-tag")].every(function(e){return getComputedStyle(e).display==="none"}),count:document.querySelectorAll(".api-tag").length})
EOF
echo done