#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
PASS=$(grep -E '^ADMIN_PASS=' "$KEYS"|head -1|sed 's/^ADMIN_PASS=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-h5login2"
AB="agent-browser --session $S"
$AB open "https://$DOMAIN/laiyipao-h5/#/pages/login/login" >/dev/null 2>&1
sleep 4
$AB fill "input[type='text']" "admin" >/dev/null 2>&1
$AB fill "input[type='password']" "$PASS" >/dev/null 2>&1
echo "=== native click .btn-primary ==="
$AB click ".btn-primary" 2>&1 | sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g;s|$PASS|<ADMIN_PASS>|g"
sleep 5
echo "=== after click ==="
$AB get url 2>&1 | sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"
cat <<'EOF' | $AB eval --stdin 2>&1 | sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"
JSON.stringify({url:location.href,ls:Object.keys(localStorage),body:document.body.innerText.slice(0,100).replace(/\n/g,' ')})
EOF
