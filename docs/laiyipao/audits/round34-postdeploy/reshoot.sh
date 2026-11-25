#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
ROOT=$PWD; AB=agent-browser; E="$ROOT/docs/laiyipao/audits/round34-postdeploy"; DATA="$E/data"; SH="$E/screenshots"
P="$ROOT/docs/laiyipao/audits/round34-wiki-level-mechanics"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels'
for V in 390 768 1280; do
  S="lyp-r34post-rs$V"
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport $V 844 >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$DATA/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" eval --stdin < "$E/scroll-page.js" > "$DATA/scroll-$V.json" 2>&1
  "$AB" --session "$S" screenshot "$SH/r34post-$V-scrolled.png" >/dev/null 2>&1
  "$AB" --session "$S" close >/dev/null 2>&1 || true
done
sha256sum "$SH"/*.png
echo RESHOOT_DONE=YES
