#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
ROOT=$PWD; AB=agent-browser; E="$ROOT/docs/laiyipao/audits/round34-postdeploy"; DATA="$E/data"; SH="$E/screenshots"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels'
S=lyp-r34post-dist
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
for N in 1 2 3 4; do
  "$AB" --session "$S" --init-script "$DATA/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 5
  "$AB" --session "$S" eval --stdin < "$DATA/scroll-d$N.js" > "$DATA/distinct-$N.json" 2>&1
  "$AB" --session "$S" screenshot "$SH/r34post-distinct-$N.png" >/dev/null 2>&1
done
"$AB" --session "$S" close >/dev/null 2>&1 || true
sha256sum "$SH"/r34post-distinct-*.png
echo DISTINCT_DONE=YES
