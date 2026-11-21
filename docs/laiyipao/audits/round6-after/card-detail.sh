#!/bin/bash
DIR="docs/laiyipao/audits/round6-after"; S=lyp-r6-admin; AB="agent-browser --session $S"
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
for v in content defenses equipment; do
  echo "=== CARD $v ==="
  $AB open "$BASE/admin/$v" >/dev/null 2>&1
  sleep 3
  cat <<EOF | $AB eval --stdin 2>&1 | sed "s/^/CARD: /"
(function(){var c=document.querySelector("#ai-insight-card");var t=c?(c.innerText||""):"NO_CARD";var recs=(window.__LYP_PROBE__.recResponses||[]).filter(function(r){return r.url.indexOf("/admin/recommendations/"+(window.__v||"") )>=0});return JSON.stringify({url:location.href,cardText:t.slice(0,400),hasLocalTag:t.indexOf("本地规则引擎")>=0,hasEmptyCode:/NO_HISTORY|NOT_ENOUGH_DATA|CONFIG_MISSING|数据不足/.test(t),kind:t.indexOf("数据不足")>=0?"empty":"funnel"})})()
EOF
done
echo "=== WIKI corrected #/wiki ==="
$AB open "$BASE/wiki" >/dev/null 2>&1
sleep 3
$AB eval --stdin 2>&1 <<'EOF' | sed "s/^/WIKI: /"
(function(){var b=document.body.innerText||"";var p=window.__LYP_PROBE__||{};var recs=(p.recResponses||[]).filter(function(r){return /admin\/recommendations\/wiki/.test(r.url)});return JSON.stringify({url:location.href,title:document.title,isLogin:/login/.test(location.href),bodyLen:b.length,hasAI:b.indexOf("AI")>=0,snippet:b.slice(0,300),wikiRec:recs.map(function(r){return {status:r.status,schema:r.body&&r.body.schema_version,empty:r.body&&r.body.empty_reason,insights:r.body&&r.body.insights}}),pe:(p.pageErrors||[]).length,ce:(p.consoleErrors||[]).length,af:(p.apiFailures||[]).length,fr:(p.failedRequests||[]).length})})()
EOF
$AB screenshot "$DIR/screenshots/r6-admin-wiki.png" >/dev/null 2>&1
echo CARD_DONE