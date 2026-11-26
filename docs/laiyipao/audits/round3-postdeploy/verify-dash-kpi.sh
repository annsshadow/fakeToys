#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN=$(grep -E '^DOMAIN=' "$KEYS"|head -1|sed 's/^DOMAIN=//;s/[[:space:]]*$//'|tr -d '\r')
S="lyp-postadmin"
AB="agent-browser --session $S"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"; }
$AB open "https://$DOMAIN/laiyipao-admin/#/admin/dashboard" >/dev/null 2>&1
sleep 3
echo "=== DASH KPI ==="
$AB eval --stdin < docs/laiyipao/audits/round3-postdeploy/probe-dash-kpi.js 2>&1 | mask
$AB screenshot docs/laiyipao/audits/round3-postdeploy/screenshots/post-admin-dashboard.png >/dev/null 2>&1
echo done