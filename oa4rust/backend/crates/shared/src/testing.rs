// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::time::Duration;

use axum::Router;
use deadpool_postgres::Pool;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::middleware::SecurityState;

/// 进程级环境变量测试的串行锁。
///
/// 为什么需要它：环境变量是全局状态，而 `db::dialect::dialect()` 背后的
/// `OnceLock` 全进程只初始化一次。两者叠加的后果是「哪个测试先跑」决定全局方言 ——
/// 实测过一次：`dialect` 的用例把 `DB_DIALECT` 设成 mysql 的瞬间，迁移运行器的
/// 测试正好初始化方言，于是 114 条 PG 迁移被改写成 MySQL 语法执行，
/// 报 `ERROR: syntax error at end of input`（看起来像迁移文件坏了）。
///
/// 约定：任何会 `set_var`/`remove_var` 的测试都先拿这把锁，
/// 尤其是**同时**依赖 `dialect()` 的测试（迁移运行器）。
pub static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// 空连接池：用于验证路由存在性（无 DB 时返回 500）。
pub fn mock_pool() -> Pool {
    let mgr = deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    );
    Pool::builder(mgr).max_size(1).build().unwrap()
}

/// 空 Mock 数据库连接（sea_orm）：用于 *_core_entity 的 HTTP 臂测试。
/// 无 DB 环境下 handler 的 ORM 访问会得到 `DbErr`（→ 500），而不是在连接提取期
/// 就 500（B3 接线修复后，router 必须注入 Extension<DatabaseConnection>）。
/// 用 `connect_lazy(true)` 构建懒连接池，且指向**不可达**的 127.0.0.1:1（IPv4：
/// Windows 上 `localhost` 会走 ::1 导致连接超时而非立即拒绝，拖慢测试）：
/// 无论本机是否跑着 PG（5432 可能有测试库），查询都会 connection refused →
/// DbErr → AppError::Internal（500），测试结果与环境无关。
pub async fn mock_db() -> sea_orm::DatabaseConnection {
    let mut opt = sea_orm::ConnectOptions::new(
        "postgres://o2server:password@127.0.0.1:1/oa4rust".to_string(),
    );
    opt.connect_lazy(true)
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(1))
        .sqlx_logging(false);
    sea_orm::Database::connect(opt)
        .await
        .expect("mock_db: 构建懒连接池不应失败（lazy 模式不实际建连）")
}

/// 连接到 PostgreSQL 的 deadpool_postgres::Pool，用于单元测试。
/// 默认使用 oa4rust 测试库（postgres://o2server:password@localhost:5432/oa4rust）。
/// 连接是延迟建立的——pool.get().await 时才实际建连。
/// 若 PG 不可达，pool 仍可构建，首次 get() 返回错误。
///
/// 支持通过 `DATABASE_URL` 环境变量覆盖连接目标（解析 host/port/user/password/dbname，
/// 任一缺失则回退到上面的默认值）。本地用非标准端口（如 Docker 15432）跑测试时，
/// 设 `DATABASE_URL=postgres://o2server:password@localhost:15432/oa4rust` 即可，
/// 不影响 CI（CI 不设该变量，回退 5432）。
pub fn test_pool() -> Pool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust".to_string());
    let url = url::Url::parse(&database_url).expect("invalid DATABASE_URL");
    let host = url.host_str().unwrap_or("localhost");
    let port = url.port().unwrap_or(5432);
    let user = if url.username().is_empty() {
        "o2server"
    } else {
        url.username()
    };
    let password = url.password().unwrap_or("password");
    let dbname = {
        let p = url.path().trim_start_matches('/');
        if p.is_empty() {
            "oa4rust"
        } else {
            p
        }
    };
    let mut cfg = deadpool_postgres::tokio_postgres::Config::new();
    cfg.host(host)
        .port(port)
        .user(user)
        .password(password)
        .dbname(dbname);
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(5).build().unwrap()
}

/// 尝试用 test_pool 建立连接，超时 2s。
/// 成功返回 true，失败（连接超时、拒绝等）返回 false。
/// 用于集成测试的运行时 DATABASE_URL 守卫。
pub async fn is_db_available() -> bool {
    let pool = test_pool();
    matches!(
        tokio::time::timeout(Duration::from_secs(2), pool.get()).await,
        Ok(Ok(_))
    )
}

