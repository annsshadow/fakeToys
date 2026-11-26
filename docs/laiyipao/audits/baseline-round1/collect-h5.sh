#!/bin/bash
# 改造前基线：H5 全 19 页真浏览器逐页采集（非物理真机，桌面 Chrome 移动端视口）
S="lyp-qa-6713e3aa104c"
BASE="http://127.0.0.1:5174/#"
DIR="docs/laiyipao/audits/baseline-round1"
PAGES="pages/index/index pages/announcements/announcements pages/stage/stage pages/battle/battle pages/bag/bag pages/mastery/mastery pages/reactions/reactions pages/enemies/enemies pages/skills/skills pages/equipment/equipment pages/gems/gems pages/skins/skins pages/battles/battles pages/shop/shop pages/signin/signin pages/rank/rank pages/verify/verify pages/me/me pages/login/login"
AB="agent-browser --session $S"

echo "===SETUP: resize mobile viewport==="
$AB resize 390 844 2>&1 | head -2

i=0
for p in $PAGES; do
  i=$((i+1))
  name=$(echo "$p" | tr '/' '_')
  echo "=====PAGE[$i]:$p====="
  $AB open "about:blank" >/dev/null 2>&1
  $AB errors --clear >/dev/null 2>&1
  $AB console --clear >/dev/null 2>&1
  $AB open "$BASE/$p" 2>&1 | head -3
  $AB wait 1000 2>&1 | head -1
  $AB screenshot "$DIR/screenshots/h5-$(printf '%02d' $i)-$name.png" 2>&1 | head -1
  echo "---META---"
  $AB get url 2>&1 | head -1
  echo "---DOM---"
  cat <<'EOF' | $AB eval --stdin 2>&1
JSON.stringify({
  probe: window.__LYP_PROBE__ || null,
  url: location.href,
  title: document.title,
  domElements: document.querySelectorAll('*').length,
  textLen: (document.body.innerText||'').trim().length,
  textSample: (document.body.innerText||'').trim().slice(0,260),
  canvasCount: document.querySelectorAll('canvas').length,
  buttonLike: document.querySelectorAll('button,[role=button],.uni-button,[class*=btn]').length,
  imgCount: document.querySelectorAll('img').length,
  empty: (document.body.innerText||'').trim().length < 10
})
EOF
  echo "---ERRORS---"
  $AB errors --json 2>&1 | head -c 1500
  echo ""
done
echo "===DONE==="
