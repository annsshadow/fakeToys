#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round51-me-radar-growth"
ALIAS=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -Atc "select token from guest_token_aliases where user_id=10 and expires_at > now() order by expires_at desc limit 1" | tr -d '\r\n')
if [ -z "$ALIAS" ]; then echo NO_LIVE_ALIAS >&2; exit 1; fi
RESP=$(curl -sS -X POST http://127.0.0.1:8080/api/v1/auth/guest -H 'Content-Type: application/json' -d "{\"guest_token\":\"$ALIAS\"}")
ALIAS="" node "$D/data/gen-alias-init.js" "$D/data/auth-init.js" <<<"$RESP"
echo ALIAS_AUTH_INIT_WRITTEN
