#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT" || exit 1
AUD=docs/laiyipao/audits/round44-r22-cta-honesty
P="$AUD/data"; X="$AUD/screenshots"
AB=$(command -v agent-browser)
S=lyp-r44-click3
PORT=5199
mkdir -p "$P" "$X"
bash docs/laiyipao/audits/round15-reaction-timeline/start-api.sh > "$P/click3-srv-api.log" 2>&1 &
API_PID=$!
(cd laiyipao/miniapp && VITE_API_BASE=http://127.0.0.1:8080/api/v1 pnpm run dev:h5 -- --host 127.0.0.1 --port $PORT --strictPort) > "$P/click3-srv-h5.log" 2>&1 &
H5_PID=$!
for i in $(seq 1 90); do
  ac=$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:8080/api/v1/config || echo 000)
  hc=$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:$PORT/ || echo 000)
  if [ "$ac" = 200 ] && [ "$hc" = 200 ]; then echo "SERVERS_READY api=$ac h5=$hc port=$PORT"; break; fi
  sleep 1
done
bash "$AUD/gen-alias-auth-init.sh"
"$AB" --session "$S" --init-script "$P/auth-init.js" open "http://127.0.0.1:$PORT/#/pages/battle/battle?level=5" > "$P/click3-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 > "$P/click3-viewport.txt" 2>&1
"$AB" --session "$S" wait --fn "() => !!document.querySelector('.r22-ai-cta')" --timeout 90000 > "$P/click3-wait.txt" 2>&1 || true
"$AB" --session "$S" eval "$(cat "$P/dialog-probe.js")" > "$P/click3-before.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-click3-before.png" > "$P/click3-before-shot.txt" 2>&1
read -r CX CY < <(node -e 'const fs=require("fs");let v=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));if(typeof v==="string")v=JSON.parse(v);const r=v.r22CtaRect;if(!r){console.log(-1,-1)}else{console.log(Math.round(r.x+r.w/2),Math.round(r.y+r.h/2))}' "$P/click3-before.json")
echo "CTA_MOVE=$CX,$CY"
if [ "$CX" -ge 0 ]; then
  { echo "MOVE=$CX,$CY"; "$AB" --session "$S" mouse move "$CX" "$CY"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/click3-cta-click.txt" 2>&1
  sleep 1
  "$AB" --session "$S" eval "$(cat "$P/dialog-probe.js")" > "$P/click3-cta-after.json" 2>&1
  "$AB" --session "$S" screenshot "$X/r44-click3-cta-after.png" > "$P/click3-cta-after-shot.txt" 2>&1
fi
read -r HX HY < <(node -e 'const fs=require("fs");let v=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));if(typeof v==="string")v=JSON.parse(v);const h=v.handCards&&v.handCards[0];if(!h){console.log(-1,-1)}else{const r=h.rect;console.log(Math.round(r.x+r.w/2),Math.round(r.y+r.h/2))}' "$P/click3-before.json")
echo "CARD_MOVE=$HX,$HY"
if [ "$HX" -ge 0 ]; then
  { echo "MOVE=$HX,$HY"; "$AB" --session "$S" mouse move "$HX" "$HY"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/click3-card-click.txt" 2>&1
  sleep 2
  "$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/click3-card-after.json" 2>&1
  "$AB" --session "$S" screenshot "$X/r44-click3-card-after.png" > "$P/click3-card-after-shot.txt" 2>&1
fi
"$AB" --session "$S" eval "$(cat "$P/r43-probe.js")" > "$P/click3-r43probe.json" 2>&1
for k in r27 r24; do
  read -r RX RY < <(node -e 'const fs=require("fs");let v=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));if(typeof v==="string")v=JSON.parse(v);const r=v[process.argv[2]+"Rect"];if(!r){console.log(-1,-1)}else{console.log(Math.round(r.x+r.w/2),Math.round(r.y+r.h/2))}' "$P/click3-r43probe.json" "$k")
  if [ "$RX" -ge 0 ]; then
    { echo "MOVE=$RX,$RY"; "$AB" --session "$S" mouse move "$RX" "$RY"; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/click3-$k-click.txt" 2>&1
    sleep 1
    "$AB" --session "$S" eval "$(cat "$P/r43-probe.js")" > "$P/click3-$k-after.json" 2>&1
  fi
done
"$AB" --session "$S" errors > "$P/click3-errors.txt" 2>&1 || true
node "$P/summary.js" "$P" click3-before.json click3-cta-after.json click3-card-after.json 2>&1 || true
echo CLICK3_DONE
kill "$API_PID" "$H5_PID" 2>/dev/null || true
wait "$API_PID" 2>/dev/null || true
wait "$H5_PID" 2>/dev/null || true
echo SERVERS_STOPPED
