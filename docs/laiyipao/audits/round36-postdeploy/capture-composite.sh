#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; E="$PWD/docs/laiyipao/audits/round36-postdeploy"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills&anchor=composite'
for VP in 390 768 1280; do
  case $VP in 390) H=844;; 768) H=1024;; 1280) H=800;; esac
  S=lyp-r36post-sec-$VP
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport "$VP" "$H" >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" set viewport "$VP" "$H" >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" eval --stdin < "$E/scroll-composite.js" > "$E/data/scroll-composite-$VP.json" 2>&1
  sleep 1
  "$AB" --session "$S" screenshot "$E/screenshots/r36post-fixed-$VP-composite.png" >/dev/null 2>&1
  echo "=== SEC $VP ==="; head -c 300 "$E/data/scroll-composite-$VP.json"; echo
  "$AB" --session "$S" close >/dev/null 2>&1 || true
done
echo SEC_DONE=YES
