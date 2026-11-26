#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round38-wiki-recipes
E="$PWD/$D"
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills&anchor=composite"
run_viewport() {
  VP=$1; W=$2; H=$3; S=lyp-r38-$VP
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport "$W" "$H" >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 5
  "$AB" --session "$S" set viewport "$W" "$H" >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" eval --stdin < "$E/recipe-probe.js" > "$E/data/probe-$VP.json" 2>&1
  "$AB" --session "$S" screenshot "$E/screenshots/r38-$VP-recipes.png" >/dev/null 2>&1
  echo "=== $VP ==="; head -c 300 "$E/data/probe-$VP.json"; echo
}
run_viewport 390 390 844
run_viewport 768 768 1024
run_viewport 1280 1280 800
echo E2E_DONE=YES
