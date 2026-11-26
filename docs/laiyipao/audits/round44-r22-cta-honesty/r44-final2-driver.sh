#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT" || exit 1
AUD=docs/laiyipao/audits/round44-r22-cta-honesty
P="$AUD/data"; X="$AUD/screenshots"
AB=$(command -v agent-browser)
S=lyp-r44-final2
mkdir -p "$P" "$X"
# API 8080 已由现有进程提供，只启动空闲端口的 H5。
(cd laiyipao/miniapp && VITE_API_BASE=http://127.0.0.1:8080/api/v1 pnpm run dev:h5 -- --host 127.0.0.1 --port 5180 --strictPort) > "$P/final2-srv-h5.log" 2>&1 &
H5_PID=$!
for i in $(seq 1 80); do
  ac=$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:8080/api/v1/config || echo 000)
  hc=$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:5180/ || echo 000)
  if [ "$ac" = 200 ] && [ "$hc" = 200 ]; then echo "SERVERS_READY api=$ac h5=$hc"; break; fi
  sleep 1
done
bash "$AUD/gen-alias-auth-init.sh"
"$AB" --session "$S" --init-script "$P/auth-init.js" open "http://127.0.0.1:5180/#/pages/battle/battle?level=5" > "$P/final2-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 > "$P/final2-viewport.txt" 2>&1
"$AB" --session "$S" wait --fn "() => !!document.querySelector('.r22-ai-cta')" --timeout 90000 > "$P/final2-wait.txt" 2>&1 || true
"$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/final2-cta-before.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-final2-cta-before.png" > "$P/final2-cta-before-shot.txt" 2>&1
{ echo MOVE=196,783; "$AB" --session "$S" mouse move 196 783; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/final2-cta-click.txt" 2>&1
sleep 1
"$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/final2-cta-after.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-final2-cta-after.png" > "$P/final2-cta-after-shot.txt" 2>&1
{ echo MOVE=70,520; "$AB" --session "$S" mouse move 70 520; "$AB" --session "$S" mouse down left; "$AB" --session "$S" mouse up left; } > "$P/final2-card-click.txt" 2>&1
sleep 1
"$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/final2-card-after.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-final2-card-after.png" > "$P/final2-card-after-shot.txt" 2>&1
"$AB" --session "$S" errors > "$P/final2-errors.txt" 2>&1 || true
node "$P/summary.js" "$P" final2-cta-before.json final2-cta-after.json final2-card-after.json 2>&1 || true
echo FINAL2_DRIVER_DONE
kill "$H5_PID" 2>/dev/null || true
wait "$H5_PID" 2>/dev/null || true
echo SERVER_STOPPED
