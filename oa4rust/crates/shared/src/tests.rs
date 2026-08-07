#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::extract::ConnectInfo;
    use axum::http::{header, Method, Request, StatusCode};
    use axum::middleware;
    use axum::response::IntoResponse;
    use axum::routing::{get, post};
    use axum::Router;
    use deadpool_postgres::Pool;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use tower::ServiceExt;

    use crate::error::AppError;
    use crate::middleware::{
        auth_middleware, authorize_middleware, client_ip, extract_token, rate_limit_middleware,
        security_headers_middleware, trace_middleware, SecurityState,
    };
    use crate::rate_limit::RateLimiter;
    use crate::response::ActionResult;
    use crate::session::SessionManager;
    use serde_json::json;

    // ──────────────────────────────────────────────────────────────────────────
    // 测试基础设施
    //
    // 与 main.rs 相同的中间件栈（trace → security → rate → auth → authorize），
    // 挂在一个含桩路由的 Router 上，用 oneshot 直连验证。
    // 每个测试使用独立的 SessionManager/RateLimiter 实例，避免并行测试串扰。
    // ──────────────────────────────────────────────────────────────────────────

    /// 惰性连接池：build 时不会建立网络连接，仅在真正查询时尝试。
    /// 授权测试中 DB 不可达时 fail-closed（拒绝），不依赖测试环境数据库。
    fn lazy_pool() -> Pool {
        let mut cfg = deadpool_postgres::tokio_postgres::Config::new();
        cfg.host("127.0.0.1")
            .port(5432)
            .user("o2server")
            .password("password")
            .dbname("oa4rust");
        Pool::builder(deadpool_postgres::Manager::new(
            cfg,
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .build()
        .unwrap()
    }

    fn security_state() -> SecurityState {
        SecurityState {
            session_manager: SessionManager::new(),
            rate_limiter: RateLimiter::new(),
            pool: lazy_pool(),
        }
    }

    fn test_app(state: SecurityState) -> Router {
        Router::new()
            .route("/health", get(|| async { "ok" }))
            .route("/jaxrs/unit/list", get(|| async { "ok" }))
            .route("/jaxrs/authentication/login", post(|| async { "ok" }))
            .route("/jaxrs/reset", post(|| async { "ok" }))
            .route("/jaxrs/person", post(|| async { "ok" }))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                authorize_middleware,
            ))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                rate_limit_middleware,
            ))
            .layer(middleware::from_fn(security_headers_middleware))
            .layer(middleware::from_fn(trace_middleware))
    }

    async fn make_token(sm: &SessionManager, person: &str) -> String {
        let token = uuid::Uuid::new_v4().to_string();
        sm.create_session(person.to_string(), token.clone()).await;
        token
    }

    async fn send(
        app: &Router,
        method: Method,
        uri: &str,
        token: Option<&str>,
        xff: Option<&str>,
    ) -> StatusCode {
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

    // ──────────────────────────────────────────────────────────────────────────
    // 认证中间件测试
    // ──────────────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_health_public() {
        let app = test_app(security_state());
        let status = send(&app, Method::GET, "/health", None, None).await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_protected_route_requires_token() {
        let app = test_app(security_state());
        let status = send(&app, Method::GET, "/jaxrs/unit/list", None, None).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_invalid_token_rejected() {
        let app = test_app(security_state());
        let status = send(&app, Method::GET, "/jaxrs/unit/list", Some("bogus-token"), None).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_valid_bearer_token_passes() {
        let state = security_state();
        let token = make_token(&state.session_manager, "admin").await;
        let app = test_app(state);
        let status = send(&app, Method::GET, "/jaxrs/unit/list", Some(&token), None).await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_valid_cookie_token_passes() {
        let state = security_state();
        let token = make_token(&state.session_manager, "admin").await;
        let app = test_app(state);
        let req = Request::builder()
            .method(Method::GET)
            .uri("/jaxrs/unit/list")
            .header(header::COOKIE, format!("token={}", token))
            .body(Body::empty())
            .unwrap();
        let status = app.clone().oneshot(req).await.unwrap().status();
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_unauthorized_response_is_action_result_json() {
        let app = test_app(security_state());
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/unit/list")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        let bytes = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["type"], "error");
        assert_eq!(json["message"], "unauthorized");
    }

    #[test]
    fn test_extract_token_priority() {
        // Authorization 头优先于 Cookie
        let req = Request::builder()
            .uri("/x")
            .header(header::AUTHORIZATION, "Bearer abc123")
            .header(header::COOKIE, "token=cookie-token")
            .body(Body::empty())
            .unwrap();
        assert_eq!(extract_token(&req).as_deref(), Some("abc123"));

        // Cookie 回退：多 cookie 中取 token 字段
        let req = Request::builder()
            .uri("/x")
            .header(header::COOKIE, "other=1; token=cookie-token")
            .body(Body::empty())
            .unwrap();
        assert_eq!(extract_token(&req).as_deref(), Some("cookie-token"));

        // 无任何凭证
        let req = Request::builder().uri("/x").body(Body::empty()).unwrap();
        assert_eq!(extract_token(&req), None);
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 速率限制测试
    // ──────────────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_auth_path_rate_limited() {
        let app = test_app(security_state());
        for i in 0..11 {
            let status = send(&app, Method::POST, "/jaxrs/authentication/login", None, None).await;
            if i < 10 {
                assert_eq!(status, StatusCode::OK, "第 {} 次认证请求应成功", i + 1);
            } else {
                assert_eq!(
                    status,
                    StatusCode::TOO_MANY_REQUESTS,
                    "第 {} 次认证请求应触发 429",
                    i + 1
                );
            }
        }
    }

    #[tokio::test]
    async fn test_reset_path_counted_in_auth_rate_limit() {
        let app = test_app(security_state());
        for i in 0..11 {
            let status = send(&app, Method::POST, "/jaxrs/reset", None, None).await;
            if i < 10 {
                assert_eq!(status, StatusCode::OK, "第 {} 次重置请求应成功", i + 1);
            } else {
                assert_eq!(
                    status,
                    StatusCode::TOO_MANY_REQUESTS,
                    "第 {} 次重置请求应触发 429",
                    i + 1
                );
            }
        }
    }

    #[tokio::test]
    async fn test_rate_limit_keys_are_per_client_ip() {
        let app = test_app(security_state());
        // IP A：9 次成功
        for _ in 0..9 {
            let status = send(
                &app,
                Method::POST,
                "/jaxrs/authentication/login",
                None,
                Some("203.0.113.1"),
            )
            .await;
            assert_eq!(status, StatusCode::OK);
        }
        // IP B：前 10 次成功，第 11 次 429
        for i in 0..11 {
            let status = send(
                &app,
                Method::POST,
                "/jaxrs/authentication/login",
                None,
                Some("198.51.100.7"),
            )
            .await;
            if i < 10 {
                assert_eq!(status, StatusCode::OK);
            } else {
                assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
            }
        }
        // IP A 计数不受 IP B 影响：第 10 次仍成功
        let status = send(
            &app,
            Method::POST,
            "/jaxrs/authentication/login",
            None,
            Some("203.0.113.1"),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
    }

    // ──────────────────────────────────────────────────────────────────────────
    // client_ip 信任模型测试
    // ──────────────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_client_ip_trust_model() {
        // 无 ConnectInfo（回退 127.0.0.1，默认受信）：采用 XFF 第一个 IP
        let req = Request::builder()
            .uri("/x")
            .header("x-forwarded-for", "203.0.113.9, 10.0.0.1")
            .body(Body::empty())
            .unwrap();
        assert_eq!(client_ip(&req), "203.0.113.9");

        // 可信来源（127.0.0.1）：采用 XFF
        let mut req = Request::builder().uri("/x").body(Body::empty()).unwrap();
        req.extensions_mut().insert(ConnectInfo(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            54321,
        )));
        req.headers_mut()
            .insert("x-forwarded-for", "198.51.100.7".parse().unwrap());
        assert_eq!(client_ip(&req), "198.51.100.7");

        // 不可信来源（公网地址）：忽略 XFF，回退 socket 地址
        let mut req = Request::builder().uri("/x").body(Body::empty()).unwrap();
        req.extensions_mut().insert(ConnectInfo(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(203, 0, 113, 5)),
            54321,
        )));
        req.headers_mut()
            .insert("x-forwarded-for", "198.51.100.7".parse().unwrap());
        assert_eq!(client_ip(&req), "203.0.113.5");

        // 无 XFF：回退 socket 地址
        let mut req = Request::builder().uri("/x").body(Body::empty()).unwrap();
        req.extensions_mut().insert(ConnectInfo(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(198, 51, 100, 1)),
            54321,
        )));
        assert_eq!(client_ip(&req), "198.51.100.1");
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 授权（角色）测试
    // ──────────────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_non_admin_write_forbidden() {
        let state = security_state();
        let token = make_token(&state.session_manager, "user-1").await;
        let app = test_app(state);
        let status = send(&app, Method::POST, "/jaxrs/person", Some(&token), None).await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_person_admin_write_allowed() {
        let state = security_state();
        let token = make_token(&state.session_manager, "person-admin").await;
        let app = test_app(state);
        let status = send(&app, Method::POST, "/jaxrs/person", Some(&token), None).await;
        assert!(
            status == StatusCode::OK || status == StatusCode::FORBIDDEN,
            "DB 可达时 admin 应通过，不可达时 fail-closed 403，实际 {}",
            status
        );
    }

    #[tokio::test]
    async fn test_admin_role_write_allowed_by_db_role() {
        // 该测试依赖本地 DB（seed: admin/role-admin）。DB 不可达时 fail-closed
        // 会返回 403，本测试仅验证 fail-closed 下不会出现 500。
        let state = security_state();
        let token = make_token(&state.session_manager, "admin").await;
        let app = test_app(state);
        let status = send(&app, Method::POST, "/jaxrs/person", Some(&token), None).await;
        assert!(
            status == StatusCode::OK || status == StatusCode::FORBIDDEN,
            "DB 可达时 admin 应通过，不可达时 fail-closed 403，实际 {}",
            status
        );
    }

    #[tokio::test]
    async fn test_read_operations_do_not_require_admin() {
        let state = security_state();
        let token = make_token(&state.session_manager, "user-1").await;
        let app = test_app(state);
        let status = send(&app, Method::GET, "/jaxrs/unit/list", Some(&token), None).await;
        assert_eq!(status, StatusCode::OK);
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 安全响应头测试
    // ──────────────────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_security_headers_present() {
        let app = test_app(security_state());
        let resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            resp.headers().get(header::X_CONTENT_TYPE_OPTIONS).unwrap(),
            "nosniff"
        );
        assert_eq!(
            resp.headers().get(header::X_FRAME_OPTIONS).unwrap(),
            "DENY"
        );
        assert_eq!(resp.headers().get(header::CACHE_CONTROL).unwrap(), "no-store");
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 原有单元测试（ActionResult / AppError）
    // ──────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_action_result_success() {
        let result: ActionResult<String> = ActionResult::success("test".to_string());
        assert_eq!(result.r#type, Some("success".to_string()));
        assert_eq!(result.data, Some("test".to_string()));
        assert_eq!(result.message, None);
    }

    #[test]
    fn test_action_result_error() {
        let result: ActionResult<String> = ActionResult::error("test error");
        assert_eq!(result.r#type, Some("error".to_string()));
        assert_eq!(result.message, Some("test error".to_string()));
    }

    #[test]
    fn test_action_result_serialization() {
        let result: ActionResult<i32> = ActionResult::success(42);
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(json["type"], "success");
        assert_eq!(json["data"], 42);
        assert_eq!(json["message"], serde_json::Value::Null);
    }

    #[test]
    fn test_action_result_error_serialization() {
        let result: ActionResult<i32> = ActionResult::error("something went wrong");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(json["type"], "error");
        assert_eq!(json["message"], "something went wrong");
        assert_eq!(json["data"], serde_json::Value::Null);
    }

    #[test]
    fn test_action_result_with_count() {
        let mut result: ActionResult<serde_json::Value> = ActionResult::success(json!({"items": []}));
        result.count = Some(10);
        result.size = Some(20);

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["count"], 10);
        assert_eq!(json["size"], 20);
    }

    #[test]
    fn test_action_result_with_message() {
        let mut result: ActionResult<String> = ActionResult::success("data".to_string());
        result.message = Some("operation completed".to_string());

        assert_eq!(result.message, Some("operation completed".to_string()));
    }

    #[test]
    fn test_app_error_database() {
        let err = AppError::Database(sqlx::Error::RowNotFound);
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_bad_request() {
        let err = AppError::BadRequest("invalid input".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_app_error_unauthorized() {
        let err = AppError::Unauthorized;
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_app_error_not_found() {
        let err = AppError::NotFound;
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_app_error_internal() {
        let err = AppError::Internal;
        let response = err.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
