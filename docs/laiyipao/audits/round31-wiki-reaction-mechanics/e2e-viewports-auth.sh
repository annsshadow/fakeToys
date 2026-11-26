#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round31-wiki-reaction-mechanics
E="$PWD/$D"
DATA="$E/data"
SH="$E/screenshots"
mkdir -p "$DATA" "$SH"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki'
for SPEC in 'desktop 1440 900' 'mobile375 375 390' 'mobile390 390 844'; do
  set -- $SPEC; V=$1; W=$2; H=$3; S="lyp-r31a-$V"
  echo "=== $V ${W}x${H} ==="
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" eval --stdin < "$E/wiki-probe.js" > "$DATA/probe-auth-$V.json" 2>&1
  "$AB" --session "$S" screenshot "$SH/r31a-$V-wiki.png" >/dev/null 2>&1; echo SHOT_$V
done
echo '=== SHA ==='
sha256sum "$SH"/r31a-*.png
echo '=== DONE ==='
