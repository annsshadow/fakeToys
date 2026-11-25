#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D="docs/laiyipao/audits/round42-wiki-level-chapter"; DATA=$D/data; AB=agent-browser; S=lyp-r42-rc
$AB close --all >/dev/null 2>&1
for tab in basics skills; do
  $AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" "http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=$tab" > "$DATA/rc-open-$tab.txt" 2>&1
  $AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
  $AB --session "$S" wait 600 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/rc-precise-probe.js" > "$DATA/rc-$tab.json" 2>&1
done
$AB --session "$S" close >/dev/null 2>&1
echo RC_DONE
