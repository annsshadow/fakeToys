#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; D=docs/laiyipao/audits/round35-wiki-gem-mechanics; E="$PWD/$D"; DATA="$E/data"
BASE="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=gems"
run_search() { W=$1; S="lyp-r35-search-$W"
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$BASE" >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" click 'input[placeholder*="搜索关卡"]' > "$DATA/search-$W-click.txt" 2>&1; echo CLICK_RC=$?
  "$AB" --session "$S" keyboard type "$W" > "$DATA/search-$W-type.txt" 2>&1; echo TYPE_RC=$?
  "$AB" --session "$S" press Enter > "$DATA/search-$W-enter.txt" 2>&1; echo ENTER_RC=$?
  sleep 2
  "$AB" --session "$S" eval --stdin < "$E/search-post.js" > "$DATA/search-$W.json" 2>&1
  "$AB" --session "$S" screenshot "$E/screenshots/r35-search-$W.png" >/dev/null 2>&1
  echo "=== SEARCH $W ==="; cat "$DATA/search-$W.json"
}
run_search 锋锐石
run_search gold
echo SEARCH_DONE=YES
