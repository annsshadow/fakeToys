#!/bin/bash
# R2 F-10-5 验证闭环：受限网速骨架屏运行时 + AI 卡入场动画中间帧（非物理真机）
S="lyp-qa-r2"
BASE="http://127.0.0.1:5174/#"
DIR="docs/laiyipao/audits/round2-after"
AB="agent-browser --session $S"
echo "=== STEP1 skeleton: delayed announcements fetch ==="
$AB open "about:blank" >/dev/null 2>&1
$AB --init-script "$DIR/skeleton-delay-init.js" open "$BASE/pages/announcements/announcements" >/dev/null 2>&1
sleep 1
echo "---skeleton state @1s---"
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/SKEL1: /'
JSON.stringify({url:location.href,skeletonRows:document.querySelectorAll('.skeleton-row').length,skeletonBars:document.querySelectorAll('.skeleton-bar').length,loadingText:(document.body.innerText||'').includes('加载中'),announcementVisible:(document.body.innerText||'').includes('共 ')})
EOF
$AB screenshot "$DIR/screenshots/r2-skeleton-running.png" 2>&1 | sed 's/^/SHOT: /'
echo "---wait 3.5s for data---"
sleep 4
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/LOADED: /'
JSON.stringify({skeletonRows:document.querySelectorAll('.skeleton-row').length,resultCount:(document.body.innerText||'').match(/共 \d+ 条公告/)?.[0]||null})
EOF
$AB screenshot "$DIR/screenshots/r2-skeleton-loaded.png" 2>&1 | sed 's/^/SHOT: /'
echo "=== STEP2 animation midframe via CDP Animation.setPlaybackRate ==="
$AB open "$BASE/pages/index/index" >/dev/null 2>&1
sleep 2
# 减慢动画到 0.1x 以便抓中间帧：先记录 animation 运行状态
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/ANIM_BEFORE: /'
(function(){var el=document.querySelector('#ai-insight');if(!el)return JSON.stringify({err:'no ai card'});var anims=el.getAnimations?el.getAnimations():[];return JSON.stringify({aiCard:1,animCount:anims.length,animNames:anims.map(function(a){return a.animationName||(a.effect&&a.effect.getKeyframes&&'keyframes')||'?'}),playStates:anims.map(function(a){return a.playState}),currentTimes:anims.map(function(a){return a.currentTime})});})()
EOF
echo "--- re-navigate to catch animation in progress ---"
$AB open "about:blank" >/dev/null 2>&1
$AB open "$BASE/pages/index/index" >/dev/null 2>&1
# 立即（不等待）抓 animation 中间态
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/ANIM_MID: /'
(function(){var el=document.querySelector('#ai-insight');if(!el)return JSON.stringify({err:'no ai card yet'});var anims=el.getAnimations?el.getAnimations():[];return JSON.stringify({aiCard:1,animCount:anims.length,states:anims.map(function(a){return a.playState}),times:anims.map(function(a){return Math.round(a.currentTime||0)}),opacity:getComputedStyle(el).opacity,transform:getComputedStyle(el).transform});})()
EOF
$AB screenshot "$DIR/screenshots/r2-ai-animation-midframe.png" 2>&1 | sed 's/^/SHOT: /'
echo "=== DONE ==="
