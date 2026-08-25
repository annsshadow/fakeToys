#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_check() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/secret/check")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "check route should be registered");
    }

    #[tokio::test]
    async fn test_set() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/secret/set")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "set route should be registered");
    }

    #[tokio::test]
    async fn test_set_cancel() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/secret/set/cancel")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "set_cancel route should be registered");
    }

    // ── plan002 U2：新增 12 条端点的路由注册测试（不依赖数据库内容）──

    #[tokio::test]
    async fn test_externaldatasources_check() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/externaldatasources/check")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "externaldatasources/check route should be registered");
    }

    #[tokio::test]
    async fn test_externaldatasources_list() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/externaldatasources/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "externaldatasources/list route should be registered");
    }

    #[tokio::test]
    async fn test_externaldatasources_set() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/externaldatasources/set")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "externaldatasources/set route should be registered");
    }

    #[tokio::test]
    async fn test_externaldatasources_set_cancel() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/externaldatasources/set/cancel")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "externaldatasources/set/cancel route should be registered");
    }

    #[tokio::test]
    async fn test_externaldatasources_validate() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/externaldatasources/validate")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "externaldatasources/validate route should be registered");
    }

    #[tokio::test]
    async fn test_h2_check() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/h2/check")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "h2/check route should be registered");
    }

    #[tokio::test]
    async fn test_restore_upload() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/restore/upload")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "restore/upload route should be registered");
    }

    #[tokio::test]
    async fn test_restore_upload_cancel() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/restore/upload/cancel")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "restore/upload/cancel route should be registered");
    }

    #[tokio::test]
    async fn test_server_execute() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/server/execute")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "server/execute route should be registered");
    }

    #[tokio::test]
    async fn test_server_execute_status() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/server/execute/status")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "server/execute/status route should be registered");
    }

    #[tokio::test]
    async fn test_server_license() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/server/license")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "server/license route should be registered");
    }

    #[tokio::test]
    async fn test_server_stop() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/server/stop")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "server/stop route should be registered");
    }

    // ── 纯逻辑测试（无 DB）：jdbc url 解析 ──

    #[test]
    fn test_parse_jdbc_host_port_jdbc_form() {
        let parsed = crate::parse_jdbc_host_port("jdbc:postgresql://dbhost:5433/o2oa");
        assert_eq!(parsed, Some(("dbhost".to_string(), 5433)));
    }

    #[test]
    fn test_parse_jdbc_host_port_standard_form_with_credentials() {
        let parsed = crate::parse_jdbc_host_port("postgresql://u:p@10.0.0.8:6542/xdb?ssl=true");
        assert_eq!(parsed, Some(("10.0.0.8".to_string(), 6542)));
    }

    #[test]
    fn test_parse_jdbc_host_port_defaults_and_garbage() {
        assert_eq!(
            crate::parse_jdbc_host_port("postgresql://onlyhost"),
            Some(("onlyhost".to_string(), 5432))
        );
        assert_eq!(crate::parse_jdbc_host_port("not-a-url"), None);
        assert_eq!(crate::parse_jdbc_host_port("postgresql://:9999/db"), None);
    }

}