#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D="docs/laiyipao/audits/round42-wiki-level-chapter"; DATA=$D/data; AB=agent-browser; S=lyp-r42-reg
$AB close --all >/dev/null 2>&1
$AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" > "$DATA/reg-open.txt" 2>&1
$AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
for tab in basics rating enemies skills gems equipment skins rules mastery levels live; do
  $AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" "http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=$tab" > "$DATA/reg-open-$tab.txt" 2>&1
  $AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
  $AB --session "$S" wait 500 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/regression-probe.js" > "$DATA/reg-$tab.json" 2>&1
done
$AB --session "$S" close >/dev/null 2>&1
echo REG_DONE
