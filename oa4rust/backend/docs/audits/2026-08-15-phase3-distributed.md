# OA4RUST Phase 3 — Distributed / Horizontal-Scaling Readiness Audit

**Date:** 2026-08-15
**Scope:** READ-ONLY feasibility assessment. No `.rs` or `Cargo.toml` modified; no `cargo build`/`cargo test` run.
**Question:** Can `oa4rust` today run **multiple replicas behind a load balancer**? What must change?

---

## 1. Current state (evidence-based, with `file:line`)

### 1.1 `SessionManager` — HYBRID (in-memory cache + Postgres persistence) ✅ mostly multi-replica safe

`crates/shared/src/session.rs`:

- In-memory cache field:
  - `session.rs:30` — `pub sessions: Arc<RwLock<std::collections::HashMap<String, Session>>>`
- `with_pool` **does** persist to Postgres (pool is stored and used):
  - `session.rs:51-57` — `pub fn with_pool(pool: Pool) -> Self` sets `pool: Some(pool)`.
  - `session.rs:109-123` — `create_session` does `INSERT INTO auth_session (token, person_id, expires_at, created_at) ... ON CONFLICT (token) DO UPDATE SET expires_at = $3`.
  - `session.rs:174-211` — `validate_session` **falls back to a DB lookup on cache miss** (`SELECT ... FROM auth_session WHERE token = $1 AND expires_at > NOW()`).
  - `session.rs:218-229` — `remove_session` deletes from both the in-memory map and the DB (`DELETE FROM auth_session WHERE token = $1`).
  - `session.rs:235-264` — `remove_sessions_by_person` batch-deletes from both memory and DB.
- Cross-instance logout invalidation via DB `auth_token_threshold` table (polled, not pushed):
  - `session.rs:132-152` — `check_token_threshold` reads `auth_token_threshold` from DB.
  - `session.rs:274-326` — `broadcast_logout` reads the threshold and purges local cache entries older than it.
- Production wiring confirms persistence is active:
  - `src/main.rs:131` — `let session_manager = SessionManager::with_pool(pool.clone());`

**Verdict:** Sessions are **DB-backed**; create/validate/logout already work across replicas. The in-memory `sessions` map is a per-replica **read cache** only. Sessions do **not** require sticky sessions.

⚠️ **Caveat (see R6):** on a cache hit, `validate_session` returns without re-querying the DB (`session.rs:161-169`) — it only consults `auth_token_threshold` (the *safe-logout* path), not a single-session removal on another replica. So a **single-session logout** performed on replica A is **not** proactively honored on replica B until that token expires. This is a minor cross-replica correctness gap, not a blocker.

### 1.2 `RateLimiter` — PURE IN-MEMORY ❌ NOT shared

`crates/shared/src/rate_limit.rs`:

- `rate_limit.rs:20` — `pub attempts: Arc<RwLock<std::collections::HashMap<String, Vec<Instant>>>>`
- `rate_limit.rs:35-41` — a per-instance background `tokio::spawn` cleanup task.
- `rate_limit.rs:60-77` — `check_rate_limit` counts using `Instant::now()` (a **per-process monotonic clock**); there is **no** Redis/DB/shared store.
- `src/main.rs:132` — `let rate_limiter = RateLimiter::new();` (production path).

**Verdict:** Rate limiting is **fully per-replica**. Behind an LB, the effective global limit is `N × per-replica limit` (N = replica count). This is a **hard blocker** for correct multi-replica rate limiting.

### 1.3 Other in-memory stateful components (grep of `lazy_static`/`once_cell`/`RwLock`/`Mutex`/`HashMap`/`broadcast`)

All of the following are **process-local** (some are process-wide `OnceLock` globals). Each breaks multi-replica unless the request that *writes* the state and the request that *reads* it land on the same replica.

