#!/bin/bash
# 改造后 Round1：H5 全 19 页真浏览器逐页复查（非物理真机，桌面 Chrome 移动端视口）
S="lyp-qa-after"
BASE="http://127.0.0.1:5174/#"
DIR="docs/laiyipao/audits/round1-after"
PROBE="docs/laiyipao/audits/round1-after/probe-init.js"
PAGES="pages/index/index pages/announcements/announcements pages/stage/stage pages/battle/battle pages/bag/bag pages/mastery/mastery pages/reactions/reactions pages/enemies/enemies pages/skills/skills pages/equipment/equipment pages/gems/gems pages/skins/skins pages/battles/battles pages/shop/shop pages/signin/signin pages/rank/rank pages/verify/verify pages/me/me pages/login/login"
AB="agent-browser --session $S"
echo "===INIT: inject probe on first open==="
$AB --init-script "$PROBE" open "$BASE/pages/index/index" >/dev/null 2>&1
$AB wait 1200 >/dev/null 2>&1
i=0
for p in $PAGES; do
  i=$((i+1))
  name=$(echo "$p" | tr '/' '_')
  echo "=====PAGE[$i]:$p====="
  $AB open "$BASE/$p" 2>&1 | head -2
  $AB wait 1000 >/dev/null 2>&1
  $AB screenshot "$DIR/screenshots/h5-after-$(printf '%02d' $i)-$name.png" 2>&1 | head -1
  echo "---META---"; $AB get url 2>&1 | head -1; echo "---DOM---"
  cat <<'EOF' | $AB eval --stdin 2>&1
JSON.stringify({
  probe: window.__LYP_PROBE__ ? {pageErrors:window.__LYP_PROBE__.pageErrors,consoleErrors:window.__LYP_PROBE__.consoleErrors,failedRequests:window.__LYP_PROBE__.failedRequests,apiFailures:window.__LYP_PROBE__.apiFailures} : null,
  url: location.href, title: document.title,
  domElements: document.querySelectorAll('*').length,
  textLen: (document.body.innerText||'').trim().length,
  textSample: (document.body.innerText||'').trim().slice(0,240),
  canvasCount: document.querySelectorAll('canvas').length,
  visual: { pageRoot: document.querySelectorAll('.page-root').length, card: document.querySelectorAll('.card').length,
    gridStyle: (()=>{const b=getComputedStyle(document.body);return b.backgroundImage&&b.backgroundImage!=='none'})(),
    cssVars: (()=>{const s=getComputedStyle(document.documentElement);return {bg:s.getPropertyValue('--bg').trim(),fire:s.getPropertyValue('--fire').trim(),glass:s.getPropertyValue('--glass-panel').trim()||s.getPropertyValue('--lyp-glass-panel').trim()}})() },
  empty: (document.body.innerText||'').trim().length < 10
})
EOF
  echo "---ERRORS---"; $AB errors --json 2>&1 | head -c 900; echo ""
done
echo "===DONE==="
