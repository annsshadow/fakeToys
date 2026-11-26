#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS"|head -1|sed "s/^DOMAIN=//;s/[[:space:]]*$//"|tr -d "\r")"
ADMIN_PASS="$(grep -E "^ADMIN_PASS=" "$KEYS"|head -1|sed "s/^ADMIN_PASS=//;s/[[:space:]]*$//"|tr -d "\r")"
DIR="docs/laiyipao/audits/round6-online"; AB="agent-browser --session lyp-r6online-h5"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo "PASS_LEN=${#ADMIN_PASS}"
echo STEP0_CLEAR; cat <<EOF | $AB eval --stdin >/dev/null 2>&1
(function(){try{localStorage.removeItem("lyp_token");localStorage.removeItem("lyp_token_exp");localStorage.removeItem("lyp_profile")}catch(e){};return true})()
EOF
$AB open "https://$DOMAIN/laiyipao-h5/#/pages/login/login" >/dev/null 2>&1; sleep 7
echo BEFORE; $AB eval --stdin < "$DIR/diag-login.js" 2>&1 | mask
echo FILL_USER; $AB fill "input[type='text']" "admin" 2>&1 | mask
echo FILL_PASS; $AB fill "input[type='password']" "$ADMIN_PASS" 2>&1 | mask
echo AFTER_FILL; $AB eval --stdin < "$DIR/diag-login.js" 2>&1 | mask
echo CLICK; $AB click ".btn-primary" 2>&1 | mask
sleep 8
echo AFTER_URL; $AB get url 2>&1 | mask
echo AFTER_STATE; $AB eval --stdin < "$DIR/diag-login.js" 2>&1 | mask
echo DIAG2_DONE