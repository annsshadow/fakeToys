#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
D="$ROOT/docs/laiyipao/audits/round53-home-pct-rounding"
ALIAS=$(docker exec lyp-qa-pg psql -U postgres -d laiyipao -Atc "select token from guest_token_aliases where user_id=10 and expires_at > now() order by expires_at desc limit 1" | tr -d '\r\n')
if [ -z "$ALIAS" ]; then echo NO_LIVE_ALIAS >&2; exit 1; fi
RESP=$(curl -sS -X POST http://127.0.0.1:8080/api/v1/auth/guest -H 'Content-Type: application/json' -d "{\"guest_token\":\"$ALIAS\"}")
ALIAS="" node "$D/data/gen-alias-init.js" "$D/data/auth-init.js" <<<"$RESP"
ME=$(node -e "let s='';process.stdin.on('data',d=>s+=d);process.stdin.on('end',()=>{const j=JSON.parse(s);process.stdout.write(j.access_token||'')})" <<<"$RESP")
if [ -z "$ME" ]; then echo NO_TOKEN >&2; exit 1; fi
node "$D/data/fetch-real-me.js" "$D/data/me-real.json" <<<"$RESP"
node "$D/data/gen-mocks.js" "$D/data/me-real.json" "$D/data/config-real.json" "$D/data"
echo R53_AUTH_AND_MOCKS_DONE
