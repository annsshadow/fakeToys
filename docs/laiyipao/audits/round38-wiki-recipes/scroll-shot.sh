#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r38-scroll
E="$PWD/docs/laiyipao/audits/round38-wiki-recipes"
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills&anchor=composite"
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
"$AB" --session "$S" eval --stdin < "$E/scroll-detail.js" > "$E/data/scroll-390.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r38-07-scroll-390.png" >/dev/null 2>&1
echo SCROLL_SHOT_DONE=YES
