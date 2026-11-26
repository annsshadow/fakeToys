#!/bin/bash
# 测试环境 H5 复查（真值内存读取，输出全部脱敏；PLAYER_LOGIN_REQUIRED=true 需真实登录）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E '^DOMAIN=' "$KEYS" | head -1 | sed 's/^DOMAIN=//;s/[[:space:]]*$//' | tr -d '\r')"
ADMIN_PASS="$(grep -E '^ADMIN_PASS=' "$KEYS" | head -1 | sed 's/^ADMIN_PASS=//;s/[[:space:]]*$//' | tr -d '\r')"
BASE="https://$DOMAIN/laiyipao-h5/#"
DIR="docs/laiyipao/audits/round3-postdeploy"
S="lyp-posth5b"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g" | sed -E 's#https://[^ ]*<YOUR_TEST_DOMAIN>#https://<YOUR_TEST_DOMAIN>#g'; }
echo "=== STEP1 open login + probe + viewport ==="
$AB --init-script "$DIR/probe-init.js" open "$BASE/pages/login/login" >/dev/null 2>&1
sleep 4
$AB set viewport 390 844 >/dev/null 2>&1
$AB get url 2>&1 | mask | sed 's/^/INIT: /'
echo "=== STEP2 REAL login (native click) ==="
$AB fill "input[type='text']" "admin" >/dev/null 2>&1
$AB fill "input[type='password']" "$ADMIN_PASS" >/dev/null 2>&1
$AB click ".btn-primary" >/dev/null 2>&1
sleep 5
$AB get url 2>&1 | mask | sed 's/^/AFTER_LOGIN: /'
cat <<'EOF' | $AB eval --stdin 2>&1 | mask | sed 's/^/AUTH: /'
JSON.stringify({url:location.href,hasToken:!!localStorage.getItem('lyp_token'),hasProfile:!!localStorage.getItem('lyp_profile')})
EOF
echo "=== STEP3 seven pages ==="
for P in gems skills equipment mastery shop reactions skins; do
  $AB open "$BASE/pages/$P/$P" >/dev/null 2>&1
  sleep 3
  $AB get url 2>&1 | mask | sed 's/^/URL: /'
  cat <<EOF | $AB eval --stdin 2>&1 | mask | sed 's/^/DOM: /'
JSON.stringify({url:location.href,isLogin:location.href.includes('/login/login'),hasToken:!!localStorage.getItem('lyp_token'),kicker:document.querySelectorAll('.section-kicker').length,kickerCn:(document.querySelector('.section-kicker-cn')?.innerText||'').trim(),kpi:document.querySelectorAll('.r3-kpi').length,kpiNum:(document.querySelector('.r3-kpi-num')?.innerText||'').trim(),chips:document.querySelectorAll('.r3-chip').length,aiCard:document.querySelectorAll('.r3-ai').length,aiText:(document.querySelector('.r3-ai')?.innerText||'').slice(0,110).replace(/\n/g,' '),empty:document.querySelectorAll('.r3-empty').length,resultCount:(document.querySelector('.result-count')?.innerText||'').trim(),rows:document.querySelectorAll('.r3-data-grid .list-row, .r3-data-grid .rxn').length,bodyLen:document.body.innerText.length,probe:window.__LYP_PROBE__?{pe:window.__LYP_PROBE__.pageErrors.length,ce:window.__LYP_PROBE__.consoleErrors.length,af:window.__LYP_PROBE__.apiFailures.length,fr:window.__LYP_PROBE__.failedRequests.length}:null})
EOF
  $AB screenshot "$DIR/screenshots/post-h5-$P.png" >/dev/null 2>&1
  echo "$P shot saved"
done
echo "=== DONE ==="
