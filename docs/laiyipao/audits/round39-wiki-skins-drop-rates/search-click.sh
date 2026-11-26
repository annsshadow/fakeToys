#!/bin/bash
# R39: wiki search hits for skins (name and code) with row-focus highlight.
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r39-search
D=docs/laiyipao/audits/round39-wiki-skins-drop-rates
E="$PWD/$D"
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skins"
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
run_search() {
  KEY=$1; Q=$2
  "$AB" --session "$S" click ".search-input" >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" press Control+a >/dev/null 2>&1
  "$AB" --session "$S" press Backspace >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" type ".search-input" "$Q" > "$E/data/search-type-$KEY.txt" 2>&1
  echo "TYPE_${KEY}_RC=$?"; sleep 2
  "$AB" --session "$S" eval --stdin < "$E/search-probe.js" > "$E/data/search-$KEY.json" 2>&1
  "$AB" --session "$S" screenshot "$E/screenshots/r39-06-search-$KEY.png" >/dev/null 2>&1
  echo "=== SEARCH $KEY $Q ==="; head -c 420 "$E/data/search-$KEY.json"; echo
}
run_search youliezhe "游猎者"
run_search skin_chalk "skin_chalk"
echo SEARCH_DONE=YES
