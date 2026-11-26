#!/bin/bash
# t57 R6 H5 三页复查（非物理真机·本地回环）
DIR="docs/laiyipao/audits/round6-after"
S="lyp-r6-h5"
AB="agent-browser --session $S"
BASE="http://127.0.0.1:5174/#"
PAGES="equipment reactions skills"
for pg in $PAGES; do
  echo "=== H5 $pg ==="
  $AB open "$BASE/pages/$pg/$pg" >/dev/null 2>&1
  sleep 4
  $AB snapshot -i -c 2>&1 | head -40 | sed "s/^/SNAP: /"
  cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/EVAL: /"
(function(){var p=window.__LYP_PROBE__||{};var b=document.body.innerText||"";var aiBlocks=[].slice.call(document.querySelectorAll("*")).filter(function(e){var t=(e.textContent||"");return e.children.length===0&&(t.indexOf("AI")>=0||t.indexOf("推荐")>=0)}).slice(0,6).map(function(e){return (e.textContent||"").trim().slice(0,80)});return JSON.stringify({url:location.href,title:document.title,bodyLen:b.length,hasLocalTag:b.indexOf("本地规则引擎")>=0,hasRecV1:b.indexOf("rec-v1")>=0,hasEmptyCode:/NO_HISTORY|NOT_ENOUGH_DATA|CONFIG_MISSING/.test(b),aiBlocks:aiBlocks,recResponses:p.recResponses||[],pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length})})()
EOF
  $AB screenshot "$DIR/screenshots/r6-h5-$pg.png" >/dev/null 2>&1
  echo "SHOT: r6-h5-$pg.png"
done
echo H5_DONE