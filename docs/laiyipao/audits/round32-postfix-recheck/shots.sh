#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser
D=docs/laiyipao/audits/round32-postfix-recheck
E="$PWD/$D"; SH="$E/screenshots"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=skills'
for SPEC in '390 390 844 r32p-01-390' '768 768 1024 r32p-02-768' '1280 1280 800 r32p-03-1280'; do
  set -- $SPEC; V=$1; W=$2; H=$3; N=$4; S="lyp-shot-$V"
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" set viewport $W $H >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
  "$AB" --session "$S" screenshot "$SH/$N.png" >/dev/null 2>&1
  echo SHOT_$N
done
echo SHOTS_DONE=YES
