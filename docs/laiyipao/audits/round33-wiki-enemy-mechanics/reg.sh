#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r33-reg
D=docs/laiyipao/audits/round33-wiki-enemy-mechanics; E="$PWD/$D"
URL='http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=enemies'
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
"$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$URL" >/dev/null 2>&1; sleep 6
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1; "$AB" --session "$S" reload >/dev/null 2>&1; sleep 6
echo '=== 表内横向滚动末列 ==='
"$AB" --session "$S" eval --stdin < "$E/scroll-probe.js" > "$E/data/scroll.json" 2>&1
echo '=== R31/R32回归 ==='
"$AB" --session "$S" eval --stdin < "$E/reg-probe.js" > "$E/data/regression.json" 2>&1
"$AB" --session "$S" screenshot "$E/screenshots/r33-06-regression.png" >/dev/null 2>&1
sha256sum "$E/screenshots/r33-06-regression.png"
echo REG_DONE=YES