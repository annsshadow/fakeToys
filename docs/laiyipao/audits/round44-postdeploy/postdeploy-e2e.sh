#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel); cd "$ROOT" || exit 1
SRC=docs/laiyipao/audits/round44-r22-cta-honesty
D=docs/laiyipao/audits/round44-postdeploy
P="$D/data"; X="$D/screenshots"; AB=$(command -v agent-browser); S=lyp-r44-pd2
export NO_PROXY="127.0.0.1,localhost"; export no_proxy="127.0.0.1,localhost"
mkdir -p "$P" "$X"
cp "$SRC/data/dialog-probe.js" "$P/dialog-probe.js"
cp "$SRC/data/card-state.js" "$P/card-state.js"
cp "$SRC/data/summary.js" "$P/summary.js"
cp "$SRC/gen-alias-auth-init.sh" "$D/gen-alias-auth-init.sh"
cp "$SRC/gen-alias-init.js" "$D/gen-alias-init.js"
echo "PRE_API=$(curl --noproxy '*' -s -o /dev/null -w '%{http_code}' --max-time 3 http://127.0.0.1:8080/api/v1/config)"
echo "PRE_H5=$(curl --noproxy '*' -s -o /dev/null -w '%{http_code}' --max-time 3 http://127.0.0.1:5274/)"
bash "$SRC/gen-alias-auth-init.sh" || { echo AUTH_FAILED; exit 1; }
cp "$SRC/data/auth-init.js" "$P/auth-init.js"
test -f "$P/auth-init.js" || { echo AUTH_INIT_MISSING; exit 1; }
"$AB" --session "$S" --init-script "$P/auth-init.js" open "http://127.0.0.1:5274/#/pages/battle/battle?level=5" > "$P/pd-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 > "$P/pd-viewport.txt" 2>&1
"$AB" --session "$S" wait --fn "() => !!document.querySelector('.r22-ai-cta')" --timeout 90000 > "$P/pd-wait.txt" 2>&1 || true
"$AB" --session "$S" eval "$(cat "$P/dialog-probe.js")" > "$P/pd-before.json" 2>&1
"$AB" --session "$S" screenshot "$X/pd-r22-before.png" > "$P/pd-before-shot.txt" 2>&1
read -r CX CY < <(node -e 'const fs=require("fs");let v=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));if(typeof v==="string")v=JSON.parse(v);const r=v.r22CtaRect;if(!r){console.log(-1,-1)}else{console.log(Math.round(r.x+r.w/2),Math.round(r.y+r.h/2))}' "$P/pd-before.json")
echo "CTA_MOVE=$CX,$CY"
if [ "$CX" -ge 0 ]; then
  { echo "MOVE=$CX,$CY"; "$AB" --session "$S" mouse move "$CX" "$CY"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/pd-cta-click.txt" 2>&1
  sleep 1
  "$AB" --session "$S" eval "$(cat "$P/dialog-probe.js")" > "$P/pd-cta-after.json" 2>&1
  "$AB" --session "$S" screenshot "$X/pd-r22-cta-after.png" > "$P/pd-cta-after-shot.txt" 2>&1
fi
read -r HX HY < <(node -e 'const fs=require("fs");let v=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));if(typeof v==="string")v=JSON.parse(v);const h=v.handCards&&v.handCards[0];if(!h){console.log(-1,-1)}else{const r=h.rect;console.log(Math.round(r.x+r.w/2),Math.round(r.y+r.h/2))}' "$P/pd-before.json")
echo "CARD_MOVE=$HX,$HY"
if [ "$HX" -ge 0 ]; then
  { echo "MOVE=$HX,$HY"; "$AB" --session "$S" mouse move "$HX" "$HY"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/pd-card-click.txt" 2>&1
  sleep 2
  "$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/pd-card-after.json" 2>&1
  "$AB" --session "$S" screenshot "$X/pd-card-after.png" > "$P/pd-card-after-shot.txt" 2>&1
fi
"$AB" --session "$S" errors > "$P/pd-errors.txt" 2>&1 || true
echo "=== SUMMARY ==="
node "$P/summary.js" "$P" pd-before.json pd-cta-after.json pd-card-after.json 2>&1 || true
echo "=== SHAS ==="
for f in "$X"/pd-*.png; do echo "$(sha256sum "$f" | cut -d' ' -f1) $(basename "$f")"; done
echo PD_DRIVER_DONE