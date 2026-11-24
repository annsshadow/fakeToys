#!/bin/bash
set -e
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round31-postdeploy
E="$PWD/$D"
DATA="$E/data"
SH="$E/screenshots"
mkdir -p "$DATA" "$SH"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki'
for SPEC in 'desktop 1440 900' 'mobile375 375 390' 'mobile390 390 844'; do
  set -- $SPEC; V=$1; W=$2; H=$3; S="lyp-r31-postdeploy-$V"
  echo "=== $V ${W}x${H} ==="
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 7
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 7
  "$AB" --session "$S" eval --stdin < "$E/wiki-probe.js" > "$DATA/probe-$V.json" 2>&1
  "$AB" --session "$S" screenshot "$SH/r31post-$V-wiki.png" >/dev/null 2>&1; echo SHOT_$V
done
echo "=== EXTRA: deeplink/scroll/search on 390 ==="
S=lyp-r31-postdeploy-mobile390
"$AB" --session "$S" eval --stdin < "$E/wiki-deeplink-check.js" > "$DATA/deeplink-load.json" 2>&1
"$AB" --session "$S" eval --stdin < "$E/wiki-scroll-probe.js" > "$DATA/scroll-lastcol.json" 2>&1
"$AB" --session "$S" screenshot "$SH/r31post-04-deeplink.png" >/dev/null 2>&1; echo SHOT_04
echo "=== SHA ==="
sha256sum "$SH"/r31post-*.png
echo "=== DONE ==="
