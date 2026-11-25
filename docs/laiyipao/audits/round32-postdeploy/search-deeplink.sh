#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r32post-nav
D=docs/laiyipao/audits/round32-postdeploy; E="$PWD/$D"; DATA="$E/data"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$DATA/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
echo '=== SEARCH ==='
"$AB" --session "$S" focus 'input[placeholder*="搜索关卡"]' > "$DATA/search-focus.txt" 2>&1; echo FOCUS_RC=$?
"$AB" --session "$S" keyboard type '温压弹' > "$DATA/search-type.txt" 2>&1; echo TYPE_RC=$?
"$AB" --session "$S" press Enter > "$DATA/search-enter.txt" 2>&1; echo ENTER_RC=$?
sleep 2
"$AB" --session "$S" eval --stdin < "$DATA/reg-search2.txt" > "$DATA/search-probe.json" 2>&1; cat "$DATA/search-probe.json"
"$AB" --session "$S" screenshot "$E/screenshots/r32post-search.png" >/dev/null 2>&1; echo SEARCH_SHOT_RC=$?
echo '=== DEEPLINK ==='
"$AB" --session "$S" --init-script "$DATA/admin-auth-init.js" open 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=enemies' >/dev/null 2>&1; sleep 6
"$AB" --session "$S" eval --stdin < "$DATA/deep6-probe.txt" > "$DATA/deep-probe.json" 2>&1; cat "$DATA/deep-probe.json"
"$AB" --session "$S" screenshot "$E/screenshots/r32post-deeplink.png" >/dev/null 2>&1; echo DEEP_SHOT_RC=$?
"$AB" --session "$S" close >/dev/null 2>&1 || true
sha256sum "$E/screenshots/r32post-search.png" "$E/screenshots/r32post-deeplink.png" 2>&1
echo SEARCH_DEEP_DONE=YES
