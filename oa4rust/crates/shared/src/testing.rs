use axum::Router;
use deadpool_postgres::Pool;

use crate::middleware::SecurityState;

/// 空连接池：用于验证路由存在性（无 DB 时返回 500）。
pub fn mock_pool() -> Pool {
    let mgr = deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    );
    Pool::builder(mgr).max_size(1).build().unwrap()
}

pub fn test_app_with(state: SecurityState, pool: Pool) -> Router {
    use axum::middleware;
    use axum::routing::{get, post};
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/jaxrs/unit/list", get(|| async { "ok" }))
        .route("/jaxrs/authentication/login", post(|| async { "ok" }))
        .route("/jaxrs/reset", post(|| async { "ok" }))
        .route("/jaxrs/person", post(|| async { "ok" }))
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
        .layer(middleware::from_fn(crate::middleware::security_headers_middleware))
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
        req.headers_mut()
            .insert(header::AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
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

    let mut req = Request::builder()
        .method(method)
        .uri(uri);
    if let Some(body) = &body {
        req = req.header(header::CONTENT_TYPE, "application/json");
    }
    let mut req = if let Some(body) = body {
        req.body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap()
    } else {
        req.body(Body::empty()).unwrap()
    };
    if let Some(token) = token {
        req.headers_mut()
            .insert(header::AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
    }
    if let Some(xff) = xff {
        req.headers_mut()
            .insert("x-forwarded-for", xff.parse().unwrap());
    }
    let response = app.clone().oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 4096).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap_or(serde_json::json!({"error": "invalid json"}));
    (status, json)
}
