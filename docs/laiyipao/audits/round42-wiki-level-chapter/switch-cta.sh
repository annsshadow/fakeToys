#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D="docs/laiyipao/audits/round42-wiki-level-chapter"; DATA=$D/data; AB=agent-browser; S=lyp-r42-switch
$AB close --all >/dev/null 2>&1
$AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/switch-open.txt" 2>&1
$AB --session "$S" set viewport 768 1024 >/dev/null 2>&1
$AB --session "$S" wait 1200 >/dev/null 2>&1
$AB --session "$S" click '.el-select' > "$DATA/switch-click1.txt" 2>&1
$AB --session "$S" press ArrowDown > "$DATA/switch-press1.txt" 2>&1
$AB --session "$S" press Enter > "$DATA/switch-enter1.txt" 2>&1
$AB --session "$S" wait 600 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/chapter-probe.js" > "$DATA/switch-ch3.json" 2>&1
$AB --session "$S" click '.el-select' > "$DATA/switch-click2.txt" 2>&1
$AB --session "$S" press ArrowDown > "$DATA/switch-press2.txt" 2>&1
$AB --session "$S" press ArrowDown > "$DATA/switch-press2.txt" 2>&1
$AB --session "$S" press ArrowDown > "$DATA/switch-press2.txt" 2>&1
$AB --session "$S" press ArrowDown > "$DATA/switch-press2.txt" 2>&1
$AB --session "$S" press Enter > "$DATA/switch-enter2.txt" 2>&1
$AB --session "$S" wait 600 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/chapter-probe.js" > "$DATA/switch-ch6.json" 2>&1
$AB --session "$S" eval --stdin < "$D/cta-probe.js" > "$DATA/cta-before.json" 2>&1
$AB --session "$S" click '.r42-level-chapter-note .r42-ai-cta a' > "$DATA/cta-click.txt" 2>&1
$AB --session "$S" wait 800 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/cta-probe.js" > "$DATA/cta-after.json" 2>&1
$AB --session "$S" screenshot "$D/screenshots/r42-cta-after.png" > "$DATA/cta-shot.txt" 2>&1
$AB --session "$S" close >/dev/null 2>&1
echo SWITCH_CTA_DONE
