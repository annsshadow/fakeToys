#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D="docs/laiyipao/audits/round42-wiki-level-chapter"; DATA=$D/data; AB=agent-browser; S=lyp-r42-dom
$AB close --all >/dev/null 2>&1
$AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/dom-open.txt" 2>&1
$AB --session "$S" set viewport 768 1024 >/dev/null 2>&1
$AB --session "$S" wait 1200 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/dom-structure-probe.js" > "$DATA/dom-structure.json" 2>&1
$AB --session "$S" close >/dev/null 2>&1
echo DOM_DONE
