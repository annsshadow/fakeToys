#!/bin/bash
# R3 H5 六个次级页真浏览器复查（非物理真机，桌面 Chrome 390x844 移动视口）
# 探针注入，逐页抓 DOM 结构 + 截图；gems 页专项核对服务端配置数据加载
S="lyp-qa-r3"
BASE="http://127.0.0.1:5174/#"
DIR="docs/laiyipao/audits/round3-after"
PROBE="$DIR/probe-init.js"
AB="agent-browser --session $S"
echo "=== STEP1 open gems + inject probe ==="
$AB --init-script "$PROBE" open "$BASE/pages/gems/gems" >/dev/null 2>&1
sleep 2
$AB set viewport 390 844 >/dev/null 2>&1
$AB reload >/dev/null 2>&1
sleep 2
for P in gems shop mastery reactions skills equipment; do
  echo "=== PAGE $P ==="
  $AB open "$BASE/pages/$P/$P" >/dev/null 2>&1
  sleep 2
  $AB get url 2>&1 | sed 's/^/URL: /'
  cat <<EOF | $AB eval --stdin 2>&1 | sed 's/^/DOM: /'
JSON.stringify({url:location.href,kicker:document.querySelectorAll('.section-kicker').length,kickerCn:(document.querySelector('.section-kicker-cn')?.innerText||'').trim(),kickerEn:(document.querySelector('.section-kicker-en')?.innerText||'').trim(),kpi:document.querySelectorAll('.r3-kpi').length,kpiNum:(document.querySelector('.r3-kpi-num')?.innerText||'').trim(),filter:document.querySelectorAll('.r3-filter').length,chips:document.querySelectorAll('.r3-chip').length,aiCard:document.querySelectorAll('.r3-ai').length,aiText:(document.querySelector('.r3-ai')?.innerText||'').slice(0,160).replace(/\n/g,' '),empty:document.querySelectorAll('.r3-empty').length,resultCount:(document.querySelector('.result-count')?.innerText||'').trim(),rows:document.querySelectorAll('.r3-data-grid .list-row, .r3-data-grid .rxn').length,bodyLen:document.body.innerText.length,probe:window.__LYP_PROBE__?{pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length,fr:window.__LYP_PROBE__.failedRequests.length}:null})
EOF
  $AB screenshot "$DIR/screenshots/r3-h5-$P.png" 2>&1 | sed 's/^/SHOT: /'
done
echo "=== DONE ==="
