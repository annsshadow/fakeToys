#!/bin/bash
set -e
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser
E="$PWD/docs/laiyipao/audits/round31-postdeploy"
for SPEC in "desktop desktop" "mobile375 mobile375" "mobile390 mobile390"; do
  set -- $SPEC; V=$1; S="lyp-r31-postdeploy-$V"
  echo "=== $V ==="
  "$AB" --session "$S" eval --stdin < "$E/state-$V.js" > "$E/data/state-$V.json" 2>&1
  cat "$E/data/state-$V.json"
  "$AB" --session "$S" screenshot "$E/screenshots/r31post-$V-wiki.png" >/dev/null 2>&1
  echo SHOT_$V
done
S=lyp-r31-postdeploy-mobile390
"$AB" --session "$S" eval --stdin < "$E/state-fourth.js" > "$E/data/state-fourth.json" 2>&1
cat "$E/data/state-fourth.json"
"$AB" --session "$S" screenshot "$E/screenshots/r31post-04-deeplink.png" >/dev/null 2>&1
echo SHOT_04
echo "=== SHA ==="
sha256sum "$E"/screenshots/r31post-*.png
