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
  set -- $SPEC; S=$1; W=$2; H=$3
  echo "=== $S ${W}x${H} ==="
  "$AB" --session "lyp-r31-$S" close >/dev/null 2>&1 || true
  "$AB" --session "lyp-r31-$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "lyp-r31-$S" set viewport $W $H >/dev/null 2>&1
  "$AB" --session "lyp-r31-$S" open "$URL" >/dev/null 2>&1; sleep 6
  "$AB" --session "lyp-r31-$S" set viewport $W $H >/dev/null 2>&1; "$AB" --session "lyp-r31-$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "lyp-r31-$S" eval --stdin < "$E/wiki-probe.js" > "$DATA/probe-$S.json" 2>&1
  "$AB" --session "lyp-r31-$S" screenshot "$SH/r31-$S-wiki.png" >/dev/null 2>&1; echo SHOT_$S
done
echo '=== SHA ==='
sha256sum "$SH"/r31-*.png
echo '=== DONE ==='
