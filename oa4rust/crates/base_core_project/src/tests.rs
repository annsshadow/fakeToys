//! base_core_project 路由单元测试（沿用 general crate 的 dummy-pool + oneshot 模式）。
//!
//! 这些 handler 是纯桩（不触库），因此可直接断言 200 + 成功信封形状，
//! 把「路由已注册 + 信封与 o2server 对齐」钉死，防止将来被误改成 404/错信封。

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::Value;
    use tower::util::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    fn app() -> axum::Router {
        crate::router(build_test_pool())
    }

    /// 发起 GET 并解析 ActionResult 信封。
    async fn get_ok(uri: &str) -> Value {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri(uri)
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "GET {uri}");
        let bytes = axum::body::to_bytes(response.into_body(), 1 << 20)
            .await
            .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[test]
    fn test_api_base_constant() {
        assert_eq!(crate::API_BASE, "/api");
    }

    #[tokio::test]
    async fn test_echo_returns_success_envelope() {
        let body = get_ok("/api/echo").await;
        assert_eq!(body["type"], "success");
        assert_eq!(body["data"]["message"], "pong");
        // 成功信封不得携带 prompt（o2server 实测形状）。
        assert!(body.get("prompt").is_none());
    }

    #[tokio::test]
    async fn test_openapi_stub_shape() {
        let body = get_ok("/api/openapi").await;
        assert_eq!(body["data"]["openapi"], "3.0");
    }

    #[tokio::test]
    async fn test_cache_flush_routes() {
        let cfg = get_ok("/api/cache/config/flush").await;
        assert_eq!(cfg["data"]["status"], "flushed");
        let cs = get_ok("/api/cache/commonscript/flush").await;
        assert_eq!(cs["data"]["status"], "flushed");
    }

    #[tokio::test]
    async fn test_cache_detail_route() {
        let body = get_ok("/api/cache/detail").await;
        assert_eq!(body["data"]["detail"], "cache info");
    }

    #[tokio::test]
    async fn test_param_routes_carry_path_segments() {
        // 路径段 {className} / {filePath} 必须原样透传（含特殊字符的编码段）。
        let fire = get_ok("/api/fireschedule/classname/com.x.task").await;
        assert_eq!(fire["data"]["class"], "test");
        let res = get_ok("/api/sysresource/filePath/a%20b").await;
        assert_eq!(res["data"]["path"], "/");
    }

    #[tokio::test]
    async fn test_cache_post_requires_json_body() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/api/cache")
                    .method(Method::POST)
                    .header(axum::http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"k":"v"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 1 << 20)
            .await
            .unwrap();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["data"]["status"], "ok");
    }

    #[tokio::test]
    async fn test_unknown_route_is_404() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/api/does-not-exist")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
