#!/bin/bash
# R40: rating_weights probe at 390x844 / 768x1024 / 1280x800 via agent-browser CLI.
cd "$(dirname "$0")/../../../.." || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round40-wiki-rating-weights
E="$PWD/$D"
run_viewport() {
  VP=$1; W=$2; H=$3; S=lyp-r40-$VP
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport "$W" "$H" >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=rating" >/dev/null 2>&1
  sleep 5
  "$AB" --session "$S" set viewport "$W" "$H" >/dev/null 2>&1
  "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" eval --stdin < "$E/rating-probe.js" > "$E/data/probe-$VP.json" 2>&1
  "$AB" --session "$S" screenshot "$E/screenshots/r40-$VP-rating.png" >/dev/null 2>&1
  echo "=== VIEWPORT $VP ==="; head -c 260 "$E/data/probe-$VP.json"; echo
}
run_viewport 390 390 844
run_viewport 768 768 1024
run_viewport 1280 1280 800
echo E2E_DONE=YES
