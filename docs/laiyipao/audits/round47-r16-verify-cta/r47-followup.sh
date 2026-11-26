#!/bin/bash
set -u
ROOT=$(git rev-parse --show-toplevel) || exit 1
cd "$ROOT" || exit 1
D=docs/laiyipao/audits/round47-r16-verify-cta
DATA=$D/data; SHOTS=$D/screenshots
AB="agent-browser --session lyp-r47c"
AUTH="$PWD/$DATA/admin-auth-init.js"
$AB close >/dev/null 2>&1 || true
$AB open --init-script "$AUTH" "http://127.0.0.1:5191/laiyipao-admin/#/admin/battles" >/dev/null 2>&1
sleep 6
$AB set viewport 1280 720 >/dev/null 2>&1
cat <<'EOF' | $AB eval --stdin > "$DATA/mark2.txt" 2>&1
(function(){var rows=[].slice.call(document.querySelectorAll('tbody tr'));var n=0;for(var i=0;i<rows.length;i++){if((rows[i].innerText||'').indexOf('477')<0)continue;var bs=[].slice.call(rows[i].querySelectorAll('button'));for(var j=0;j<bs.length;j++){if((bs[j].innerText||'').trim().indexOf('详情')>=0){bs[j].id='r47-detail-477';n++;}}}return 'marked='+n;})()
EOF
$AB click "#r47-detail-477" >/dev/null 2>&1
sleep 3
$AB screenshot "$SHOTS/r47-detail-open.png" >/dev/null 2>&1
$AB eval --stdin < "$D/probe2.js" > "$DATA/probe2.json" 2>&1
$AB close >/dev/null 2>&1 || true
echo R47_FOLLOWUP_DONE=YES