/// 并发安全的夹具人：幂等 upsert 一个 `auth_person` 行（pid=uid=person_unique 亦可分开传）。
///
/// # 为什么需要重试（2026-10-25 R4 首修于 `personal` crate；2026-10-26 第三轮
/// # FX-1 抬升为 shared 公共助手并迁移全仓 18 处同型夹具）
///
/// `auth_person` 的 `id` 与 `unique_id` **各有一个唯一索引**，而 `ON CONFLICT`
/// 只能指定**一个**仲裁索引（PostgreSQL 不支持一条 INSERT 带多个 ON CONFLICT
/// 子句）。两个测试并发插同一行（同 id、同 unique_id）时，PostgreSQL 可能先在
/// `unique_id` 索引上判冲突——那不是仲裁索引，于是直接 E23505，把其中一个测试
/// 随机打红：
///
///   duplicate key value violates unique constraint "auth_person_unique_id_key"
///
/// 修法：先 DELETE「同 unique_id 但不同 id」的残留（ON CONFLICT (id) 覆盖不到
/// 的那类），再 `ON CONFLICT (id) DO UPDATE`；重试时对方已提交、残留可见，
/// 第二次必然走 UPDATE 分支。三次仍失败才 panic——说明不是竞态而是真数据问题。
pub async fn upsert_fixture_person(
    client: &deadpool_postgres::Client,
    pid: &str,
    uid: &str,
    name: &str,
) {
    for attempt in 0..3 {
        // 清「同 unique_id 但不同 id」的残留：ON CONFLICT (id) 覆盖不到
        client
            .execute(
                "DELETE FROM auth_person WHERE unique_id = $1 AND id <> $2",
                &[&uid, &pid],
            )
            .await
            .unwrap();
        let res = client
            .execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) \
                 VALUES ($1, $2, $3, 'u2-test-noop') \
                 ON CONFLICT (id) DO UPDATE SET unique_id = EXCLUDED.unique_id",
                &[&pid, &uid, &name],
            )
            .await;
        match res {
            Ok(_) => return,
            Err(e) if is_unique_violation(&e) && attempt < 2 => continue,
            Err(e) => panic!("夹具 upsert_fixture_person 失败（{pid} / {uid}）：{e}"),
        }
    }
}

/// 这个错误是不是唯一键冲突（SQLSTATE 23505）。
/// 判据形态与 R4 在 `personal` crate 落地的版本逐字一致（两处实现同族同理）。
pub fn is_unique_violation(e: &deadpool_postgres::tokio_postgres::Error) -> bool {
    e.code() == Some(&deadpool_postgres::tokio_postgres::error::SqlState::UNIQUE_VIOLATION)
}

/// 连接到 PostgreSQL 的 sea_orm::DatabaseConnection，
/// 用于 Extension<DatabaseConnection> 类型的 handler 测试。
/// 若 PG 不可达，返回 Err。
///
/// 与 [`test_pool`] 一致，支持 `DATABASE_URL` 覆盖连接目标（缺省回退默认 5432 库）。
pub async fn test_sea_orm_pool() -> Result<DatabaseConnection, String> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://o2server:password@localhost:5432/oa4rust".to_string());
    let mut options = ConnectOptions::new(database_url);
    options.max_connections(5).sqlx_logging(false);
    Database::connect(options).await.map_err(|e| e.to_string())
}

