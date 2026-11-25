#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-wiki-chapters; DATA=$D/data; AB=agent-browser; S=lyp-r41-s2
$AB close --all
$AB --session "$S" set viewport 1280 800
$AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/s2-open.txt" 2>&1
echo OPEN_RC=$?
$AB --session "$S" wait 1500
$AB --session "$S" eval --stdin < "$D/chapter-probe.js" > "$DATA/s2-probe.txt" 2>&1
echo PROBE_RC=$?
$AB --session "$S" close
