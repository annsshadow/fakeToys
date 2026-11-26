#!/bin/bash
# t38 R4 真浏览器复查 - H5 六页（本地回环，非物理真机，游客态）
BASE="http://127.0.0.1:5174/#"
DIR="docs/laiyipao/audits/round4-after"
S="lyp-r4after-h5"
AB="agent-browser --session $S"
FIRST=1
for P in bag battles enemies me rank skins; do
  if [ $FIRST -eq 1 ]; then
    $AB --init-script "$DIR/probe-init.js" open "$BASE/pages/$P/$P" >/dev/null 2>&1
    FIRST=0
  else
    $AB open "$BASE/pages/$P/$P" >/dev/null 2>&1
  fi
  sleep 4
  $AB set viewport 390 844 >/dev/null 2>&1
  $AB screenshot "$DIR/screenshots/r4-h5-$P.png" >/dev/null 2>&1
  echo "=== $P ==="
  $AB get url 2>&1 | sed "s/^/URL: /"
  $AB get title 2>&1 | sed "s/^/TITLE: /"
  cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/DOM: /"
JSON.stringify({url:location.href,isLogin:location.href.indexOf("/login/login")>=0,title:document.title,kicker:document.querySelectorAll(".section-kicker").length,kickerCn:(document.querySelector(".section-kicker-cn")||{}).innerText||"",kpi:document.querySelectorAll(".r3-kpi,.r4-kpi").length,ai:document.querySelectorAll(".r3-ai,.r4-ai").length,aiBadge:(document.querySelector(".r3-ai-badge")||{}).innerText||"",aiConclusion:(document.querySelector(".r3-ai-conclusion")||{}).innerText||"",aiBasis:(document.querySelector(".r3-ai-basis")||{}).innerText||"",aiCta:(document.querySelector(".r3-ai-cta")||{}).innerText||"",filter:document.querySelectorAll(".r3-filter,.r4-filter,.chip-row").length,grid:document.querySelectorAll(".r3-data-grid,.r4-grid").length,rows:document.querySelectorAll(".r3-data-grid tr, .r4-grid tr, .gem-row, .r3-row").length,bodyLen:document.body.innerText.length,probe:(function(){var p=window.__LYP_PROBE__||{};return {pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length}})()})
EOF
done
echo DONE