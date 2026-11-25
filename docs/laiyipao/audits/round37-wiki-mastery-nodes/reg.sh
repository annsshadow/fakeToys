#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 1
AB=agent-browser; S=lyp-r37-reg
D=docs/laiyipao/audits/round37-wiki-mastery-nodes
E="$PWD/$D"
BASE="http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab="
"$AB" --session "$S" close >/dev/null 2>&1 || true
"$AB" --session "$S" open about:blank >/dev/null 2>&1; sleep 1
"$AB" --session "$S" set viewport 390 844 >/dev/null 2>&1
for TAB in basics skills composite enemies levels gems mastery; do
  echo "=== REG TAB $TAB ==="
  "$AB" --session "$S" --init-script "$E/data/admin-auth-init.js" open "${BASE}${TAB}" >/dev/null 2>&1; sleep 5
  "$AB" --session "$S" eval --stdin < "$E/reg-probe.js" > "$E/data/reg-$TAB.json" 2>&1
  "$AB" --session "$S" screenshot "$E/screenshots/r37-reg-$TAB.png" >/dev/null 2>&1
done
echo REG_DONE=YES