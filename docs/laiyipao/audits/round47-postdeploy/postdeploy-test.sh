#!/bin/bash
set -u
ROOT=$(git rev-parse --show-toplevel) || exit 1
cd "$ROOT" || exit 1
D=docs/laiyipao/audits/round47-postdeploy
DATA=$D/data
SHOTS=$D/screenshots
S=lyp-r47pd
AB="agent-browser --session $S"
BASE=http://127.0.0.1:5191/laiyipao-admin
AUTH="$PWD/$DATA/admin-auth-init.js"
$AB close >/dev/null 2>&1 || true
$AB open --init-script "$AUTH" "$BASE/#/admin/battles" > "$DATA/open.txt" 2>&1
sleep 6
$AB set viewport 1280 720 >/dev/null 2>&1
sleep 1
echo "OPEN_URL=$($AB get url 2>&1)"
cat <<'EOF' | $AB eval --stdin > "$DATA/mark.txt" 2>&1
(function(){var rows=[].slice.call(document.querySelectorAll('tbody tr'));var n=0;for(var i=0;i<rows.length;i++){if((rows[i].innerText||'').indexOf('477')<0)continue;var bs=[].slice.call(rows[i].querySelectorAll('button'));for(var j=0;j<bs.length;j++){if((bs[j].innerText||'').trim().indexOf('详情')>=0){bs[j].id='r47-detail-477';n++;}}}return 'marked='+n;})()
EOF
echo "MARK=$(cat "$DATA/mark.txt")"
$AB click "#r47-detail-477" > "$DATA/detail-click.txt" 2>&1
sleep 3
cat <<'EOF' | $AB eval --stdin > "$DATA/cta-scroll.txt" 2>&1
(function(){var c=document.querySelector('.r16-timeline-cta');if(!c)return 'no-cta';c.scrollIntoView({block:'center'});return 'scrolled';})()
EOF
sleep 1
$AB eval --stdin < docs/laiyipao/audits/round47-r16-verify-cta/detail-probe.js > "$DATA/cta-before.json" 2>&1
$AB screenshot "$SHOTS/pd-cta-before.png" >/dev/null 2>&1
echo "BEFORE=$(cat "$DATA/cta-before.json" | head -c 400)"
$AB close >/dev/null 2>&1 || true
echo POSTDEPLOY_TEST_DONE=YES