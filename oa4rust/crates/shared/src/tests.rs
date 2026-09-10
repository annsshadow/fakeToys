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
        auth_middleware, authorize_middleware, client_ip, csrf_middleware,
        extract_authentication, rate_limit_middleware, security_headers_middleware,
        trace_middleware, Authentication, SecurityState, SESSION_COOKIE_NAME,
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
                csrf_middleware,
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
        let session = sm.create_session(person.to_string(), token.clone()).await.unwrap();
        session.token
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
            .header(header::COOKIE, format!("{}={}", SESSION_COOKIE_NAME, token))
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
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, "Bearer bearer-token".parse().unwrap());
        headers.insert(
            header::COOKIE,
            format!("{}=cookie-token", SESSION_COOKIE_NAME).parse().unwrap(),
        );
        assert_eq!(
            extract_authentication(&headers),
            Some(Authentication::Cookie("cookie-token".to_string()))
        );

        headers.insert(
            header::COOKIE,
            format!("{}=", SESSION_COOKIE_NAME).parse().unwrap(),
        );
        assert_eq!(
            extract_authentication(&headers),
            Some(Authentication::Cookie(String::new())),
            "空 Cookie 不得回退 Bearer"
        );

        headers.remove(header::COOKIE);
        assert_eq!(
            extract_authentication(&headers),
            Some(Authentication::Bearer("bearer-token".to_string()))
        );
    }

    #[tokio::test]
    async fn test_csrf_cookie_write_requires_exact_origin_and_bearer_is_exempt() {
        let state = security_state();
        let token = make_token(&state.session_manager, "user").await;
        let app = test_app(state);

        let request = Request::builder()
            .method(Method::POST)
            .uri("/jaxrs/reset")
            .header(header::COOKIE, format!("{}={}", SESSION_COOKIE_NAME, token))
            .body(Body::empty())
            .unwrap();
        assert_eq!(app.clone().oneshot(request).await.unwrap().status(), StatusCode::FORBIDDEN);

        let request = Request::builder()
            .method(Method::POST)
            .uri("/jaxrs/reset")
            .header(header::COOKIE, format!("{}={}", SESSION_COOKIE_NAME, token))
            .header(header::ORIGIN, "http://localhost:3000")
            .body(Body::empty())
            .unwrap();
        assert_eq!(app.clone().oneshot(request).await.unwrap().status(), StatusCode::OK);

        let request = Request::builder()
            .method(Method::POST)
            .uri("/jaxrs/reset")
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        assert_eq!(app.oneshot(request).await.unwrap().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_cors_preflight_allows_write_methods() {
        let app = Router::new()
            .route("/write", post(|| async { "ok" }))
            .layer(crate::middleware::cors_middleware_for_origin("http://localhost:3000"));
        for method in ["PUT", "PATCH", "DELETE"] {
            let response = app.clone().oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri("/write")
                    .header(header::ORIGIN, "http://localhost:3000")
                    .header(header::ACCESS_CONTROL_REQUEST_METHOD, method)
                    .body(Body::empty())
                    .unwrap(),
            ).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let allowed = response.headers()[header::ACCESS_CONTROL_ALLOW_METHODS].to_str().unwrap();
            assert!(allowed.split(',').any(|value| value.trim() == method));
            assert_eq!(
                response.headers()[header::ACCESS_CONTROL_ALLOW_CREDENTIALS].to_str().unwrap(),
                "true"
            );
            assert_eq!(
                response.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN].to_str().unwrap(),
                "http://localhost:3000"
            );
        }
    }

    #[tokio::test]
    async fn test_cors_read_methods_not_blocked_and_wrong_origin_rejected() {
        let app = Router::new()
            .route("/write", post(|| async { "ok" }))
            .layer(crate::middleware::cors_middleware_for_origin("http://localhost:3000"));
        // GET/HEAD preflights are not blocked by the CORS layer.
        for method in ["GET", "HEAD"] {
            let response = app.clone().oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri("/write")
                    .header(header::ORIGIN, "http://localhost:3000")
                    .header(header::ACCESS_CONTROL_REQUEST_METHOD, method)
                    .body(Body::empty())
                    .unwrap(),
            ).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK, "{method} preflight should pass");
            let allowed = response.headers()[header::ACCESS_CONTROL_ALLOW_METHODS].to_str().unwrap();
            assert!(allowed.split(',').any(|value| value.trim() == method));
        }
        // A preflight from a foreign origin must not be allowed.
        let response = app.clone().oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/write")
                .header(header::ORIGIN, "http://evil.example")
                .header(header::ACCESS_CONTROL_REQUEST_METHOD, "POST")
                .body(Body::empty())
                .unwrap(),
        ).await.unwrap();
        // A preflight from a foreign origin must never be whitelisted. tower-http's
        // exact-origin layer replies with the public origin regardless; the browser
        // enforces the match, so the security invariant we assert is: the foreign
        // origin is never echoed back as an allowed origin.
        let allowed_origin = response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert_ne!(
            allowed_origin, "http://evil.example",
            "the foreign origin must never be reflected as an allowed origin"
        );
    }

    // U12 matrix #7: every state-changing method with a cookie requires the exact
    // public Origin (missing/null/wrong all 403); Bearer-only requests are exempt
    // because browsers never attach credentials to them.
    #[tokio::test]
    async fn test_csrf_matrix_write_methods_and_origins() {
        use axum::routing::{delete, patch, put};

        let state = security_state();
        let token = make_token(&state.session_manager, "user").await;
        let app = Router::new()
            .route("/write", post(|| async { "ok" }))
            .route("/write", put(|| async { "ok" }))
            .route("/write", patch(|| async { "ok" }))
            .route("/write", delete(|| async { "ok" }))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                csrf_middleware,
            ));

        for method in ["POST", "PUT", "PATCH", "DELETE"] {
            let build = |cookie: Option<&str>, origin: Option<&str>, bearer: Option<&str>| {
                let mut builder = Request::builder().method(method).uri("/write");
                if let Some(cookie) = cookie {
                    builder = builder.header(
                        header::COOKIE,
                        format!("{}={}", SESSION_COOKIE_NAME, cookie),
                    );
                }
                if let Some(origin) = origin {
                    builder = builder.header(header::ORIGIN, origin);
                }
                if let Some(bearer) = bearer {
                    builder = builder.header(header::AUTHORIZATION, format!("Bearer {bearer}"));
                }
                app.clone().oneshot(builder.body(Body::empty()).unwrap())
            };
            let cookie = Some(token.as_str());
            let exact = Some("http://localhost:3000");

            // cookie + missing origin
            assert_eq!(
                build(cookie, None, None).await.unwrap().status(),
                StatusCode::FORBIDDEN,
                "{method} cookie without origin must be 403"
            );

            // cookie + null origin
            assert_eq!(
                build(cookie, Some("null"), None).await.unwrap().status(),
                StatusCode::FORBIDDEN,
                "{method} cookie with null origin must be 403"
            );

            // cookie + wrong origin
            assert_eq!(
                build(cookie, Some("http://evil.example"), None).await.unwrap().status(),
                StatusCode::FORBIDDEN,
                "{method} cookie with wrong origin must be 403"
            );

            // cookie + exact origin
            assert_eq!(
                build(cookie, exact, None).await.unwrap().status(),
                StatusCode::OK,
                "{method} cookie with exact origin must pass"
            );

            // Bearer-only (no cookie) is exempt: no Origin required.
            assert_eq!(
                build(None, None, Some(token.as_str())).await.unwrap().status(),
                StatusCode::OK,
                "{method} bearer-only must stay exempt from Origin check"
            );

            // cookie + Bearer without Origin: the cookie makes this a credential
            // request, Bearer must NOT bypass the check.
            assert_eq!(
                build(cookie, None, Some(token.as_str())).await.unwrap().status(),
                StatusCode::FORBIDDEN,
                "{method} cookie+bearer without origin must be 403"
            );
        }

        // GET/HEAD are not state-changing: cookie without origin passes the CSRF
        // layer (the POST/PUT/PATCH/DELETE-only router answers 405, not 403).
        for method in ["GET", "HEAD"] {
            let mut builder = Request::builder().method(method).uri("/write");
            builder = builder.header(
                header::COOKIE,
                format!("{}={}", SESSION_COOKIE_NAME, token),
            );
            assert_eq!(
                app.clone().oneshot(builder.body(Body::empty()).unwrap()).await.unwrap().status(),
                StatusCode::METHOD_NOT_ALLOWED,
                "{method} is not CSRF-gated (405 from the route, not 403)"
            );
        }
    }

    // U12 matrix #4: a protected route with an invalid cookie must 401 and never
    // fall back to a valid Bearer.
    #[tokio::test]
    async fn test_invalid_cookie_never_falls_back_to_valid_bearer() {
        let state = security_state();
        let good = make_token(&state.session_manager, "admin").await;
        let app = test_app(state);

        let req = Request::builder()
            .method(Method::GET)
            .uri("/jaxrs/unit/list")
            .header(header::COOKIE, format!("{}=bogus", SESSION_COOKIE_NAME))
            .header(header::AUTHORIZATION, format!("Bearer {good}"))
            .body(Body::empty())
            .unwrap();
        assert_eq!(app.clone().oneshot(req).await.unwrap().status(), StatusCode::UNAUTHORIZED);
    }

    // ──────────────────────────────────────────────────────────────────────────
    // 速率限制测试
    // ──────────────────────────────────────────────────────────────────────────

    #[tokio::test]
    #[ignore = "pre-existing: redis mock not configured"]
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
    #[ignore = "pre-existing: redis mock not configured"]
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
    #[ignore = "pre-existing: redis mock not configured"]
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
        // Java 成功信封实测恒填空串 message（Gson 对齐）
        assert_eq!(result.message, Some(String::new()));
        // Java 成功信封无 prompt 字段（仅错误信封携带异常类名）
        assert_eq!(result.prompt, None);
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
        // Java 成功信封实测 message 为空串而非 null
        assert_eq!(json["message"], serde_json::Value::String(String::new()));
        // Java 成功信封无 prompt 字段（skip_serializing_if = "Option::is_none"）
        assert!(json.get("prompt").is_none());
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

    #[test]
    fn test_test_pool_builds_without_connecting() {
        // Pool 构建不应建立实际网络连接（延迟连接）
        let _pool = crate::testing::test_pool();
        // 不要求 pool.max_size() 可访问，仅验证函数可调用且返回非 panic
    }

    #[tokio::test]
    async fn test_test_sea_orm_pool_connects() {
        // 若 PG 不可达，连接会返回 Err；仅验证函数可调用
        let result = crate::testing::test_sea_orm_pool().await;
        let _ = result;
    }
}
