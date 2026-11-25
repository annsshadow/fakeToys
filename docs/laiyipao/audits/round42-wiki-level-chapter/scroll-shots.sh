#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D="docs/laiyipao/audits/round42-wiki-level-chapter"; DATA=$D/data; AB=agent-browser; S=lyp-r42-scroll
$AB close --all >/dev/null 2>&1
for vp in 390:844 768:1024 1280:800; do
  w=${vp%%:*}; h=${vp##*:}
  $AB --session "$S" open --init-script "$PWD/$DATA/admin-auth-init.js" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/ss-open-$w.txt" 2>&1
  $AB --session "$S" set viewport $w $h >/dev/null 2>&1
  $AB --session "$S" wait 1000 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/scroll-shot-probe.js" > "$DATA/ss-state-$w.json" 2>&1
  $AB --session "$S" wait 400 >/dev/null 2>&1
  $AB --session "$S" screenshot "$D/screenshots/r42-level-table-$w.png" > "$DATA/ss-shot-$w.txt" 2>&1
done
$AB --session "$S" close >/dev/null 2>&1
echo SCROLL_SHOTS_DONE
