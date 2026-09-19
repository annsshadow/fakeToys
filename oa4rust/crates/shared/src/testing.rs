use std::time::Duration;

use axum::Router;
use deadpool_postgres::Pool;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::middleware::SecurityState;

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
pub fn test_pool() -> Pool {
    let mut cfg = deadpool_postgres::tokio_postgres::Config::new();
    cfg.host("localhost")
        .port(5432)
        .user("o2server")
        .password("password")
        .dbname("oa4rust");
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
pub async fn test_sea_orm_pool() -> Result<DatabaseConnection, String> {
    let mut options = ConnectOptions::new("postgres://o2server:password@localhost:5432/oa4rust");
    options.max_connections(5).sqlx_logging(false);
    Database::connect(options).await.map_err(|e| e.to_string())
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
