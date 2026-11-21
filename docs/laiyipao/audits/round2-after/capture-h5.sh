#!/bin/bash
# R2 H5 首屏三页真浏览器复查（非物理真机，桌面 Chrome 390x844 移动视口）
S="lyp-qa-r2"
BASE="http://127.0.0.1:5174/#"
DIR="docs/laiyipao/audits/round2-after"
PROBE="$DIR/probe-init.js"
AB="agent-browser --session $S"
echo "=== STEP1 open home + inject probe ==="
$AB --init-script "$PROBE" open "$BASE/pages/index/index" >/dev/null 2>&1
sleep 2
echo "open_rc=$?"
echo "=== STEP2 set viewport ==="
$AB set viewport 390 844 >/dev/null 2>&1
$AB reload >/dev/null 2>&1
sleep 2
echo "=== STEP3 capture home ==="
$AB get url 2>&1 | sed 's/^/URL: /'
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/HOME: /'
JSON.stringify({url:location.href,vp:[innerWidth,innerHeight],aiCard:document.querySelectorAll('#ai-insight.ai-insight-card').length,aiText:(document.querySelector('#ai-insight')?.innerText||'').slice(0,200),announcement:document.body.innerText.includes('公告'),probe:window.__LYP_PROBE__?{pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length,fr:window.__LYP_PROBE__.failedRequests.length}:null})
EOF
$AB screenshot "$DIR/screenshots/r2-h5-home-ai.png" 2>&1 | sed 's/^/SHOT: /'
echo "=== STEP4 capture stage ==="
$AB open "$BASE/pages/stage/stage" >/dev/null 2>&1
sleep 2
$AB get url 2>&1 | sed 's/^/URL: /'
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/STAGE: /'
JSON.stringify({url:location.href,hero:document.querySelectorAll('.stage-hero,.mission-brief,.stage-brief').length,continueCta:document.body.innerText.includes('继续挑战'),sigil:document.body.innerText.includes('作战简报'),probe:window.__LYP_PROBE__?{pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length}:null})
EOF
$AB screenshot "$DIR/screenshots/r2-h5-stage-hero.png" 2>&1 | sed 's/^/SHOT: /'
echo "=== STEP5 capture battle ==="
$AB open "$BASE/pages/battle/battle" >/dev/null 2>&1
sleep 2
$AB get url 2>&1 | sed 's/^/URL: /'
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/BATTLE: /'
JSON.stringify({url:location.href,sigilRail:document.querySelectorAll('.battle-sigil-rail').length,sigilLabel:document.querySelector('.battle-sigil-rail')?.getAttribute('aria-label'),canvas:document.querySelectorAll('canvas').length,probe:window.__LYP_PROBE__?{pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length}:null})
EOF
$AB screenshot "$DIR/screenshots/r2-h5-battle-sigil.png" 2>&1 | sed 's/^/SHOT: /'
echo "=== DONE ==="
