#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-wiki-chapters; DATA=$D/data; AB=agent-browser; S=lyp-r41-drop
init="$PWD/$DATA/admin-auth-init.js"
$AB close --all >/dev/null 2>&1
$AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
$AB --session "$S" open --init-script "$init" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' >/dev/null 2>&1
$AB --session "$S" wait 1500 >/dev/null 2>&1
for C in 3 6; do
 $AB --session "$S" select '.r34-level-mechanics .el-select' $C > "$DATA/drop-select-$C.txt" 2>&1
 $AB --session "$S" wait 800 >/dev/null 2>&1
 $AB --session "$S" eval --stdin < "$D/chapter-dropdown-probe.js" > "$DATA/drop-$C.json" 2>&1
done
$AB --session "$S" close >/dev/null 2>&1
echo DROP_DONE
