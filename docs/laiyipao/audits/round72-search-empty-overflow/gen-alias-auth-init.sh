#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round72-search-empty-overflow"
GEN="$ROOT/docs/laiyipao/audits/round56-visual-walk/data/gen-alias-init.js"
TOKEN=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -Atc "select guest_token from users where id=10" | tr -d '\r\n')
if [ -z "$TOKEN" ]; then echo NO_GUEST_TOKEN >&2; exit 1; fi
RESP=$(curl -sS -X POST http://127.0.0.1:8080/api/v1/auth/guest -H 'Content-Type: application/json' -d "{\"guest_token\":\"$TOKEN\"}")
TOKEN="" node "$GEN" "$D/data/auth-init.js" <<<"$RESP"
echo R72_AUTH_DONE