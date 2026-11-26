#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round33-postdeploy
E="$PWD/$D"; DATA="$E/data"; SH="$E/screenshots"
mkdir -p "$DATA" "$SH"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=enemies'
for SPEC in '390 390 844' '768 768 1024' '1280 1280 800'; do
  set -- $SPEC; V=$1; W=$2; H=$3; S="lyp-r33post-$V"
  echo "=== $V ${W}x${H} ==="
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$DATA/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" eval --stdin < "$E/enemy-probe.js" > "$DATA/probe-$V.json" 2>&1
  "$AB" --session "$S" screenshot "$SH/r33post-$V-enemies.png" >/dev/null 2>&1
  "$AB" --session "$S" close >/dev/null 2>&1 || true
done
sha256sum "$SH"/*.png
echo E2E_DONE=YES
