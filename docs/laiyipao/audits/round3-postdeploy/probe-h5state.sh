#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-posth5"
AB="agent-browser --session $S"
echo "=== current url ==="
$AB get url 2>&1 | sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"
echo "=== storage ==="
cat <<'EOF' | $AB eval --stdin 2>&1 | sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g;s|$ADMIN_PASS|<ADMIN_PASS>|g"
JSON.stringify({ls:Object.keys(localStorage),lsTokens:Object.keys(localStorage).filter(function(k){return /token|auth|user|session/i.test(k)}).map(function(k){return k+'='+String(localStorage.getItem(k)).slice(0,20)}),ss:Object.keys(sessionStorage)})
EOF
echo "=== apiFailures detail ==="
cat <<'EOF' | $AB eval --stdin 2>&1 | sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"
JSON.stringify((window.__LYP_PROBE__?.apiFailures||[]).slice(-10))
EOF
