#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-wiki-chapters; DATA=$D/data; AB=agent-browser; S=lyp-r41-vpdbg2
init="$PWD/$DATA/admin-auth-init.js"
$AB close --all >/dev/null 2>&1
$AB --session "$S" open --init-script "$init" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/vpdbg2-open.txt" 2>&1
$AB --session "$S" set viewport 390 844 > "$DATA/vpdbg2-set.txt" 2>&1
$AB --session "$S" wait 1200 >/dev/null 2>&1
$AB --session "$S" eval 'JSON.stringify({iw:innerWidth,ih:innerHeight,ow:outerWidth,oh:outerHeight})' > "$DATA/vpdbg2.txt" 2>&1
$AB --session "$S" close >/dev/null 2>&1
echo VPDBG2_DONE
