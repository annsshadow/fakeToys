#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round48-r12-growth-cta"
S="lyp-r48-bag"
AB="agent-browser --session $S"
BASE="http://127.0.0.1:5201/#/pages/bag/bag"
run(){ "$@" >"$D/data/$1.log" 2>&1; }
echo DRIVER_START
$AB close >/dev/null 2>&1 || true
$AB open --init-script "$D/data/init-element.js" "$BASE" > "$D/data/open.txt" 2>&1
echo OPEN_URL=$($AB get url 2>/dev/null || true)
$AB set viewport 390 844 > "$D/data/viewport.txt" 2>&1
$AB wait '.r12-priority-cta' > "$D/data/wait.txt" 2>&1
sleep 2
cat "$D/r48-probe.js" | $AB eval --stdin > "$D/data/bag-before.json" 2>&1
$AB screenshot "$D/screenshots/r48-bag-before.png" > "$D/data/ss-before.txt" 2>&1
python3 - <<'PY' 2>/dev/null || true
PY
echo BAG_BEFORE_DONE
echo DRIVER_END
