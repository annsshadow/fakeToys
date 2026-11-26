#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r37post-last
E="$PWD/docs/laiyipao/audits/round37-postdeploy"
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=mastery" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
"$AB" --session "$S" eval --stdin < "$E/scroll-detail.js" > "$E/data/lastcol-1280.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r37post-06-lastcol-1280.png" >/dev/null 2>&1
echo LASTCOL_DONE=YES