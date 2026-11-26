#!/bin/bash
# 改造后 Round1：admin 12 视图真浏览器逐页复查（非物理真机）
S="lyp-admin-after"
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round1-after"
PROBE="docs/laiyipao/audits/round1-after/probe-init.js"
VIEWS="admin/dashboard admin/levels admin/skills admin/equipment admin/users admin/battles admin/recomputes admin/defenses admin/economy admin/content wiki"
AB="agent-browser --session $S"
echo "===LOGIN==="
$AB --init-script "$PROBE" open "$BASE/admin/login" >/dev/null 2>&1
$AB wait 1200 >/dev/null 2>&1
cat <<'EOF' | $AB eval --stdin 2>&1 | head -3
(()=>{const ins=[...document.querySelectorAll('input')]; if(ins.length<2)return 'NO_INPUTS:'+ins.length; return 'INPUTS='+ins.length})()
EOF
echo "(login performed separately via snapshot/eval)"
i=0
for v in $VIEWS; do
  i=$((i+1))
  name=$(echo "$v" | tr '/' '_')
  echo "=====ADMINVIEW[$i]:$v====="
  $AB open "$BASE/$v" 2>&1 | head -2
  $AB wait 1200 >/dev/null 2>&1
  $AB screenshot "$DIR/screenshots/admin-after-$(printf '%02d' $i)-$name.png" 2>&1 | head -1
  echo "---META---"; $AB get url 2>&1|head -1; $AB get title 2>&1|head -1
  echo "---DOM---"
  cat <<'EOF' | $AB eval --stdin 2>&1
JSON.stringify({
  probe: window.__LYP_PROBE__ ? {pageErrors:window.__LYP_PROBE__.pageErrors,consoleErrors:window.__LYP_PROBE__.consoleErrors,failedRequests:window.__LYP_PROBE__.failedRequests,apiFailures:window.__LYP_PROBE__.apiFailures} : null,
  url: location.href, title: document.title,
  domElements: document.querySelectorAll('*').length,
  textLen: (document.body.innerText||'').trim().length,
  textSample: (document.body.innerText||'').trim().slice(0,200),
  tables: document.querySelectorAll('table').length, rows: document.querySelectorAll('tr').length,
  buttons: document.querySelectorAll('button').length, links: document.querySelectorAll('a').length,
  charts: document.querySelectorAll('[_echarts_instance_]').length,
  cmdTrigger: document.querySelectorAll('.command-trigger').length,
  skeleton: document.querySelectorAll('[class*=skeleton]').length,
  visualCss: (()=>{const s=getComputedStyle(document.documentElement);return {bg:s.getPropertyValue('--lyp-bg').trim(),accent:s.getPropertyValue('--lyp-accent').trim(),glass:s.getPropertyValue('--lyp-glass-panel').trim()}})()
})
EOF
  echo "---ERRORS---"; $AB errors --json 2>&1 | head -c 900; echo ""
done
echo "===DONE==="
