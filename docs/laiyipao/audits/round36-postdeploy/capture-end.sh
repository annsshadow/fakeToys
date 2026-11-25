#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; E="$PWD/docs/laiyipao/audits/round36-postdeploy"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills&anchor=composite'
for SPEC in '390 844' '768 1024' '1280 800'; do
  set -- $SPEC; VP=$1; H=$2; S=lyp-r36post-end-$VP
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport "$VP" "$H" >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" set viewport "$VP" "$H" >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" eval --stdin < "$E/scroll-bottom.js" > "$E/data/scroll-bottom-$VP.json" 2>&1
  sleep 1
  "$AB" --session "$S" screenshot "$E/screenshots/r36post-$VP-composite-end.png" >/dev/null 2>&1
  echo "=== END $VP ==="; head -c 300 "$E/data/scroll-bottom-$VP.json"; echo
  "$AB" --session "$S" close >/dev/null 2>&1 || true
done
echo END_DONE=YES
