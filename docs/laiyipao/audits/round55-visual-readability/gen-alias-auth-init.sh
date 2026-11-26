#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round55-visual-readability"
GUEST=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -Atc "select guest_token from users where id=10" | tr -d '\r\n')
if [ -z "$GUEST" ]; then echo NO_QA_GUEST >&2; exit 1; fi
RESP=$(curl -sS -X POST http://127.0.0.1:8080/api/v1/auth/guest -H 'Content-Type: application/json' -d "{\"guest_token\":\"$GUEST\"}")
GUEST="" node "$D/data/gen-alias-init.js" "$D/data/auth-init.js" <<<"$RESP"
echo R55_AUTH_DONE
