#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D="docs/laiyipao/audits/round42-wiki-level-chapter"; DATA=$D/data; AB=agent-browser; S=lyp-r42-ch3
$AB close --all >/dev/null 2>&1
$AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/ch3-open.txt" 2>&1
$AB --session "$S" set viewport 768 1024 >/dev/null 2>&1
$AB --session "$S" wait 1000 >/dev/null 2>&1
$AB --session "$S" click '.el-select' > "$DATA/ch3-click.txt" 2>&1
$AB --session "$S" press ArrowDown > "$DATA/ch3-down.txt" 2>&1
$AB --session "$S" press ArrowDown > "$DATA/ch3-down.txt" 2>&1
$AB --session "$S" press Enter > "$DATA/ch3-enter.txt" 2>&1
$AB --session "$S" wait 600 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/chapter-probe.js" > "$DATA/ch3-probe.json" 2>&1
$AB --session "$S" close >/dev/null 2>&1
echo CH3_DONE
