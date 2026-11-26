#!/bin/bash
# 改造前基线：admin 12 视图真浏览器逐页采集（非物理真机）
S="lyp-admin-qa"
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
DIR="docs/laiyipao/audits/baseline-round1"
AB="agent-browser --session $S"
VIEWS="admin/dashboard admin/levels admin/skills admin/equipment admin/users admin/battles admin/recomputes admin/defenses admin/economy admin/content wiki"
i=0
for v in $VIEWS; do
  i=$((i+1))
  name=$(echo "$v" | tr '/' '_')
  echo "=====ADMINVIEW[$i]:$v====="
  $AB errors --clear >/dev/null 2>&1
  $AB console --clear >/dev/null 2>&1
  $AB open "$BASE/$v" >/dev/null 2>&1
  $AB wait 1200 >/dev/null 2>&1
  echo "---META---"
  $AB get url 2>&1 | head -1
  $AB get title 2>&1 | head -1
  $AB screenshot "$DIR/screenshots/admin-$(printf '%02d' $i)-$name.png" 2>&1 | head -1
  echo "---DOM---"
  cat <<'EOF' | $AB eval --stdin 2>&1
JSON.stringify({
  probe: window.__LYP_PROBE__ || null,
  url: location.href,
  title: document.title,
  domElements: document.querySelectorAll('*').length,
  textLen: (document.body.innerText||'').trim().length,
  textSample: (document.body.innerText||'').trim().slice(0,220),
  tables: document.querySelectorAll('table').length,
  rows: document.querySelectorAll('tr').length,
  buttons: document.querySelectorAll('button').length,
  links: document.querySelectorAll('a').length,
  charts: document.querySelectorAll('[_echarts_instance_]').length,
  empty: (document.body.innerText||'').trim().length < 20,
  errorPage: (document.body.innerText||'').indexOf('401')>=0 || (document.body.innerText||'').indexOf('403')>=0 || (document.body.innerText||'').indexOf('错误')>=0
})
EOF
  echo "---ERRORS---"
  $AB errors --json 2>&1 | head -c 1200
  echo ""
  echo "---APICALLS---"
  $AB network requests 2>&1 | grep -iE 'api/v1' | head -15
done
echo "===DONE==="
