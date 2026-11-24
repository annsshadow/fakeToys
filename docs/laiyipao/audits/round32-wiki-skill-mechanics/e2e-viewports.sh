#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round32-wiki-skill-mechanics
E="$PWD/$D"
DATA="$E/data"
SH="$E/screenshots"
mkdir -p "$DATA" "$SH"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills'
for SPEC in '390 390 844' '768 768 1024' '1280 1280 800'; do
  set -- $SPEC; V=$1; W=$2; H=$3; S="lyp-r32-$V"
  echo "=== $V ${W}x${H} ==="
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" eval --stdin < "$E/wiki-skill-probe.js" > "$DATA/probe-$V.json" 2>&1
  "$AB" --session "$S" screenshot "$SH/r32-$V-skills.png" >/dev/null 2>&1; echo SHOT_$V
done
echo '=== SHA ==='
sha256sum "$SH"/r32-*.png
echo '=== DONE ==='