| # | Component | Location | Shared? | Type | Impact under LB |
|---|---|---|---|---|---|
| C | OAuth CSRF `state` store | `crates/auth/src/oauth.rs:38` (`OAUTH_STATES` `OnceLock<Mutex<HashMap>>`) | No (process-global) | OAuth state | Third-party login broken: state minted on A, callback on B → rejected |
| C | PKCE verifier store | `crates/auth/src/oauth.rs:81-83` (`pkce_store` `OnceLock<Mutex<HashMap<String,PkceEntry>>>`) | No (process-global) | PKCE | Same as above |
| D | WeChat `access_token` cache | `crates/auth/src/mpweixin.rs:74-75` (`WECHAT_ACCESS_TOKEN` `OnceLock<Mutex<(String,Instant)>>`) | No (process-global) | Token cache | WeChat token is appid-global; two replicas re-fetch → mutual invalidation → intermittent `40001` |
| E | SMS code store | `crates/auth/src/lib.rs:326` (`CodeStore` `Mutex<HashMap>`), global via `code_store()` `OnceLock` at `lib.rs:396` | No (process-global) | OTP | SMS code issued on A, verified on B → login fails |
| E | Temp token store (2FA bind) | `crates/auth/src/lib.rs:411` (`TempTokenStore` `Mutex<HashMap>`) | No (process-global) | Temp token | 2FA step broken across replicas |
| F | Captcha store | `crates/auth/src/captcha.rs:48` (`CaptchaStore` `Mutex<HashMap>`; comment `captcha.rs:47` "运行时使用全局单例") | No (process-global) | Captcha | Captcha generated on A, verified on B → fails |
| G | Scan-login bind store | `crates/auth/src/bind.rs:39` (`BindStore` `Mutex<HashMap>`) | No (process-global) | Bind meta | QR meta created on A, polled on B → not found |
| H | Password-reset code store | `crates/personal_extend/src/password.rs:42-43` (`ResetCodeStore` `Arc<RwLock<HashMap>>`) | No (per-instance) | OTP | Reset code issued on A, verified on B → fails |
| I | Password-reset code store | `crates/personal/src/reset.rs:35-36` (`ResetCodeStore` `Arc<RwLock<HashMap>>`) | No (per-instance) | OTP | Same as H |
| J | BBS token store | `crates/bbs_assemble_control/src/lib.rs:23-25` (`thread_local! static TOKEN_STORE: Mutex<HashMap<String,String>>`), used by `login` (`lib.rs:899`) / `logout` (`lib.rs:919`) | No — **per-thread** | Session token | **Broken even single-instance multi-threaded**; also broken under LB |
| K | Scheduled jobs | `crates/program_center/src/lib.rs:2687,2720,2733` (`x_program_schedule` w/ `cron_expression`, `server_node`); no in-process executor observed | N/A | Job table | No active executor found in this audit; if one is added it must be single-leader/`server_node`-pinned to avoid duplicate fire |

**Not a concern (stateless singletons):** `ai_assemble_control` `CLIENT` (`OnceLock<reqwest::Client>`, `lib.rs:1066`), `auth` HTTP client `OnceLock`s (`welink.rs:8`, `qiyeweixin.rs:9`, `oauth.rs:32`, `mpweixin.rs:32`) — these hold no mutable shared state.

**No websocket connection registries found:** grep for `axum::extract::ws`, `tungstenite`, `tokio_tungstenite`, `WebSocket` returned nothing. No `broadcast`/`mpsc` channel-based in-memory pub/sub registries found beyond the rate-limiter's internal cleanup task.

**No shared-state backend exists today:** grep across all `Cargo.toml` for `redis`/`redb`/`moka`/`memcached`/`etcd`/`consul` returned **no matches**. There is currently **zero** shared K/V or cache infrastructure.

---

## 2. Gaps (what prevents N-replica deployment today)

1. **Rate limiter is per-replica** (1.2) → global limit is `N ×` intended; trivially bypassed by spreading requests across replicas.
2. **Auth short-lived state is process-local** (C, D, E, F, G, H, I) → every interactive auth flow (OAuth/PKCE, captcha, SMS OTP, 2FA temp token, scan-login, password reset) breaks unless both legs of the flow hit the same replica.
3. **WeChat token race** (D) → correctness bug even at modest scale; replicas invalidate each other's token.
4. **BBS `TOKEN_STORE` is `thread_local!`** (J) → pre-existing defect: not shared across worker threads *within one process*, and certainly not across replicas. Login/logout in `bbs_assemble_control` are unreliable today.
5. **Session cross-replica cache staleness** (R6) → single-session logout not honored on other replicas until expiry.
6. **No shared-state infra** (Redis/DB cache) exists → all fixes above require introducing one.
7. **Future scheduled-job executor** (K) → `x_program_schedule` schema supports `server_node` pinning, but if an in-process runner is introduced it must be leader-elected or it will double-fire across replicas.

---

## 3. Recommended approach

