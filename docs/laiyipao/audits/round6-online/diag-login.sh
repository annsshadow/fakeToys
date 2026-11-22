#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS"|head -1|sed "s/^DOMAIN=//;s/[[:space:]]*$//"|tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS"|head -1|sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//"|tr -d "\r")"
DIR="docs/laiyipao/audits/round6-online"; AB="agent-browser --session lyp-r6online-h5"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
$AB open "https://$DOMAIN/laiyipao-h5/#/pages/login/login" >/dev/null 2>&1; sleep 5
echo BEFORE; $AB eval --stdin < "$DIR/diag-login.js" 2>&1 | mask
echo FILL_USER; $AB fill "input[placeholder='用户名']" "admin" >/dev/null 2>&1; echo rc=$?
echo FILL_PASS; $AB fill "input[placeholder='口令']" "$ADMIN_PASS" >/dev/null 2>&1; echo rc=$?
echo AFTER_FILL; $AB eval --stdin < "$DIR/diag-login.js" 2>&1 | mask
echo CLICK; $AB click ".btn-primary" 2>&1 | mask
sleep 7
echo AFTER_CLICK_URL; $AB get url 2>&1 | mask
echo AFTER_CLICK_STATE; $AB eval --stdin < "$DIR/diag-login.js" 2>&1 | mask
echo DIAG_DONE