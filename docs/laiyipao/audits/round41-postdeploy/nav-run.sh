#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-postdeploy; DATA=$D/data; AB=agent-browser; S=lyp-r41nav
$AB close --all >/dev/null 2>&1
$AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=basics' >/dev/null 2>&1
$AB --session "$S" wait 1200 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/nav-probe.js" > "$DATA/nav-basics.json" 2>&1
$AB close --all >/dev/null 2>&1
echo NAV_DONE
