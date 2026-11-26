#!/bin/bash
# t57 R6 admin 五视图复查（非物理真机·本地回环·口令脱敏）
DIR="docs/laiyipao/audits/round6-after"
S=lyp-r6-admin; AB="agent-browser --session $S"
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
for v in content defenses equipment wiki; do
  echo "=== ADMIN $v ==="
  $AB open "$BASE/admin/$v" >/dev/null 2>&1
  sleep 3
  cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/EVAL: /"
(function(){var p=window.__LYP_PROBE__||{};var b=document.body.innerText||"";var recs=(p.recResponses||[]).filter(function(r){return /admin\/recommendations/.test(r.url)});return JSON.stringify({url:location.href,title:document.title,isLogin:/login/.test(location.href),bodyLen:b.length,hasLocalTag:b.indexOf("本地规则引擎")>=0,hasAI:b.indexOf("AI")>=0,hasRecV1:b.indexOf("rec-v1")>=0,apiTag:(document.querySelector(".api-tag")||{}).innerText||"",recCalls:recs.map(function(r){return {scope:(r.url.split("/recommendations/")[1]||"").split("?")[0],status:r.status,schema:r.body&&r.body.schema_version,empty:r.body&&r.body.empty_reason,insights:r.body&&r.body.insights,headline:r.body&&r.body.headline}}),pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length})})()
EOF
  $AB screenshot "$DIR/screenshots/r6-admin-$v.png" >/dev/null 2>&1
  echo "SHOT: r6-admin-$v.png"
done
echo "=== ADMIN login view ==="
$AB open "$BASE/admin/login" >/dev/null 2>&1
sleep 3
cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/EVAL: /"
(function(){var b=document.body.innerText||"";return JSON.stringify({url:location.href,title:document.title,hasAI:b.indexOf("AI")>=0,snippet:b.slice(0,300)})})()
EOF
$AB screenshot "$DIR/screenshots/r6-admin-login.png" >/dev/null 2>&1
echo ADMIN_DONE