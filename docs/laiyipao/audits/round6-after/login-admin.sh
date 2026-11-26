#!/bin/bash
S=lyp-r6-admin; AB="agent-browser --session $S"
PASS=$(grep -oE "BOOTSTRAP_ADMIN_PASS[^,)]{0,60}" laiyipao/server/internal/config/config.go | grep -oE "\"[A-Za-z0-9_]+\"" | head -1 | tr -d '\''"' )
if [ ${#PASS} -ne 10 ]; then PASS=admin12345; fi
echo PASS_LEN=${#PASS}
$AB eval --stdin 2>&1 <<'EOF'
localStorage.clear(); JSON.stringify({cleared:1})
EOF
$AB fill "input[placeholder='admin']" 'admin' 2>&1 | tail -1
$AB fill "input[placeholder='至少 10 位']" "$PASS" 2>&1 | tail -1
$AB eval --stdin 2>&1 <<'EOF'
(function(){var bs=[].slice.call(document.querySelectorAll("button"));for(var i=0;i<bs.length;i++){var t=(bs[i].innerText||"").trim();if(t.indexOf("登")>=0&&t.indexOf("录")>=0){bs[i].click();return "clicked:"+t}}return "no-btn"})()
EOF
sleep 4
$AB eval --stdin 2>&1 <<'EOF'
JSON.stringify({url:location.href,title:document.title,hasToken:!!localStorage.getItem("lyp_admin_token"),tokenLen:(localStorage.getItem("lyp_admin_token")||"").length})
EOF
echo LOGIN_DONE