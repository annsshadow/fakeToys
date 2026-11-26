#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D="docs/laiyipao/audits/round42-wiki-level-chapter"; DATA=$D/data; AB=agent-browser; S=lyp-r42-click
$AB close --all >/dev/null 2>&1
$AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/click-open.txt" 2>&1
$AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
$AB --session "$S" wait 1200 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/click-probe.js" > "$DATA/click-before.json" 2>&1
$AB --session "$S" click '.r34-level-table-scroll .el-table__body tbody tr:first-child td:last-child a' > "$DATA/click-link.txt" 2>&1
$AB --session "$S" wait 800 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/click-probe.js" > "$DATA/click-after.json" 2>&1
$AB --session "$S" screenshot "$D/screenshots/r42-crossref-after.png" > "$DATA/click-shot.txt" 2>&1
$AB --session "$S" eval --stdin < "$D/level-chapter-probe.js" > "$DATA/click-levelprobe.json" 2>&1
$AB --session "$S" close >/dev/null 2>&1
echo CLICK_DONE
