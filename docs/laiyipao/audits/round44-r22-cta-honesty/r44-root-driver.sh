#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT" || exit 1
AUD=docs/laiyipao/audits/round44-r22-cta-honesty
P="$AUD/data"; X="$AUD/screenshots"
AB=$(command -v agent-browser)
S=lyp-r44-root
mkdir -p "$P" "$X"
bash docs/laiyipao/audits/round15-reaction-timeline/start-api.sh > "$P/root-srv-api.log" 2>&1 &
API_PID=$!
(cd laiyipao/miniapp && pnpm run dev:h5 -- --host 127.0.0.1 --port 5174) > "$P/root-srv-h5.log" 2>&1 &
H5_PID=$!
for i in $(seq 1 80); do
  ac=$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:8080/api/v1/config || echo 000)
  hc=$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:5174/ || echo 000)
  if [ "$ac" = 200 ] && [ "$hc" = 200 ]; then echo "SERVERS_READY api=$ac h5=$hc"; break; fi
  sleep 1
done
bash "$AUD/gen-alias-auth-init.sh"
"$AB" --session "$S" --init-script "$P/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" > "$P/root-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 > "$P/root-viewport.txt" 2>&1
sleep 2
"$AB" --session "$S" eval "$(cat "$P/card-state.js")" > "$P/root-initial.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-root-initial.png" > "$P/root-initial-shot.txt" 2>&1
"$AB" --session "$S" wait --fn "() => !!document.querySelector('.sheet') && document.querySelectorAll('.hand-card').length > 0" --timeout 90000 > "$P/root-wait-card.txt" 2>&1 || true
"$AB" --session "$S" eval "$(cat "$P/dialog-probe.js")" > "$P/root-card.json" 2>&1
"$AB" --session "$S" screenshot "$X/r44-root-card.png" > "$P/root-card-shot.txt" 2>&1
node "$P/summary.js" "$P" root-initial.json root-card.json 2>&1 || true
echo ROOT_DRIVER_DONE
kill "$API_PID" "$H5_PID" 2>/dev/null || true
wait "$API_PID" 2>/dev/null || true
wait "$H5_PID" 2>/dev/null || true
echo SERVERS_STOPPED
