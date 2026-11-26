#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-h5probe"
AB="agent-browser --session $S"
$AB open "https://$DOMAIN/laiyipao-h5/#/pages/login/login" >/dev/null 2>&1
sleep 4
cat <<'EOF' | $AB eval --stdin 2>&1 | sed "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"
(function(){var all=[...document.querySelectorAll('*')].filter(function(e){return (e.innerText||'').trim()==='登录' && e.children.length===0});return JSON.stringify(all.map(function(e){return {tag:e.tagName,cls:(e.className||'').slice(0,60),role:e.getAttribute('role'),parentCls:(e.parentElement?.className||'').slice(0,60),parentTag:e.parentElement?.tagName}}));})()
EOF
