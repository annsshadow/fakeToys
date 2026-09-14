# Auth rollback drill — Rust-side (2026-09-10)

Live, functional drill against the Rust server proving the HttpOnly-cookie auth
matrix and a Rust-only rollback/recovery cycle. Executed locally (dev
PostgreSQL, `AUTH_COOKIE_SECURE=false`, `APP_PUBLIC_ORIGIN=http://127.0.0.1:3000`).
Sanitized: no raw cookie/token values — only SHA-256 prefixes, attributes, and
HTTP statuses. Reproduce: `deploy/auth-rollback-drill.sh`.

## Environment
- Server: `target/debug/oa4rust` (workspace MSRV 1.85 toolchain), dev DB `oa4rust`.
- Test user: `it-login` (seeded bcrypt). Session TTL 900s for the drill.
- Time: 2026-09-10T08:42Z (functional run), 08:48Z (nginx -t).

## Functional matrix (observed via `deploy/auth-rollback-drill.sh`)
| # | Check | Expected | Observed |
|---|-------|----------|----------|
| 1 | login (no creds) sets `oa4rust_session`; JSON has no `token` | 200 + Set-Cookie, `body_token_field=0` | PASS (`sha bc26313d7cf6b329`) |
| 1a | cookie attrs host-only HttpOnly/Lax/Path=/ + Max-Age=TTL | `HttpOnly SameSite=Lax Path=/ Max-Age=900`, no `Domain` | PASS |
| 2 | current-user via cookie | 200, no `token` | PASS |
| 3 | refresh rotates; old cookie invalidated | new sha `6513400123fe49ea`, old cookie 401 | PASS |
| 4 | refresh is cookie-only (Bearer, no cookie) | 401 | PASS |
| 5 | logout idempotent, clears cookie `Max-Age=0` | 200, `Max-Age=0` | PASS |
| 6 | invalid cookie never falls back to a valid Bearer | 401 | PASS |
| 7 | CSRF on cookie writes: no-origin 403 / wrong-origin 403 / exact-origin 200 / Bearer-only 200 | as stated | PASS |
| 8 | rollback: logout clears, re-login recovers, revoked cookie stays dead | 200 + new sha `b414dae7c48eaabe`, revoked cookie 401 | PASS |

Notes surfaced by the drill:
- A cookie-authenticated logout/refresh is itself CSRF-gated: without an
  `Origin` matching `APP_PUBLIC_ORIGIN` it returns 403 (a real browser always
  sends Origin on credentialed subresource requests, so this is expected).
- `refresh` rejects Bearer-only requests (browser contract); a CLI/service that
  only carries Bearer cannot refresh — that is the intended split and the CLI
  refresh contract is deferred.
- Re-login does not revoke other live sessions of the same user; logout
  revokes the presented session. Full user revocation uses `safe/logout`.

## nginx config validation (`nginx -t`, docker `nginx:1.27-alpine`)
| Config | Result |
|--------|--------|
| `deploy/nginx-auth-routes-rust-drill.conf` | `syntax is ok` / `test is successful` |
| `deploy/nginx-auth-routes-java-drill.conf` | `syntax is ok` / `test is successful` |

Both proxy only `/jaxrs/(authentication|person|password|reset|secret)` to their
upstream; the Rust config points at `127.0.0.1:3000`, the Java config at the
o2server host. The switch is "replace the active auth-route config and
`nginx -t` then `nginx -s reload`".

## Java fallback — DOCUMENTED, NOT EXECUTED (EXTERNAL/BLOCKED)
Switching auth to the Java (o2server) side is an operational step that was not
performed here. Known degradations that must be accepted before it is done:
1. Rust `oa4rust_session` cookies are NOT valid on the Java side (separate
   token formats/stores) — users must re-authenticate.
2. Java restores JSON-token and legacy-cookie behavior (the thing this plan
   removes); CSRF/Origin enforcement is not equivalent to the Rust strict check.
3. Requires a running o2server (fixed-digest image `o2oa/o2server@sha256:0735...`)
   with its secret initialized.
4. OAuth/SSO provider configs must be re-validated on the Java side.

## Still BLOCKED (honest, not marked passed)
- Real OAuth/OIDC provider end-to-end (no live provider credentials locally).
- Java↔Rust session interop (architecturally separate token stores).
- Bearer-compatibility exit threshold: telemetry event
  `auth_compat="bearer"` now logs on protected routes; "zero" must be observed
  in a real traffic window before Bearer is retired. Not yet observed.
- GitHub required-checks / remote workflow activation (no push this round).

## Rollback drill manifest
See `auth-rollback-drill-2026-09-10.manifest.sha256` for the SHA-256 of each
archived artifact in this directory and the referenced deploy/ files.
