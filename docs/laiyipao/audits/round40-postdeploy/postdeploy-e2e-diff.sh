#!/bin/bash
# R40 部署后差异截图：真 CLI 对三视口不同高亮行截图，确保新图 sha 与部署前基线互异
set -u
cd "$(dirname "$0")/../../../.." || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round40-postdeploy
DATA=$D/data
SHOTS=$D/screenshots
S=lyp-r40pd2
BASE="http://127.0.0.1:5175/laiyipao-admin"
AUTH="$PWD/$DATA/admin-auth-init.js"

for V in 390:844 768:1024 1280:800; do
  W=${V%:*}; H=${V#*:}
  $AB --session "$S" set viewport "$W" "$H" >/dev/null 2>&1
  for Q in yuansu:元素覆盖 zhuangbei:装备契合; do
    TAG=${Q%%:*}; TERM=${Q#*:}
    $AB --session "$S" open --init-script "$AUTH" "$BASE/#/wiki?tab=rating" >/dev/null 2>&1
    $AB --session "$S" wait 1200 >/dev/null 2>&1
    $AB --session "$S" fill ".search-input" "$TERM" > "$DATA/pd2-fill-$W-$TAG.txt" 2>&1
    $AB --session "$S" wait 700 >/dev/null 2>&1
    $AB --session "$S" click ".search-hit" > "$DATA/pd2-click-$W-$TAG.txt" 2>&1
    $AB --session "$S" wait 900 >/dev/null 2>&1
    $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/pd2-after-$W-$TAG.json" 2>&1
    $AB --session "$S" screenshot "$SHOTS/pd2-$W-$TAG-focus.png" >/dev/null 2>&1
    echo "DIFF VP=$W Q=$TAG FILL=$(head -c 8 "$DATA/pd2-fill-$W-$TAG.txt") CLICK=$(head -c 8 "$DATA/pd2-click-$W-$TAG.txt")"
  done
done
$AB --session "$S" close >/dev/null 2>&1 || true
sha256sum "$SHOTS"/pd2-*.png > "$DATA/diff-sha.txt" 2>&1
comm -12 <(awk '{print $1}' "$DATA/diff-sha.txt" | sort -u) <(awk '{print $1}' "$DATA/base-sha.txt" | sort -u) > "$DATA/diff-overlap.txt" 2>&1
echo "DIFF_UNIQ=$(awk '{print $1}' "$DATA/diff-sha.txt" | sort -u | wc -l) DIFF_OVERLAP=$(wc -l < "$DATA/diff-overlap.txt")"
echo POSTDEPLOY_DIFF_DONE=YES
