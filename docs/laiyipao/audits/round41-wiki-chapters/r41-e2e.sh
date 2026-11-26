#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-wiki-chapters; DATA=$D/data; AB=agent-browser; S=lyp-r41-e2e
init="$PWD/$DATA/admin-auth-init.js"
$AB close --all >/dev/null 2>&1
$AB --session "$S" set viewport 1280 800 >/dev/null 2>&1
$AB --session "$S" open --init-script "$init" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/e2e-open.txt" 2>&1
$AB --session "$S" wait 1500 >/dev/null 2>&1
$AB --session "$S" select '.r34-level-mechanics .el-select' 3 > "$DATA/chapter-select3.txt" 2>&1
$AB --session "$S" wait 700 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/tab-probe.js" > "$DATA/chapter3.json" 2>&1
$AB --session "$S" select '.r34-level-mechanics .el-select' 6 > "$DATA/chapter-select6.txt" 2>&1
$AB --session "$S" wait 700 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/tab-probe.js" > "$DATA/chapter6.json" 2>&1
$AB --session "$S" click '.r41-ai-note a' > "$DATA/cta-click.txt" 2>&1
$AB --session "$S" wait 500 >/dev/null 2>&1
$AB --session "$S" eval --stdin < "$D/chapter-probe.js" > "$DATA/cta-post.json" 2>&1
for Q in 冻原 rotor_vane 反应链 元素覆盖; do
  case "$Q" in 冻原) N=fanyuan;; rotor_vane) N=rotor;; 反应链) N=fanyinglian;; 元素覆盖) N=yuansu;; esac
  $AB --session "$S" fill '.search-input' "$Q" > "$DATA/search-fill-$N.txt" 2>&1
  $AB --session "$S" wait 600 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/search-$N.json" 2>&1
  $AB --session "$S" click '.search-hit' > "$DATA/search-click-$N.txt" 2>&1
  $AB --session "$S" wait 700 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/searchpick-$N.json" 2>&1
done
$AB --session "$S" close >/dev/null 2>&1
echo R41_E2E_DONE
