#!/bin/bash
set -u
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-postdeploy; DATA=$D/data; AB=agent-browser; S=lyp-r41reg4
$AB close --all >/dev/null 2>&1
$AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
$AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=basics' >/dev/null 2>&1
$AB --session "$S" wait 1500 >/dev/null 2>&1
for T in basics skills composite enemies levels gems skins mastery rules rating; do
  $AB --session "$S" click "a.nav-item[href=\"#/wiki?tab=$T\"]" > "$DATA/reg4-click-$T.txt" 2>&1
  $AB --session "$S" wait 700 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/reg-probe.js" > "$DATA/reg4-$T.json" 2>&1
done
$AB close --all >/dev/null 2>&1
echo REG4_DONE
