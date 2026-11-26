#!/bin/bash
# R40: type query via CLI, capture search panel; click first hit when present.
cd "$(dirname "$0")/../../../.." || exit 1
AB=agent-browser; S=lyp-r40-search
D=docs/laiyipao/audits/round40-wiki-rating-weights
E="$PWD/$D"
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=rating"
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
run_q() {
  KEY=$1; Q=$2
  "$AB" --session "$S" click ".search-input" >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" press Control+a >/dev/null 2>&1
  "$AB" --session "$S" press Backspace >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" type ".search-input" "$Q" >/dev/null 2>&1; sleep 2
  "$AB" --session "$S" eval --stdin < "$E/search-probe.js" > "$E/data/search-$KEY.json" 2>&1
  "$AB" --session "$S" screenshot "$E/screenshots/r40-06-search-$KEY.png" >/dev/null 2>&1
  "$AB" --session "$S" click ".search-hit" > "$E/data/search-click-$KEY.txt" 2>&1
  echo "CLICK_$KEY_RC=$?"; sleep 2
  "$AB" --session "$S" eval --stdin < "$E/search-probe.js" > "$E/data/searchpick-$KEY.json" 2>&1
  "$AB" --session "$S" screenshot "$E/screenshots/r40-07-searchpick-$KEY.png" >/dev/null 2>&1
  echo "=== SEARCH $KEY ==="; head -c 420 "$E/data/search-$KEY.json"; echo
  echo "=== SEARCHPICK $KEY ==="; head -c 420 "$E/data/searchpick-$KEY.json"; echo
}
run_q fanyinglian "反应链"
run_q mechanic_depth "mechanic_depth"
echo SEARCH_DONE=YES
