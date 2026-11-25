#!/bin/bash
# R42 部署后真浏览器回归：三视口逐章关卡所属章节 + 真CLI交叉引用/CTA + 十一tab + 新图sha
set -u
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round42-postdeploy; DATA=$D/data; SHOTS=$D/screenshots
AB=agent-browser; S=lyp-r42pd; BASE=http://127.0.0.1:5275/laiyipao-admin
init="$PWD/$DATA/admin-auth-init.js"
mkdir -p "$DATA" "$SHOTS"
curl -sS "http://127.0.0.1:8080/api/v1/config" > "$DATA/config.json"
echo "CONFIG_BYTES=$(wc -c < "$DATA/config.json" | tr -d " ")"

click_tab() { $AB --session "$S" click "a.nav-item[href="#/wiki?tab=$1"]" > "$DATA/pd-click-$1.txt" 2>&1; }
select_chapter() {
  $AB --session "$S" click ".r34-level-mechanics .el-select" > "$DATA/pd-sel-$1-$2.txt" 2>&1
  for i in $(seq 1 "$2"); do $AB --session "$S" press ArrowDown >> "$DATA/pd-sel-$1-$2.txt" 2>&1; done
  $AB --session "$S" press Enter >> "$DATA/pd-sel-$1-$2.txt" 2>&1
  $AB --session "$S" wait 1500 >/dev/null 2>&1
}

for W in 390:844 768:1024 1280:800; do
  VP=${W%%:*}; VH=${W##*:}
  $AB close --all >/dev/null 2>&1
  $AB --session "$S" open --init-script "$init" "$BASE/#/wiki?tab=levels" > "$DATA/pd-open-$VP.txt" 2>&1
  $AB --session "$S" set viewport "$VP" "$VH" > "$DATA/pd-vp-$VP.txt" 2>&1
  $AB --session "$S" wait 1200 >/dev/null 2>&1
  for IDX in 0 1 2 3 4 5; do
    $AB --session "$S" open --init-script "$init" "$BASE/#/wiki?tab=levels" > "$DATA/pd-levelopen-$VP-$IDX.txt" 2>&1
    $AB --session "$S" wait 900 >/dev/null 2>&1
    if [ "$IDX" -gt 0 ]; then select_chapter "$VP" "$IDX"; fi
    $AB --session "$S" eval --stdin < "$D/level-chapter-probe.js" > "$DATA/pd-level-$VP-ch$((IDX+1)).json" 2>&1
    if [ "$IDX" -eq 2 ] || [ "$IDX" -eq 5 ]; then $AB --session "$S" screenshot "$SHOTS/pd2-sel-debug-$VP-ch$((IDX+1)).png" > "$DATA/pd-selshot-$VP-$IDX.txt" 2>&1; fi
  done
  $AB --session "$S" screenshot "$SHOTS/pd2-level-ch6-$VP.png" > "$DATA/pd-shot-level-$VP.txt" 2>&1
  # 回到第一章，真CLI点击列内交叉引用链接
  $AB --session "$S" open --init-script "$init" "$BASE/#/wiki?tab=levels" > "$DATA/pd-reopen-$VP.txt" 2>&1
  $AB --session "$S" wait 900 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/click-probe.js" > "$DATA/pd-click-before-$VP.json" 2>&1
  $AB --session "$S" click ".r34-level-table-scroll .el-table__body tbody tr:first-child td:last-child a" > "$DATA/pd-click-link-$VP.txt" 2>&1
  $AB --session "$S" wait 800 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/click-probe.js" > "$DATA/pd-click-after-$VP.json" 2>&1
  $AB --session "$S" screenshot "$SHOTS/pd2-crossref-$VP.png" > "$DATA/pd-shot-crossref-$VP.txt" 2>&1
  # AI CTA 真CLI点击，应停留 levels 且 anchor=chapter-mechanics
  $AB --session "$S" eval --stdin < "$D/cta-probe.js" > "$DATA/pd-cta-before-$VP.json" 2>&1
  $AB --session "$S" click ".r42-level-chapter-note .r42-ai-cta a" > "$DATA/pd-cta-click-$VP.txt" 2>&1
  $AB --session "$S" wait 800 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/cta-probe.js" > "$DATA/pd-cta-after-$VP.json" 2>&1
  $AB --session "$S" screenshot "$SHOTS/pd2-cta-$VP.png" > "$DATA/pd-shot-cta-$VP.txt" 2>&1
  # 章节搜索 rotor_vane 命中并点击高亮
  $AB --session "$S" open --init-script "$init" "$BASE/#/wiki?tab=levels" > "$DATA/pd-reopen2-$VP.txt" 2>&1
  $AB --session "$S" wait 900 >/dev/null 2>&1
  $AB --session "$S" fill ".search-input" rotor_vane > "$DATA/pd-fill-$VP.txt" 2>&1
  $AB --session "$S" wait 600 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/pd-hits-$VP.json" 2>&1
  $AB --session "$S" click ".search-hit" > "$DATA/pd-search-click-$VP.txt" 2>&1
  $AB --session "$S" wait 700 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/pd-pick-$VP.json" 2>&1
  $AB --session "$S" screenshot "$SHOTS/pd2-search-$VP.png" > "$DATA/pd-shot-search-$VP.txt" 2>&1
  # R31–R41 十一 tab 回归（basics章节新增r42段落，不按旧假阳性处理）
  for T in basics skills composite enemies levels gems skins mastery rules rating; do
    click_tab "$T"; $AB --session "$S" wait 350 >/dev/null 2>&1
    $AB --session "$S" eval --stdin < "$D/nav-probe.js" > "$DATA/pd-reg-$T-$VP.json" 2>&1
  done
  $AB --session "$S" screenshot "$SHOTS/pd2-reg-$VP.png" > "$DATA/pd-shot-reg-$VP.txt" 2>&1
  echo "VIEWPORT_${VP}_DONE"
done
$AB close --all >/dev/null 2>&1
echo PD_E2E_DONE