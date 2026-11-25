#!/bin/bash
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-wiki-chapters; DATA=$D/data; SHOTS=$D/screenshots; AB=agent-browser; S=lyp-r41-qa
mkdir -p "$SHOTS"
init="$PWD/$DATA/admin-auth-init.js"
$AB close --all >/dev/null 2>&1
for W in 390 768 1280; do
  H=800; [ "$W" = 390 ] && H=844; [ "$W" = 768 ] && H=1024
  $AB --session "$S" set viewport $W $H >/dev/null 2>&1
  $AB --session "$S" open --init-script "$init" 'http://127.0.0.1:5175/laiyipao-admin/#/wiki?tab=levels' > "$DATA/vp-open-$W.txt" 2>&1
  $AB --session "$S" wait 1500 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/chapter-probe.js" > "$DATA/vp-chapter-$W.json" 2>&1
  $AB --session "$S" scrollintoview '.r41-chapter-mechanics' >/dev/null 2>&1
  $AB --session "$S" wait 500 >/dev/null 2>&1
  $AB --session "$S" screenshot "$SHOTS/r41-chapter-$W.png" > "$DATA/vp-shot-$W.txt" 2>&1
  $AB --session "$S" eval --stdin < "$D/scroll-probe.js" > "$DATA/vp-scroll-$W.json" 2>&1
done
$AB --session "$S" close >/dev/null 2>&1
echo R41_VIEWPORTS_DONE
