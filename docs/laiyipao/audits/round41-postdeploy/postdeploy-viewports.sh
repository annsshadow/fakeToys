#!/bin/bash
# R41 部署后视口校正复核：open 后 set viewport，捕获互异新图并保留搜索高亮证据
set -u
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-postdeploy; DATA=$D/data; SHOTS=$D/screenshots
AB=agent-browser; S=lyp-r41pd2; BASE=http://127.0.0.1:5175/laiyipao-admin
init="$PWD/$DATA/admin-auth-init.js"
for W in 390:844 768:1024 1280:800; do
  VP=${W%%:*}; VH=${W##*:}
  $AB close --all >/dev/null 2>&1
  $AB --session "$S" open --init-script "$init" "$BASE/#/wiki?tab=levels" > "$DATA/pd2-open-$VP.txt" 2>&1
  $AB --session "$S" set viewport "$VP" "$VH" >/dev/null 2>&1
  $AB --session "$S" wait 900 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/chapter-probe.js" > "$DATA/pd2-probe-$VP.json" 2>&1
  $AB --session "$S" screenshot "$SHOTS/pd2-chapter-$VP.png" > "$DATA/pd2-shot-chapter-$VP.txt" 2>&1
  $AB --session "$S" fill '.search-input' rotor_vane > "$DATA/pd2-fill-$VP.txt" 2>&1
  $AB --session "$S" wait 600 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/pd2-hits-$VP.json" 2>&1
  $AB --session "$S" click '.search-hit' > "$DATA/pd2-click-$VP.txt" 2>&1
  $AB --session "$S" wait 700 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/pd2-pick-$VP.json" 2>&1
  $AB --session "$S" screenshot "$SHOTS/pd2-search-$VP.png" > "$DATA/pd2-shot-search-$VP.txt" 2>&1
done
$AB close --all >/dev/null 2>&1
echo PD2_VIEWPORTS_DONE
