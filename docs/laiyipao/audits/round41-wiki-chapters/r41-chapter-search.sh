#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-wiki-chapters; DATA=$D/data; AB=agent-browser; S=lyp-r41-chapter
init="$PWD/$DATA/admin-auth-init.js"
$AB close --all >/dev/null 2>&1
$AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
$AB --session "$S" open --init-script "$init" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' >/dev/null 2>&1
$AB --session "$S" wait 1500 >/dev/null 2>&1
$AB --session "$S" fill '.search-input' '冻原' > "$DATA/ch-fill-fanyuan.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/chapter-hit-probe.js" > "$DATA/ch-hits-fanyuan.json" 2>&1
$AB --session "$S" find text '冻原' nth 20 click > "$DATA/ch-click-fanyuan.txt" 2>&1
$AB --session "$S" wait 700 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/chapter-hit-probe.js" > "$DATA/ch-pick-fanyuan.json" 2>&1
$AB --session "$S" fill '.search-input' '母巢外围' > "$DATA/ch-fill-muchao.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/chapter-hit-probe.js" > "$DATA/ch-hits-muchao.json" 2>&1
$AB --session "$S" click '.search-hit' > "$DATA/ch-click-muchao.txt" 2>&1
$AB --session "$S" wait 700 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/chapter-hit-probe.js" > "$DATA/ch-pick-muchao.json" 2>&1
$AB --session "$S" close >/dev/null 2>&1
echo CH_DONE
