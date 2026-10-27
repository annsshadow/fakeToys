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

/// 尝试连接默认测试 Redis（redis://127.0.0.1:6379），超时 2s。
///
/// Redis 在本项目中是**可选依赖**（见 `crate::redis` 顶部注释：不可达时降级为
/// 进程内内存实现）。CI 的 unit-tests job 只起了 postgres service，没有任何
/// Redis 服务，因此凡是以「能连上 Redis」为前提的测试都必须在缺服务时优雅跳过，
/// 否则会断言失败 -> panic -> `cargo test` 以 exit code 101 结束。
///
/// 与 `is_db_available()` 对称，供测试做运行时守卫。
pub async fn is_redis_available() -> bool {
    let url =
        crate::redis::redis_url_from_env().unwrap_or_else(|| "redis://127.0.0.1:6379".to_string());
    matches!(
        tokio::time::timeout(
            Duration::from_secs(2),
            crate::redis::RedisPool::from_url(&url)
        )
        .await,
        Ok(Ok(_))
    )
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
