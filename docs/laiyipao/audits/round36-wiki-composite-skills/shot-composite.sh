#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser
D="docs/laiyipao/audits/round36-wiki-composite-skills"
E="$PWD/$D"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills&anchor=composite'
for VP in 390 768 1280; do
  case $VP in 390) H=844;; 768) H=1024;; 1280) H=800;; esac
  S=lyp-r36-$VP
  "$AB" --session "$S" open "$URL" >/dev/null 2>&1; sleep 3
  "$AB" --session "$S" set viewport "$VP" "$H" >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 5
  "$AB" --session "$S" eval --stdin < "$E/scroll-composite.js" > "$E/data/scroll-$VP.json" 2>&1
  sleep 1
  "$AB" --session "$S" screenshot "$E/screenshots/r36-$VP-composite.png" >/dev/null 2>&1
  echo "=== SHOT $VP ==="; cat "$E/data/scroll-$VP.json"; echo
done
echo SHOTS_DONE=YES
