#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D="docs/laiyipao/audits/round42-wiki-level-chapter"; DATA=$D/data; AB=agent-browser; S=lyp-r42-vp
init="$PWD/$DATA/admin-auth-init.js"
$AB close --all >/dev/null 2>&1
for vp in 390:844 768:1024 1280:800; do
  w=${vp%%:*}; h=${vp##*:}
  $AB --session "$S" open --init-script "$init" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/vp-open-$w.txt" 2>&1
  $AB --session "$S" set viewport $w $h > "$DATA/vp-set-$w.txt" 2>&1
  $AB --session "$S" wait 1200 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/level-chapter-probe.js" > "$DATA/vp-$w.json" 2>&1
  $AB --session "$S" screenshot "$D/screenshots/r42-level-chapter-$w.png" > "$DATA/vp-shot-$w.txt" 2>&1
done
$AB --session "$S" close >/dev/null 2>&1
echo THREE_VP_DONE