**Decision: prefer full shared-state (Redis) over sticky sessions.** Sticky sessions alone are *insufficient and fragile* here — several flows (OAuth state, PKCE, captcha, OTP, bind meta) require *write-then-read* consistency, and stickiness only helps if every request in a flow is perfectly pinned (which breaks on reconnect, LB rebalance, or any cross-flow token reuse such as the BBS `TOKEN_STORE`). A shared store makes all replicas equivalent and removes the dependency on pinning.

Phased plan:

- **Phase 3a — Shared-state foundation (new infra dependency).** Add a Redis client (e.g. `redis`/`deadpool-redis`) with env config + pool, behind a `Cache`/`SharedStore` trait. Implement an in-memory backend for dev/tests and a Redis backend for prod. This is the **only** new runtime dependency required.
- **Phase 3b — Externalize short-lived auth state to Redis (TTL-backed).** Move RateLimiter to a Redis **sliding-window**/`INCR`+`EXPIRE` or token-bucket (`rate_limit.rs:20,60-77`); move captcha (`captcha.rs:48`), bind (`bind.rs:39`), OAuth state (`oauth.rs:38`), PKCE (`oauth.rs:81`), SMS code (`lib.rs:326` + `code_store()` global), temp token (`lib.rs:411`), and both reset-code stores (`personal_extend/src/password.rs:42`, `personal/src/reset.rs:35`) into Redis with appropriate TTLs. This closes gaps C–I.
- **Phase 3c — WeChat token single-flight (D).** Store the token in Redis with a coordinated refresh lock (single-flight / `SET NX` claim) so only one replica refreshes at a time; eliminates mutual invalidation.
- **Phase 3d — Sessions (1.1).** Keep Postgres as the source of truth (already done). To close R6, either (a) shorten the in-memory cache TTL and re-validate against DB on every request, or (b) back the cache with Redis and use Redis pub/sub to actively purge on `remove_session`/`remove_sessions_by_person`. Sessions then need **no** sticky sessions.
- **Phase 3e — Fix BBS `TOKEN_STORE` (J).** Replace the `thread_local!` map (`bbs_assemble_control/src/lib.rs:23-25`) with DB or Redis; this simultaneously fixes a latent single-instance multi-thread bug.
- **Phase 3f — Scheduled jobs (K).** When an executor is added, make it single-leader (Redis lock / `server_node` claim row) so `x_program_schedule` jobs fire exactly once.

---

## 4. Risk register

| ID | Risk | Severity | Likelihood today | Remediation cost | Notes |
|---|---|---|---|---|---|
| R1 | Rate limit under-enforced behind LB (N× effective) | High | Certain under LB | Low–Med | Redis sliding window |
| R2 | OAuth/PKCE login broken under LB | High | Certain under LB | Med | Redis state+PKCE (C) |
| R3 | Captcha / bind / SMS-OTP / temp-token / reset-code flows broken under LB | High | Certain under LB | Med | Redis stores (E,F,G,H,I) |
| R4 | WeChat `access_token` mutual invalidation → intermittent `40001` | High | Likely (any ≥2 replicas) | Low–Med | Redis single-flight (D) |
| R5 | BBS `TOKEN_STORE` `thread_local!` → login/logout broken (even single-instance multi-thread) | High | Certain | Low | Replace with DB/Redis (J); also a pre-existing bug |
| R6 | Single-session logout not honored on other replicas until token expiry | Medium | Certain under LB | Low | Redis cache + pub/sub purge (1.1 caveat) |
| R7 | Duplicate scheduled-job execution once an executor is added | Medium | Future | Med | Leader election / `server_node` pin (K) |
| R8 | No shared-state infra in workspace | Medium | n/a (prerequisite) | Med | Introduce Redis (3a) |
| R9 | Redis becomes a new SPOF / network-latency dependency | Low–Med | n/a | Med | HA Redis (replica/sentinel); in-memory fallback for dev |

---

## 5. Conclusion

Today `oa4rust` is **single-instance by construction**: `SessionManager` is the *only* component that is already DB-backed and multi-replica tolerant, while `RateLimiter` and a cluster of process-local auth-state stores (OAuth state, PKCE, captcha, SMS/reset OTP, scan-login bind, WeChat token) plus a `thread_local!` BBS token store are all in-memory and will silently break behind a load balancer. A correct multi-replica deployment requires introducing a shared store (Redis) for rate limiting and short-lived auth state, externalizing the BBS token store, and either accepting or closing the small session-logout staleness window; sticky sessions alone are insufficient. The good news is that the persistence layer (Postgres) and the session cross-instance invalidation pattern (`auth_token_threshold`) already exist, so the work is concentrated, well-scoped, and does not require re-architecting the request path.
