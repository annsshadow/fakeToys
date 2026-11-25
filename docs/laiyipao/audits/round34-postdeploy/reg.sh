#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
ROOT=$PWD; AB=agent-browser; S=lyp-r34post-reg; E="$ROOT/docs/laiyipao/audits/round34-postdeploy"
P="$ROOT/docs/laiyipao/audits/round34-wiki-level-mechanics"
BASE="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab="
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
for TAB in basics skills enemies; do
  echo "=== REG $TAB ==="
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "$BASE$TAB" >/dev/null 2>&1; sleep 5
  "$AB" --session "$S" eval --stdin < "$P/reg-probe.js" > "$E/data/reg-$TAB.json" 2>&1
  "$AB" --session "$S" screenshot "$E/screenshots/r34post-reg-$TAB.png" >/dev/null 2>&1
done
"$AB" --session "$S" close >/dev/null 2>&1 || true
echo REG_DONE=YES
