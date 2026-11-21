#!/bin/bash
KEYS="D:/WORKSPACE/fakeToys/laiyipao/deploy/.local.keys"
DOMAIN="$(grep -E '^DOMAIN=' "$KEYS" | head -1 | sed 's/^DOMAIN=//;s/[[:space:]]*$//' | tr -d '\r')"
ADMIN_PASS="$(grep -E '^ADMIN_PASS=' "$KEYS" | head -1 | sed 's/^ADMIN_PASS=//;s/[[:space:]]*$//' | tr -d '\r')"
mask(){ sed -e "s|$DOMAIN|<YOUR_TEST_DOMAIN>|g" -e "s|https://$DOMAIN|https://<YOUR_TEST_DOMAIN>|g" -e "s|http://$DOMAIN|http://<YOUR_TEST_DOMAIN>|g" -e "s|$ADMIN_PASS|<ADMIN_PASS>|g"; }
probe(){ local label="$1" url="$2"; curl -sL -k -o /dev/null -w "${label}: code=%{http_code} redirect=%{num_redirects} final=%{url_effective}\n" --max-time 25 "$url" | mask; }
echo "--- https endpoints (follow redirects) ---"
probe admin_root "https://$DOMAIN/laiyipao-admin/"
probe api_healthz "https://$DOMAIN/laiyipao-api/healthz"
probe api_readyz "https://$DOMAIN/laiyipao-api/readyz"
probe h5_root "https://$DOMAIN/laiyipao-h5/"
echo "--- api config endpoint ---"
probe api_config "https://$DOMAIN/laiyipao-api/api/v1/config"
echo "--- done ---"