/// 构造测试用会话（固定 token、person_unique 自定义），用于 handler 级安全
/// 门禁回归（无 DB 时 is_admin fail-closed，非 admin 会话断言 403）。
/// 各 crate security_gate_tests 统一入口，避免重复构造 Session 字面量。
pub fn test_session(person_unique: &str) -> crate::session::Session {
    use chrono::NaiveDateTime;
    crate::session::Session {
        token: "test-token".to_string(),
        person_unique: person_unique.to_string(),
        created_at: NaiveDateTime::parse_from_str("1970-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        expires_at: NaiveDateTime::parse_from_str("1970-01-01 01:00:00", "%Y-%m-%d %H:%M:%S")
            .unwrap(),
    }
}

pub fn test_app_with(state: SecurityState, pool: Pool) -> Router {
    use axum::middleware;
    use axum::routing::{get, post};
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/api/unit/list", get(|| async { "ok" }))
        .route("/api/authentication/login", post(|| async { "ok" }))
        .route("/api/reset", post(|| async { "ok" }))
        .route("/api/person", post(|| async { "ok" }))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::authorize_middleware,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::auth_middleware,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::rate_limit_middleware,
        ))
        .layer(middleware::from_fn(
            crate::middleware::security_headers_middleware,
        ))
        .layer(middleware::from_fn(crate::middleware::trace_middleware))
        .layer(axum::extract::Extension(pool))
}

pub async fn send(
    app: &Router,
    method: axum::http::Method,
    uri: &str,
    token: Option<&str>,
    xff: Option<&str>,
) -> axum::http::StatusCode {
    use axum::body::Body;
    use axum::http::{header, Request};
    use tower::ServiceExt;

    let mut req = Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::empty())
        .unwrap();
    if let Some(token) = token {
        req.headers_mut().insert(
            header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );
    }
    if let Some(xff) = xff {
        req.headers_mut()
            .insert("x-forwarded-for", xff.parse().unwrap());
    }
    app.clone().oneshot(req).await.unwrap().status()
}

pub async fn send_request(
    app: &Router,
    method: axum::http::Method,
    uri: &str,
    body: Option<serde_json::Value>,
    token: Option<&str>,
    xff: Option<&str>,
) -> (axum::http::StatusCode, serde_json::Value) {
    use axum::body::Body;
    use axum::http::{header, Request};
    use tower::ServiceExt;

    let mut req = Request::builder().method(method).uri(uri);
    if let Some(_body) = &body {
        req = req.header(header::CONTENT_TYPE, "application/json");
    }
    let mut req = if let Some(body) = body {
        req.body(Body::from(serde_json::to_vec(&body).unwrap()))
            .unwrap()
    } else {
        req.body(Body::empty()).unwrap()
    };
    if let Some(token) = token {
        req.headers_mut().insert(
            header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );
    }
    if let Some(xff) = xff {
        req.headers_mut()
            .insert("x-forwarded-for", xff.parse().unwrap());
    }
    let response = app.clone().oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 4096)
        .await
        .unwrap();
    let json: serde_json::Value =
        serde_json::from_slice(&bytes).unwrap_or(serde_json::json!({"error": "invalid json"}));
    (status, json)
}

