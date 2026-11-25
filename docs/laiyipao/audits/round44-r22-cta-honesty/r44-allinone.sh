#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT" || exit 1
AUD=docs/laiyipao/audits/round44-r22-cta-honesty
P="$AUD/data"
mkdir -p "$P"
echo "=== START API ==="
bash docs/laiyipao/audits/round15-reaction-timeline/start-api.sh >"$P/srv-api.log" 2>&1 &
API_PID=$!
echo "=== START H5 ==="
(cd laiyipao/miniapp && pnpm run dev:h5 -- --host 127.0.0.1 --port 5174) >"$P/srv-h5.log" 2>&1 &
H5_PID=$!
for i in $(seq 1 80); do
  ac=$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:8080/api/v1/config || echo 000)
  hc=$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:5174/oa4rust-h5/ || echo 000)
  if [ "$ac" = 200 ] && [ "$hc" = 200 ]; then echo "SERVERS_READY api=$ac h5=$hc"; break; fi
  sleep 1
done
echo "=== MINT AUTH ==="
bash "$AUD/gen-alias-auth-init.sh"
echo "=== BROWSER DRIVER ==="
bash "$AUD/r44-alias-driver.sh"
echo "=== SUMMARY ==="
node "$P/summary.js" "$P" alias-initial.json alias-card.json alias-card2.json
echo "=== STOP SERVERS ==="
kill "$API_PID" "$H5_PID" 2>/dev/null || true
wait "$API_PID" 2>/dev/null || true
wait "$H5_PID" 2>/dev/null || true
echo ALL_IN_ONE_DONE
