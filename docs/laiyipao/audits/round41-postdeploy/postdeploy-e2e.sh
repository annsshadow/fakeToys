#!/bin/bash
# R41 部署后真浏览器回归：三视口六章字段、CTA、chapter 搜索、十 tab、差异新图
set -u
cd "D:/WORKSPACE/fakeToys/.agents/worktrees/dsh-d0bc4721-3248" || exit 1
D=docs/laiyipao/audits/round41-postdeploy; DATA=$D/data; SHOTS=$D/screenshots
AB=agent-browser; S=lyp-r41pd; BASE=http://127.0.0.1:5175/laiyipao-admin
init="$PWD/$DATA/admin-auth-init.js"
mkdir -p "$DATA" "$SHOTS"
curl -sS 'http://127.0.0.1:8080/api/v1/config' > "$DATA/config.json"
echo "CONFIG_BYTES=$(wc -c < "$DATA/config.json" | tr -d ' ')"
click_tab() { $AB --session "$S" click "a.nav-item[href=\"#/wiki?tab=$1\"]" > "$DATA/pd-click-$1.txt" 2>&1; }
for W in 390:844 768:1024 1280:800; do
  VP=${W%%:*}; VH=${W##*:}
  $AB close --all >/dev/null 2>&1
  $AB --session "$S" set viewport "$VP" "$VH" >/dev/null 2>&1
  $AB --session "$S" open --init-script "$init" "$BASE/#/wiki?tab=levels" > "$DATA/pd-open-$VP.txt" 2>&1
  $AB --session "$S" wait 1200 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/chapter-probe.js" > "$DATA/probe-$VP.json" 2>&1
  $AB --session "$S" screenshot "$SHOTS/pd-chapter-$VP.png" > "$DATA/pd-shot-chapter-$VP.txt" 2>&1
  $AB --session "$S" eval --stdin < "$D/cta-pre.js" > "$DATA/cta-pre-$VP.json" 2>&1
  $AB --session "$S" click '.r41-ai-note a' > "$DATA/pd-cta-click-$VP.txt" 2>&1
  $AB --session "$S" wait 500 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/cta-post.js" > "$DATA/cta-post-$VP.json" 2>&1
  $AB --session "$S" screenshot "$SHOTS/pd-cta-$VP.png" > "$DATA/pd-shot-cta-$VP.txt" 2>&1
  $AB --session "$S" open --init-script "$init" "$BASE/#/wiki?tab=levels" > "$DATA/pd-reopen-$VP.txt" 2>&1
  $AB --session "$S" wait 1000 >/dev/null 2>&1
  $AB --session "$S" fill '.search-input' rotor_vane > "$DATA/pd-fill-$VP.txt" 2>&1
  $AB --session "$S" wait 600 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/pd-hits-$VP.json" 2>&1
  $AB --session "$S" click '.search-hit' > "$DATA/pd-click-$VP.txt" 2>&1
  $AB --session "$S" wait 700 >/dev/null 2>&1
  $AB --session "$S" eval --stdin < "$D/search-probe.js" > "$DATA/pd-pick-$VP.json" 2>&1
  $AB --session "$S" screenshot "$SHOTS/pd-chapter-search-$VP.png" > "$DATA/pd-shot-search-$VP.txt" 2>&1
  for T in basics skills composite enemies levels gems skins mastery rules rating; do
    click_tab "$T"; $AB --session "$S" wait 350 >/dev/null 2>&1
    $AB --session "$S" eval --stdin < "$D/reg-probe.js" > "$DATA/reg-$T.json" 2>&1
  done
  $AB --session "$S" screenshot "$SHOTS/pd-reg-$VP.png" > "$DATA/pd-shot-reg-$VP.txt" 2>&1
done
$AB close --all >/dev/null 2>&1
echo PD_E2E_DONE