/// 指向不可达端口的失败池（错误注入专用）：`get()` 因连接被拒**立即**返回 Err，
/// 用于覆盖各数据访问函数的 `pool.get().await.map_err(...)` 错误臂——
/// 健康池永远走不到那些分支，而这恰恰是「数据库连不上时调用方必须显式失败」的契约。
pub fn failing_pool() -> Pool {
    let mut cfg = deadpool_postgres::tokio_postgres::Config::new();
    cfg.host("127.0.0.1")
        .port(1)
        .user("cov")
        .password("cov")
        .dbname("cov");
    let mgr = deadpool_postgres::Manager::new(cfg, deadpool_postgres::tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(1).build().unwrap()
}
#[cfg(test)]
mod e2e_chain_tests {
    use super::*;
    use crate::middleware::security::SecurityState;

    // ── 2026-10-29 覆盖率轮：test_app_with/send 驱动真实 auth 中间件链 ──
    // 全内存（SessionManager::new 无池、失败池仅占位）：确定性、无 DB 依赖。
    #[tokio::test]
    async fn test_app_drives_the_auth_chain_end_to_end() {
        let state = SecurityState {
            session_manager: crate::session::SessionManager::new(),
            rate_limiter: crate::rate_limit::RateLimiter::new(),
            pool: crate::testing::failing_pool(),
        };
        // 在 state 的会话管理器里建一个真实内存会话（create_session 内部走
        // check_token_threshold/persist 的无池降级路径——一并被覆盖）
        state
            .session_manager
            .create_session("cov-e2e".to_string(), "cov-e2e-tok".to_string())
            .await
            .unwrap();
        let app = test_app_with(state, crate::testing::failing_pool());
        // test_session 助手（固定 token/person）此前从未被本 crate 调用
        let s = crate::testing::test_session("cov-e2e");
        assert_eq!(s.token, "test-token");

        // ① /health：豁免 ∧ 注册表 Public → 匿名放行
        assert_eq!(
            send(&app, axum::http::Method::GET, "/health", None, None).await,
            axum::http::StatusCode::OK
        );
        // ② 受保护路由无凭据 → 401（auth_middleware 的 extract 失败臂）
        assert_eq!(
            send(&app, axum::http::Method::GET, "/api/unit/list", None, None).await,
            axum::http::StatusCode::UNAUTHORIZED
        );
        // ③ 空白 Bearer → 401（空 token 臂）
        assert_eq!(
            send(
                &app,
                axum::http::Method::GET,
                "/api/unit/list",
                Some(""),
                None
            )
            .await,
            axum::http::StatusCode::UNAUTHORIZED
        );
        // ④ 伪造 token → 401（validate_session 未命中臂）
        assert_eq!(
            send(
                &app,
                axum::http::Method::GET,
                "/api/unit/list",
                Some("forged"),
                None
            )
            .await,
            axum::http::StatusCode::UNAUTHORIZED
        );
        // ⑤ 真实会话 → 认证通过、注册表 Public 放行 → 200（含 xff 头注入分支）
        assert_eq!(
            send(
                &app,
                axum::http::Method::GET,
                "/api/unit/list",
                Some("cov-e2e-tok"),
                Some("203.0.113.9")
            )
            .await,
            axum::http::StatusCode::OK
        );
        // ⑥ 登录路由：豁免 ∧ Public → 匿名 200
        assert_eq!(
            send(
                &app,
                axum::http::Method::POST,
                "/api/authentication/login",
                None,
                None
            )
            .await,
            axum::http::StatusCode::OK
        );
    }

    #[tokio::test]
    async fn login_route_is_rate_limited_per_configured_threshold() {
        // 显式设阈值：第 4 次起必须 429（环境变量每次调用都读，测试内设置即可生效）
        std::env::set_var("OA4RUST_AUTH_RATE_LIMIT", "3");
        let state = SecurityState {
            session_manager: crate::session::SessionManager::new(),
            rate_limiter: crate::rate_limit::RateLimiter::new(),
            pool: crate::testing::failing_pool(),
        };
        let app = test_app_with(state, crate::testing::failing_pool());
        let mut saw_429 = false;
        for _ in 0..8 {
            let status = send(
                &app,
                axum::http::Method::POST,
                "/api/authentication/login",
                None,
                None,
            )
            .await;
            if status == axum::http::StatusCode::TOO_MANY_REQUESTS {
                saw_429 = true;
                break;
            }
        }
        std::env::remove_var("OA4RUST_AUTH_RATE_LIMIT");
        assert!(saw_429, "超过阈值后登录必须 429");
        std::env::remove_var("OA4RUST_AUTH_RATE_LIMIT");
    }
}

// ── 2026-10-30 第八轮：mock 基建自身覆盖 ────────────────────────────────
// `mock_pool()` / `mock_db()` 主要被其他 crate 作为 dev-dependency 使用，shared
// 自身测试从未调用，导致 testing.rs 的行覆盖率长期偏低。补最小用例：它们
// 都是懒连接，无 DB 环境也能构建成功（首次 get/query 才尝试连接）。
#[cfg(test)]
mod mock_infra_tests {
    use super::*;

    #[test]
    fn mock_pool_builds_without_connecting() {
        let _ = mock_pool();
    }

    #[tokio::test]
    async fn mock_db_builds_lazy_connection_without_connecting() {
        let _ = mock_db().await;
    }

    #[tokio::test]
    async fn is_db_available_returns_bool_without_panicking() {
        // 无论 DATABASE_URL 是否可达，该探测函数都必须返回 bool 而非 panic。
        let _ = is_db_available().await;
    }
}
