#!/bin/bash
set -e
D=docs/laiyipao/audits/round20-combat-efficiency-trends
TOK=$(curl -s -X POST http://127.0.0.1:8080/api/v1/admin/login -H 'Content-Type: application/json' -d '{"username":"admin","password":"admin12345"}' | sed -n 's/.*"access_token":"\([^"]*\)".*/\1/p')
[ -n "$TOK" ] || { echo NO_TOKEN; exit 1; }
sed "s|__R20_TOKEN__|$TOK|g" "$D/admin-probe.tpl.js" > "$D/admin-probe.js"
echo ADMIN_PROBE_READY TOKEN_LEN=${#TOK}
