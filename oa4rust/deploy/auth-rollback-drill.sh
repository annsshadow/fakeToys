#!/usr/bin/env bash
# Auth rollback drill (Rust-side). Functional matrix against a live Rust server.
# Sanitized: never prints raw cookie/token values, only SHA-256 prefixes + attributes.
set -u
B=http://127.0.0.1:3000
O="http://127.0.0.1:3000"    # == APP_PUBLIC_ORIGIN
O_EVIL="http://evil.example"
JAR=/tmp/oa4rust-drill.cookies
rm -f "$JAR"
OUT="${1:-drill-results.txt}"
: > "$OUT"

last_cookie() { python - "$JAR" <<'PY'
import re,sys
try:
    c=open(sys.argv[1]).read()
except Exception:
    print(''); sys.exit()
m=re.findall(r'oa4rust_session\s+(\S+)', c)
print(m[-1] if m else '')
PY
}
sha() { printf '%s' "$1" | sha256sum | cut -c1-16; }
cookie_attrs() { grep -i '^set-cookie:' /tmp/dh | head -1 | grep -oi 'httponly\|secure\|samesite=[a-z]*\|path=/\|max-age=[0-9]*\|domain=' | tr '\n' ' '; }
token_sha() { local sc; sc=$(grep -i '^set-cookie:' /tmp/dh | head -1 | sed 's/^[Ss]et-[Cc]ookie: //'); local v="${sc#*=}"; v="${v%%;*}"; [ -n "$v" ] && [ "$v" != " " ] && [ "${sc%%=*}" = "oa4rust_session" ] && sha "$v" || echo none; }

# req METHOD PATH [COOKIE_HEADER|""] [ORIGIN|""] [EXTRA...]
# COOKIE_HEADER "" => no Cookie header at all (true Bearer-only / no-credential)
req() {
  local method="$1" path="$2" cookie="$3" origin="$4"; shift 4
  local args=(-s -X "$method" "$B$path" -D /tmp/dh -o /tmp/db -c "$JAR" -w '')
  [ -n "$cookie" ] && args+=(-H "Cookie: $cookie")
  [ -n "$origin" ] && args+=(-H "Origin: $origin")
  args+=("$@")
  curl "${args[@]}" >/dev/null 2>&1
  local st; st=$(head -1 /tmp/dh | awk '{print $2}')
  echo "status=$st cookie_attr=[$(cookie_attrs)] set_token_sha=$(token_sha) body_token_field=$(grep -c '"token"' /tmp/db 2>/dev/null || echo 0)"
}
post() { # login/refresh body helper
  req "$1" "$2" "$3" "$4" -H 'Content-Type: application/json' -d "$5"
}

echo "=== W4 auth rollback drill (Rust-side) $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" | tee -a "$OUT"

echo "-- 1. login (no credentials) -> Set-Cookie; body must NOT carry a token" | tee -a "$OUT"
post POST /jaxrs/authentication "" "" '{"credential":"it-login","password":"testpass123"}' | tee -a "$OUT"
LIVE="$(last_cookie)"
echo "   session token sha=$(sha "$LIVE") (value never printed)" | tee -a "$OUT"

echo "-- 2. current-user with session cookie -> 200, body no token" | tee -a "$OUT"
req GET /jaxrs/authentication/who "oa4rust_session=$LIVE" "" | tee -a "$OUT"

echo "-- 3. refresh via cookie rotates: old cookie then 401" | tee -a "$OUT"
OLD="$LIVE"
req POST /jaxrs/authentication/refresh "oa4rust_session=$LIVE" "$O" | tee -a "$OUT"
LIVE="$(last_cookie)"
echo "   old cookie (rotated out) now: " ; req GET /jaxrs/authentication/who "oa4rust_session=$OLD" "" | tee -a "$OUT"

echo "-- 4. refresh is cookie-only: valid Bearer (no Cookie header) -> 401" | tee -a "$OUT"
req POST /jaxrs/authentication/refresh "" "$O" -H "Authorization: Bearer $LIVE" | tee -a "$OUT"

echo "-- 5. logout idempotent (cookie-auth delete needs exact Origin to pass CSRF)" | tee -a "$OUT"
req DELETE /jaxrs/authentication "oa4rust_session=$LIVE" "$O" | tee -a "$OUT"
req DELETE /jaxrs/authentication "" "" | tee -a "$OUT"

echo "-- 6. invalid cookie never falls back to a valid Bearer -> 401" | tee -a "$OUT"
req GET /jaxrs/authentication/who "oa4rust_session=bogus" "" -H "Authorization: Bearer $LIVE" | tee -a "$OUT"

echo "-- 7. CSRF: cookie-auth writes require exact APP_PUBLIC_ORIGIN (probe = POST logout)" | tee -a "$OUT"
post POST /jaxrs/authentication "" "" '{"credential":"it-login","password":"testpass123"}' >/dev/null
LIVE="$(last_cookie)"
echo "   (cookie, no origin)       expect 403:" ; req POST /jaxrs/authentication/logout "oa4rust_session=$LIVE" "" | tee -a "$OUT"
echo "   (cookie, wrong origin)    expect 403:" ; req POST /jaxrs/authentication/logout "oa4rust_session=$LIVE" "$O_EVIL" | tee -a "$OUT"
echo "   (cookie, exact origin)    expect 200:" ; req POST /jaxrs/authentication/logout "oa4rust_session=$LIVE" "$O" | tee -a "$OUT"
post POST /jaxrs/authentication "" "" '{"credential":"it-login","password":"testpass123"}' >/dev/null
LIVE="$(last_cookie)"
echo "   (Bearer-only, no cookie)  expect 200 (credential requests exempt):" ; req POST /jaxrs/authentication/logout "" "" -H "Authorization: Bearer $LIVE" | tee -a "$OUT"

echo "-- 8. rollback simulation: logout (clear, cookie+origin) -> re-login recovers; revoked cookie stays dead" | tee -a "$OUT"
DEAD="$LIVE"
req DELETE /jaxrs/authentication "oa4rust_session=$DEAD" "$O" | tee -a "$OUT"
post POST /jaxrs/authentication "" "" '{"credential":"it-login","password":"testpass123"}' | tee -a "$OUT"
LIVE="$(last_cookie)"
echo "   re-login current-user (new session): " ; req GET /jaxrs/authentication/who "oa4rust_session=$LIVE" "" | tee -a "$OUT"
echo "   revoked pre-logout cookie (expect 401): " ; req GET /jaxrs/authentication/who "oa4rust_session=$DEAD" "" | tee -a "$OUT"

echo "=== end drill ===" | tee -a "$OUT"
