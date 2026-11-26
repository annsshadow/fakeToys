#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-wiki-chapters; DATA=$D/data; AB=agent-browser; S=lyp-r41-dropkeys
init="$PWD/$DATA/admin-auth-init.js"
$AB close --all >/dev/null 2>&1
$AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
$AB --session "$S" open --init-script "$init" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' >/dev/null 2>&1
$AB --session "$S" wait 1500 >/dev/null 2>&1
$AB --session "$S" click '.r34-level-mechanics .el-select' > "$DATA/dropkeys-open3.txt" 2>&1
$AB --session "$S" press ArrowDown >/dev/null 2>&1
$AB --session "$S" press ArrowDown >/dev/null 2>&1
$AB --session "$S" press Enter > "$DATA/dropkeys-enter3.txt" 2>&1
$AB --session "$S" wait 800 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/chapter-dropdown-probe.js" > "$DATA/dropkeys-3.json" 2>&1
$AB --session "$S" click '.r34-level-mechanics .el-select' > "$DATA/dropkeys-open6.txt" 2>&1
$AB --session "$S" press ArrowDown >/dev/null 2>&1
$AB --session "$S" press ArrowDown >/dev/null 2>&1
$AB --session "$S" press ArrowDown >/dev/null 2>&1
$AB --session "$S" press Enter > "$DATA/dropkeys-enter6.txt" 2>&1
$AB --session "$S" wait 800 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/chapter-dropdown-probe.js" > "$DATA/dropkeys-6.json" 2>&1
$AB --session "$S" close >/dev/null 2>&1
echo DROPKEYS_DONE
