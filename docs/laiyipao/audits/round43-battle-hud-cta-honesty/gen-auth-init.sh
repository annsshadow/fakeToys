#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round43-battle-hud-cta-honesty"
GT=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -Atc "select guest_token from users where id=10" | tr -d '\r\n')
RESP=$(curl -s -X POST http://127.0.0.1:8080/api/v1/auth/guest -H 'Content-Type: application/json' -d "{\"guest_token\":\"$GT\"}")
GT="$GT" node "$D/gen-auth-init.js" "$D/data/auth-init.js" <<<"$RESP"
echo AUTH_INIT_WRITTEN
