#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel); cd "$ROOT" || exit 1
AUD=docs/laiyipao/audits/round44-r22-cta-honesty; P="$AUD/data"; X="$AUD/screenshots"; AB=$(command -v agent-browser); S=lyp-r44-closed
mkdir -p "$X"
printf 'pre_api='; curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:8080/api/v1/config || true; echo
printf 'pre_h5='; curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:5174/ || true; echo
bash "$AUD/gen-alias-auth-init.sh" || { echo AUTH_FAILED; exit 1; }
"$AB" --session "$S" --init-script "$P/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" > "$P/closed-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 > "$P/closed-viewport.txt" 2>&1
sleep 2
"$AB" --session "$S" wait --fn "() => !!document.querySelector('.sheet') && document.querySelectorAll('.hand-card').length > 0" --timeout 90000 > "$P/closed-wait.txt" 2>&1 || true
"$AB" --session "$S" eval "$(cat "$P/dialog-probe.js")" > "$P/closed-before.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-closed-before.png" > "$P/closed-before-shot.txt" 2>&1
read -r CX CY < <(node -e 'const fs=require("fs");let v=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));if(typeof v==="string")v=JSON.parse(v);const r=v.r22CtaRect;if(!r){console.log(-1,-1)}else{console.log(Math.round(r.x+r.w/2),Math.round(r.y+r.h/2))}' "$P/closed-before.json")
echo "CTA_MOVE=$CX,$CY"
if [ "$CX" -ge 0 ]; then
  { echo "MOVE=$CX,$CY"; "$AB" --session "$S" mouse move "$CX" "$CY"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/closed-cta-click.txt" 2>&1
  sleep 1
  "$AB" --session "$S" eval "$(cat "$P/dialog-probe.js")" > "$P/closed-cta-after.json" 2>&1
  "$AB" --session "$S" screenshot "$X/r44-closed-cta-after.png" > "$P/closed-cta-after-shot.txt" 2>&1
  read -r HX HY < <(node -e 'const fs=require("fs");let v=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));if(typeof v==="string")v=JSON.parse(v);const h=v.handCards&&v.handCards[0];if(!h){console.log(-1,-1)}else{const r=h.rect;console.log(Math.round(r.x+r.w/2),Math.round(r.y+r.h/2))}' "$P/closed-cta-after.json")
  echo "CARD_MOVE=$HX,$HY"
  if [ "$HX" -ge 0 ]; then
    { echo "MOVE=$HX,$HY"; "$AB" --session "$S" mouse move "$HX" "$HY"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/closed-card-click.txt" 2>&1
    sleep 2
    "$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/closed-card-after.json" 2>&1
    "$AB" --session "$S" screenshot "$X/r44-closed-card-after.png" > "$P/closed-card-after-shot.txt" 2>&1
  fi
fi
"$AB" --session "$S" errors > "$P/closed-errors.txt" 2>&1 || true
node "$P/summary.js" "$P" closed-before.json closed-cta-after.json closed-card-after.json 2>&1 || true
echo CLOSED_DRIVER_DONE
