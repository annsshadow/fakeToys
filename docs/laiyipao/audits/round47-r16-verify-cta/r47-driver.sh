#!/bin/bash
set -u
ROOT=$(git rev-parse --show-toplevel) || exit 1
cd "$ROOT" || exit 1
D=docs/laiyipao/audits/round47-r16-verify-cta
DATA=$D/data
SHOTS=$D/screenshots
S=lyp-r47b
AB="agent-browser --session $S"
BASE="http://127.0.0.1:5191/laiyipao-admin"
AUTH="$PWD/$DATA/admin-auth-init.js"
$AB close >/dev/null 2>&1 || true
$AB open --init-script "$AUTH" "$BASE/#/admin/battles" > "$DATA/open2.txt" 2>&1
sleep 6
$AB set viewport 1280 720 >/dev/null 2>&1
sleep 1
echo "OPEN_URL=$($AB get url 2>&1)"
$AB eval --stdin < "$D/r47-probe.js" > "$DATA/list-probe.json" 2>&1
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
$AB eval --stdin < "$D/r47-probe.js" > "$DATA/cta-before.json" 2>&1
$AB screenshot "$SHOTS/r47-cta-before.png" >/dev/null 2>&1
CX=$(python "$D/num.py" "$DATA/cta-before.json" cx 2>>"$DATA/num-err.txt")
CY=$(python "$D/num.py" "$DATA/cta-before.json" cy 2>>"$DATA/num-err.txt")
echo "CTA_MOVE=$CX,$CY"
$AB mouse move "$CX" "$CY" > "$DATA/cta-move.txt" 2>&1
$AB mouse down left > "$DATA/cta-down.txt" 2>&1
$AB mouse up left > "$DATA/cta-up.txt" 2>&1
sleep 2
$AB eval --stdin < "$D/r47-probe.js" > "$DATA/cta-after.json" 2>&1
$AB screenshot "$SHOTS/r47-cta-after.png" >/dev/null 2>&1
python "$D/r47-summary.py" > "$DATA/summary.txt" 2>&1
$AB close >/dev/null 2>&1 || true
echo R47_DRIVER_DONE=YES
