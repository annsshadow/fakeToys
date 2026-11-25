#!/usr/bin/env bash
set -uo pipefail
ROOT=$(git rev-parse --show-toplevel); cd "$ROOT" || exit 1
AUD=docs/laiyipao/audits/round44-r22-cta-honesty; P="$AUD/data"; X="$AUD/screenshots"; AB=$(command -v agent-browser); S=lyp-r44-baseline
mkdir -p "$X"
printf 'pre_api='; curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:8080/api/v1/config || true; echo
printf 'pre_h5='; curl -s -o /dev/null -w '%{http_code}' --max-time 2 http://127.0.0.1:5174/ || true; echo
bash "$AUD/gen-alias-auth-init.sh" || { echo AUTH_FAILED; exit 1; }
"$AB" --session "$S" --init-script "$P/auth-init.js" open "http://127.0.0.1:5174/#/pages/battle/battle?level=5" > "$P/base-open.txt" 2>&1
"$AB" --session "$S" set viewport 390 844 > "$P/base-viewport.txt" 2>&1
sleep 2
for t in 1 2 3 4 5 6; do
  "$AB" --session "$S" eval "$(cat "$P/fast-state.js")" > "$P/base-$t.json" 2>&1
  echo "base-$t $(node -e 'const fs=require("fs");let v=JSON.parse(fs.readFileSync(process.argv[1],"utf8"));if(typeof v==="string")v=JSON.parse(v);console.log(JSON.stringify({clock:v.clock,onCardSelect:v.onCardSelect,r22:v.r22cta,hasPause:v.hasPause,hasResume:v.hasResume}))' "$P/base-$t.json")"
  sleep 2
done
node "$P/summary.js" "$P" base-1.json base-2.json base-3.json base-4.json base-5.json base-6.json 2>&1 || true
echo BASE_DRIVER_DONE
