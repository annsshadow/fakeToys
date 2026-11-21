#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-h5probe"
AB="agent-browser --session $S"
$AB open "https://$DOMAIN/laiyipao-h5/#/pages/login/login" >/dev/null 2>&1
sleep 5
echo "=== URL ==="
$AB get url 2>&1 | sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"
echo "=== FORM ==="
cat <<'EOF' | $AB eval --stdin 2>&1 | sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"
JSON.stringify({url:location.href,title:document.title,inputs:[...document.querySelectorAll('input')].map(function(i){return {ph:i.placeholder||'',type:i.type||'text',cls:(i.className||'').slice(0,50)}}),buttons:[...document.querySelectorAll('button')].slice(0,8).map(function(b){return (b.innerText||'').trim()}),texts:[...document.querySelectorAll('[class*=btn],[role=button]')].slice(0,8).map(function(b){return (b.innerText||'').trim()}),body:document.body.innerText.slice(0,260).replace(/\n/g,' ')})
EOF
echo "=== DONE ==="
