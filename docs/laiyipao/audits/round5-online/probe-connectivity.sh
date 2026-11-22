#!/bin/bash
# t44 线上连通性探测（真值内存读取，输出全脱敏）
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E "^DOMAIN=" "$KEYS" | head -1 | sed "s/^DOMAIN=//;s/[[:space:]]*$//" | tr -d "\r")"
if [ -z "$DOMAIN" ]; then echo "DOMAIN_MISSING"; exit 2; fi
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g"; }
echo "domain_len=${#DOMAIN}"
for path in /laiyipao-admin/ /laiyipao-h5/ /laiyipao-api/healthz /laiyipao-api/readyz /laiyipao-api/api/v1/config; do
  code=$(curl -s -o /dev/null -w "%{http_code}" --max-time 12 -k "https://$DOMAIN$path" 2>/dev/null)
  echo "$path -> $code" | mask
done