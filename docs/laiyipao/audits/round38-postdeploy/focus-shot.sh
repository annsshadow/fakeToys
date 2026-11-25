#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser
E="$PWD/docs/laiyipao/audits/round38-postdeploy"
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills&anchor=composite"
for VP in 390 768 1280; do
  case $VP in 390) W=390;H=844;; 768) W=768;H=1024;; 1280) W=1280;H=800;; esac
  S=lyp-r38-focus-$VP
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport "$W" "$H" >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" set viewport "$W" "$H" >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" scrollintoview ".r38-recipe-mechanics" >/dev/null 2>&1; sleep 2
  "$AB" --session "$S" screenshot "$E/screenshots/r38-focus-$VP.png" >/dev/null 2>&1
  echo "FOCUS_${VP}_DONE"
done
S=lyp-r38-focus-1280
"$AB" --session "$S" click ".search-input" >/dev/null 2>&1; sleep 1
"$AB" --session "$S" type ".search-input" "焦土" >/dev/null 2>&1; sleep 2
"$AB" --session "$S" eval --stdin < "$E/detail-search.js" > "$E/data/detail-search-jiaotu.json" 2>&1
echo DETAIL_DONE=YES
