#!/bin/bash
# 干净会话：新开 H5 会话，原生登录，注入后置F2诊断监听器，玩一段战斗读 defaultPrevented
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
PASS=$(grep -E '^ADMIN_PASS=' "$KEYS"|head -1|sed 's/^ADMIN_PASS=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-f2final"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$PASS|<ADMIN_PASS>|g"; }
$AB open "https://$DOMAIN/laiyipao-h5/#/pages/login/login" >/dev/null 2>&1; sleep 4
$AB set viewport 390 844 >/dev/null 2>&1
$AB fill "input[type='text']" admin >/dev/null 2>&1
$AB fill "input[type='password']" "$PASS" >/dev/null 2>&1
$AB click ".btn-primary" >/dev/null 2>&1; sleep 4
echo "=== install F2 post-listener on index ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/f2-diag-install.js 2>&1 | mask
echo "=== go battle + install again ==="
$AB open "https://$DOMAIN/laiyipao-h5/#/pages/battle/battle" >/dev/null 2>&1; sleep 2
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/f2-diag-install.js 2>&1 | mask
echo "=== play 20s (auto skip cards) ==="
for n in 1 2 3 4 5 6; do $AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/e2e-battle-check.js >/dev/null 2>&1; sleep 3; done
echo "=== FINAL F2 DIAG ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/f2-final-read.js 2>&1 | mask
echo done