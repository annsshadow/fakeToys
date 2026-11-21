#!/bin/bash
# 测试环境连通性探测（真值从 .local.keys 读入内存，所有输出脱敏）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E '^DOMAIN=' "$KEYS" | head -1 | sed 's/^DOMAIN=//;s/[[:space:]]*$//' | tr -d '\r')"
ADMIN_PASS="$(grep -E '^ADMIN_PASS=' "$KEYS" | head -1 | sed 's/^ADMIN_PASS=//;s/[[:space:]]*$//' | tr -d '\r')"
# 智能加协议（不回显真值）
case "$DOMAIN" in http://*|https://*) HOST="$DOMAIN";; *) HOST="http://$DOMAIN";; esac
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|$HOST|<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
echo "domain_len=${#DOMAIN} pass_len=${#ADMIN_PASS}"
echo "--- admin healthz ---"
curl -s -o /dev/null -w 'admin_root=%{http_code}\n' --max-time 20 "$HOST/laiyipao-admin/" | mask
curl -s -o /dev/null -w 'api_healthz=%{http_code}\n' --max-time 20 "$HOST/laiyipao-api/healthz" | mask
curl -s -o /dev/null -w 'api_readyz=%{http_code}\n' --max-time 20 "$HOST/laiyipao-api/readyz" | mask
curl -s -o /dev/null -w 'h5_root=%{http_code}\n' --max-time 20 "$HOST/laiyipao-h5/" | mask
echo "--- done (all values masked) ---"
