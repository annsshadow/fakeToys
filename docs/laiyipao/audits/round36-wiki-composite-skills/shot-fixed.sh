#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; E="$PWD/docs/laiyipao/audits/round36-wiki-composite-skills"
for VP in 768 1280; do
  case $VP in 768) H=1024;; 1280) H=800;; esac
  "$AB" --session "lyp-r36-$VP" open "http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills" >/dev/null 2>&1; sleep 2
  "$AB" --session "lyp-r36-$VP" set viewport "$VP" "$H" >/dev/null 2>&1; "$AB" --session "lyp-r36-$VP" reload >/dev/null 2>&1; sleep 4
  "$AB" --session "lyp-r36-$VP" eval --stdin < "$E/scroll-composite.js" > "$E/data/scroll-fixed-$VP.json" 2>&1
  sleep 1; "$AB" --session "lyp-r36-$VP" screenshot "$E/screenshots/r36-fixed-$VP-composite.png" >/dev/null 2>&1
  echo "FIXED_$VP="; cat "$E/data/scroll-fixed-$VP.json"; echo
done
echo FIXED_DONE=YES
