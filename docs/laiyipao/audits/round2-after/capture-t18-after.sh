#!/bin/bash
# t18 后 admin 看板/战报真浏览器复查（非物理真机，桌面 Chrome，本地回环）
S="lyp-admin-r2"
BASE="http://127.0.0.1:5175/laiyipao-admin/#"
DIR="docs/laiyipao/audits/round2-after"
PROBE="$DIR/probe-init.js"
AB="agent-browser --session $S"
PASS="${ADMIN_PASS:-}"
echo "=== STEP1 login + probe ==="
$AB --init-script "$PROBE" open "$BASE/admin/login" >/dev/null 2>&1
sleep 2
$AB fill "input[placeholder='admin']" "admin" >/dev/null 2>&1
$AB fill "input[placeholder='至少 10 位']" "$PASS" >/dev/null 2>&1
cat <<'EOF' | $AB eval --stdin >/dev/null 2>&1
(function(){var bs=document.querySelectorAll('button');for(var i=0;i<bs.length;i++){if((bs[i].innerText||'').trim()==='登录'){bs[i].click();return;}})()
EOF
sleep 3
echo "URL: $($AB get url 2>&1)"
echo "=== STEP2 dashboard t18 structure ==="
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/DASH_T18: /'
JSON.stringify({url:location.href,
  eyebrow:(function(){var e=document.querySelectorAll('.eyebrow,.section-eyebrow,[class*=eyebrow]');return Array.from(e).map(function(x){return (x.innerText||'').trim()}).filter(Boolean).slice(0,8)})(),
  kpiPrimary:document.querySelectorAll('.kpi-primary').length,
  kpiSecondary:document.querySelectorAll('.kpi-secondary').length,
  aiCard:document.querySelectorAll('.ai-insight-card,.ai-insight').length,
  gridSections:(function(){var e=document.querySelectorAll('[class*=kpi],[class*=metric],[class*=grid]');return e.length})(),
  bodyHasCore:((document.body.innerText||'').indexOf('核心指标')>=0),
  bodyHasEconomy:((document.body.innerText||'').indexOf('经济与防线')>=0),
  bodyHasTrend:((document.body.innerText||'').indexOf('趋势与分布')>=0),
  charts:document.querySelectorAll('canvas,.echarts,.chart-primary,[class*=chart]').length,
  probe:window.__LYP_PROBE__?{pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length,fr:window.__LYP_PROBE__.failedRequests.length}:null})
EOF
$AB screenshot "$DIR/screenshots/r2-admin-dashboard-t18.png" 2>&1 | sed 's/^/SHOT: /'
echo "=== STEP3 battles t18 structure ==="
$AB open "$BASE/admin/battles" >/dev/null 2>&1
sleep 3
echo "URL: $($AB get url 2>&1)"
cat <<'EOF' | $AB eval --stdin 2>&1 | sed 's/^/BATTLES_T18: /'
JSON.stringify({url:location.href,
  quickMetrics:(function(){var e=document.querySelectorAll('[class*=quick],[class*=summary],[class*=glance],[class*=stat]');return Array.from(e).map(function(x){return (x.innerText||'').trim()}).filter(Boolean).slice(0,10)})(),
  bodyHasRecords:((document.body.innerText||'').indexOf('本页记录')>=0),
  bodyHasWinRate:((document.body.innerText||'').indexOf('本页胜率')>=0),
  bodyHasVerify:((document.body.innerText||'').indexOf('验真异常')>=0),
  bodyHasConserve:((document.body.innerText||'').indexOf('守恒可疑')>=0),
  filterRows:document.querySelectorAll('.filter-row,[class*=filter-row]').length,
  tableRows:document.querySelectorAll('tbody tr,.el-table__row').length,
  probe:window.__LYP_PROBE__?{pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length,fr:window.__LYP_PROBE__.failedRequests.length}:null})
EOF
$AB screenshot "$DIR/screenshots/r2-admin-battles-t18.png" 2>&1 | sed 's/^/SHOT: /'
echo "=== DONE ==="
