#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; E="$PWD/docs/laiyipao/audits/round37-postdeploy"
URL="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=mastery"
for VP in 390 768 1280; do
  case $VP in 390) H=844;; 768) H=1024;; 1280) H=800;; esac
  S=lyp-r37post-chk-$VP
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport "$VP" "$H" >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 5
  "$AB" --session "$S" set viewport "$VP" "$H" >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 5
  "$AB" --session "$S" eval --stdin < "$E/scroll-detail.js" > "$E/data/scroll-detail-$VP.json" 2>&1
  echo "=== $VP ==="; head -c 300 "$E/data/scroll-detail-$VP.json"; echo
done
echo DETAIL_DONE=YES