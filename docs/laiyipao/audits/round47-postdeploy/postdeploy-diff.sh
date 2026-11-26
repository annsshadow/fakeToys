#!/bin/bash
set -u
ROOT=$(git rev-parse --show-toplevel) || exit 1
cd "$ROOT" || exit 1
D=docs/laiyipao/audits/round47-postdeploy
QA=docs/laiyipao/audits/round47-r16-verify-cta
DATA=$D/data
SHOTS=$D/screenshots
S=lyp-r47pd2
AB="agent-browser --session $S"
BASE=http://127.0.0.1:5191/laiyipao-admin
AUTH="$PWD/$DATA/admin-auth-init.js"
$AB close >/dev/null 2>&1 || true
$AB open --init-script "$AUTH" "$BASE/#/admin/battles" > "$DATA/diff-open.txt" 2>&1
sleep 6
$AB set viewport 1366 768 >/dev/null 2>&1
sleep 1
$AB screenshot "$SHOTS/pd2-list-1366.png" >/dev/null 2>&1
cat <<'EOF' | $AB eval --stdin > "$DATA/diff-mark.txt" 2>&1
(function(){var rows=[].slice.call(document.querySelectorAll('tbody tr'));var n=0;for(var i=0;i<rows.length;i++){if((rows[i].innerText||'').indexOf('477')<0)continue;var bs=[].slice.call(rows[i].querySelectorAll('button'));for(var j=0;j<bs.length;j++){if((bs[j].innerText||'').trim().indexOf('详情')>=0){bs[j].id='r47-detail-477';n++;}}}return 'marked='+n;})()
EOF
$AB click "#r47-detail-477" > "$DATA/diff-click.txt" 2>&1
sleep 3
$AB screenshot "$SHOTS/pd2-detail-1366.png" >/dev/null 2>&1
$AB eval --stdin < "$QA/detail-probe.js" > "$DATA/diff-before.json" 2>&1
$AB screenshot "$SHOTS/pd2-cta-before-1366.png" >/dev/null 2>&1
CX=$(python "$D/num.py" "$DATA/diff-before.json" ctaRect.cx)
CY=$(python "$D/num.py" "$DATA/diff-before.json" ctaRect.cy)
echo "DIFF_MOVE=$CX,$CY"
$AB mouse move "$CX" "$CY" > "$DATA/diff-move.txt" 2>&1
$AB mouse down left > "$DATA/diff-down.txt" 2>&1
$AB mouse up left > "$DATA/diff-up.txt" 2>&1
sleep 2
$AB eval --stdin < "$QA/detail-probe.js" > "$DATA/diff-after.json" 2>&1
$AB screenshot "$SHOTS/pd2-cta-after-1366.png" >/dev/null 2>&1
$AB close >/dev/null 2>&1 || true
echo POSTDEPLOY_DIFF_DONE=YES