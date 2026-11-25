#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT" || exit 1
AUD=docs/laiyipao/audits/round44-r22-cta-honesty
P="$AUD/data"; X="$AUD/screenshots"
AB=$(command -v agent-browser)
S=lyp-r44-final
mkdir -p "$P" "$X"
bash docs/laiyipao/audits/round15-reaction-timeline/start-api.sh > "$P/final-srv-api.log" 2>&1 &
API_PID=$!
(cd laiyipao/miniapp && pnpm run dev:h5 -- --host 127.0.0.1 --port 5174) > "$P/final-srv-h5.log" 2>&1 &
H5_PID=$!
for i in $(seq 1 80); do
  ac=$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:8080/api/v1/config || echo 000)
  hc=$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:5174/ || echo 000)
  if [ "$ac" = 200 ] && [ "$hc" = 200 ]; then echo "SERVERS_READY api=$ac h5=$hc"; break; fi
  sleep 1
done
bash "$AUD/gen-alias-auth-init.sh"
"$AB" --session "$S" --init-script "$P/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" > "$P/final-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 > "$P/final-viewport.txt" 2>&1
"$AB" --session "$S" wait --fn "() => !!document.querySelector('.r22-ai-cta')" --timeout 90000 > "$P/final-wait.txt" 2>&1 || true
"$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/final-cta-before.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-final-cta-before.png" > "$P/final-cta-before-shot.txt" 2>&1
# 真实低层鼠标坐标点击 r22 CTA 文案区域（x≈196, y≈783）
{ echo MOVE=196,783; "$AB" --session "$S" mouse move 196 783; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/final-cta-click.txt" 2>&1
sleep 1
"$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/final-cta-after.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-final-cta-after.png" > "$P/final-cta-after-shot.txt" 2>&1
# 正对照：真实点击第一张手牌卡牌主体，观察选卡面板退出（takeCard 生效）
{ echo MOVE=70,520; "$AB" --session "$S" mouse move 70 520; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/final-card-click.txt" 2>&1
sleep 1
"$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/final-card-after.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-final-card-after.png" > "$P/final-card-after-shot.txt" 2>&1
"$AB" --session "$S" errors > "$P/final-errors.txt" 2>&1 || true
node "$P/summary.js" "$P" final-cta-before.json final-cta-after.json final-card-after.json 2>&1 || true
echo FINAL_DRIVER_DONE
kill "$API_PID" "$H5_PID" 2>/dev/null || true
wait "$API_PID" 2>/dev/null || true
wait "$H5_PID" 2>/dev/null || true
echo SERVERS_STOPPED
