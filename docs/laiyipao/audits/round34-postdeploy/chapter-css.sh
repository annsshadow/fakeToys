#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
ROOT=$PWD; AB=agent-browser; E="$ROOT/docs/laiyipao/audits/round34-postdeploy"
P="$ROOT/docs/laiyipao/audits/round34-wiki-level-mechanics"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels'
for N in 3 5 6; do
  S="lyp-r34post-css$N"
  "$AB" --session "$S" close >/dev/null 2>&1 || true
  "$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
  "$AB" --session "$S" set viewport 1280 800 >/dev/null 2>&1
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 5
  "$AB" --session "$S" reload >/dev/null 2>&1; sleep 4
  "$AB" --session "$S" click ".r34-level-mechanics .el-select" > "$E/data/css-$N-open.log" 2>&1; echo OPEN$N=$?
  sleep 1
  "$AB" --session "$S" click ".el-select-dropdown__item:nth-child($N)" > "$E/data/css-$N-pick.log" 2>&1; echo PICK$N=$?
  sleep 2
  "$AB" --session "$S" eval --stdin < "$P/boss-probe.js" > "$E/data/css-$N-rows.json" 2>&1
  "$AB" --session "$S" screenshot "$E/screenshots/r34post-css-$N.png" >/dev/null 2>&1
  "$AB" --session "$S" close >/dev/null 2>&1 || true
done
echo CSS_DONE=YES
