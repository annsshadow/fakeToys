#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-wiki-chapters; DATA=$D/data; SH=$D/screenshots; AB=agent-browser; S=lyp-r41-vp-correct
init="$PWD/$DATA/admin-auth-init.js"; mkdir -p "$SH"
$AB close --all >/dev/null 2>&1
for spec in 390:844 768:1024 1280:800; do
  w=${spec%%:*}; h=${spec##*:}
  $AB --session "$S" open --init-script "$init" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/vpc-open-$w.txt" 2>&1
  $AB --session "$S" set viewport "$w" "$h" > "$DATA/vpc-set-$w.txt" 2>&1
  $AB --session "$S" wait 1600 >/dev/null 2>&1
  $AB --session "$S" eval 'JSON.stringify({iw:innerWidth,ih:innerHeight,hash:location.hash,docW:document.documentElement.scrollWidth,chapterRows:document.querySelectorAll(".r41-chapter-table-scroll .el-table__body tbody tr").length})' > "$DATA/vpc-state-$w.json" 2>&1
  $AB --session "$S" screenshot "$SH/r41-chapter-$w-correct.png" > "$DATA/vpc-shot-$w.txt" 2>&1
done
$AB --session "$S" close >/dev/null 2>&1
echo VPC_DONE
