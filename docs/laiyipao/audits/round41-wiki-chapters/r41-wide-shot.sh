#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-wiki-chapters; DATA=$D/data; SH=$D/screenshots; AB=agent-browser; S=lyp-r41-wide
init="$PWD/$DATA/admin-auth-init.js"; mkdir -p "$SH"
$AB close --all >/dev/null 2>&1
$AB --session "$S" set viewport 1280 1200 >/dev/null 2>&1
$AB --session "$S" open --init-script "$init" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' >/dev/null 2>&1
$AB --session "$S" wait 1500 >/dev/null 2>&1
$AB --session "$S" screenshot "$SH/r41-wide-full-verify.png" > "$DATA/wide-shot.txt" 2>&1
$AB --session "$S" close >/dev/null 2>&1
echo WIDE_DONE
