#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel); cd "$ROOT" || exit 1
D=docs/laiyipao/audits/round44-postdeploy
P="$D/data"; X="$D/screenshots"; AB=$(command -v agent-browser); S=lyp-r44-pd2
export NO_PROXY="127.0.0.1,localhost"; export no_proxy="127.0.0.1,localhost"
mkdir -p "$P" "$X"
# 轮询等待到 card_select（r22 CTA 出现），最多约 120s；当前 session 多已就绪
READY=0
for i in $(seq 1 40); do
  "$AB" --session "$S" eval "$(cat "$P/dialog-probe.js")" > "$P/pd2-check.json" 2>&1
  R=$(node -e 'const fs=require("fs");let v=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));if(typeof v==="string")v=JSON.parse(v);console.log(v.hasR22cta?1:0)' "$P/pd2-check.json" 2>/dev/null || echo 0)
  if [ "$R" = "1" ]; then READY=1; break; fi
  sleep 2
done
echo "CARD_SELECT_READY=$READY"
cp "$P/pd2-check.json" "$P/pd2-before.json"
"$AB" --session "$S" screenshot "$X/pd2-r22-before.png" > "$P/pd2-before-shot.txt" 2>&1
read -r CX CY < <(node -e 'const fs=require("fs");let v=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));if(typeof v==="string")v=JSON.parse(v);const r=v.r22CtaRect;if(!r){console.log(-1,-1)}else{console.log(Math.round(r.x+r.w/2),Math.round(r.y+r.h/2))}' "$P/pd2-before.json")
echo "CTA_MOVE=$CX,$CY"
if [ "$CX" -ge 0 ]; then
  { echo "MOVE=$CX,$CY"; "$AB" --session "$S" mouse move "$CX" "$CY"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/pd2-cta-click.txt" 2>&1
  sleep 1
  "$AB" --session "$S" eval "$(cat "$P/dialog-probe.js")" > "$P/pd2-cta-after.json" 2>&1
  "$AB" --session "$S" screenshot "$X/pd2-r22-cta-after.png" > "$P/pd2-cta-after-shot.txt" 2>&1
else
  cp "$P/pd2-before.json" "$P/pd2-cta-after.json"
fi
read -r HX HY < <(node -e 'const fs=require("fs");let v=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));if(typeof v==="string")v=JSON.parse(v);const h=v.handCards&&v.handCards[0];if(!h){console.log(-1,-1)}else{const r=h.rect;console.log(Math.round(r.x+r.w/2),Math.round(r.y+r.h/2))}' "$P/pd2-before.json")
echo "CARD_MOVE=$HX,$HY"
if [ "$HX" -ge 0 ]; then
  { echo "MOVE=$HX,$HY"; "$AB" --session "$S" mouse move "$HX" "$HY"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/pd2-card-click.txt" 2>&1
  sleep 2
  "$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/pd2-card-after.json" 2>&1
  "$AB" --session "$S" screenshot "$X/pd2-card-after.png" > "$P/pd2-card-after-shot.txt" 2>&1
fi
"$AB" --session "$S" errors > "$P/pd2-errors.txt" 2>&1 || true
echo "=== SUMMARY ==="
node "$P/summary.js" "$P" pd2-before.json pd2-cta-after.json pd2-card-after.json 2>&1 || true
echo "=== SHAS ==="
for f in "$X"/pd2-*.png; do echo "$(sha256sum "$f" | cut -d' ' -f1) $(basename "$f")"; done
echo PD2_DRIVER_DONE
