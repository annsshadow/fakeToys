#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_echo_get() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/base/echo/get")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "echo_get route should be registered");
    }

    #[tokio::test]
    async fn test_cache_detail() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/base/cache/detail")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "cache_detail route should be registered");
    }

    #[tokio::test]
    async fn test_openapi_info() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/base/openapi/info")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "openapi_info route should be registered");
    }

    // ── plan002 U2：新增 5 条端点的路由注册测试 ──

    #[tokio::test]
    async fn test_cache_receive() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/base/cache")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "cache receive route should be registered");
    }

    #[tokio::test]
    async fn test_cache_config_flush() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/base/cache/config/flush")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "cache config/flush route should be registered");
    }

    #[tokio::test]
    async fn test_cache_commonscript_flush() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/base/cache/commonscript/flush")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "cache commonscript/flush route should be registered");
    }

    #[tokio::test]
    async fn test_fireschedule_execute() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/base/fireschedule/classname/com.x.processplatform.service.processing.ScheduleApplication")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "fireschedule execute route should be registered");
    }

    #[tokio::test]
    async fn test_sysresource_list() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/base/sysresource/filePath/(0)")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "sysresource listResource route should be registered");
    }

    // ── 纯逻辑测试（无 DB）：路径穿越防护 ──

    #[test]
    fn test_sanitize_resource_path_root_forms() {
        assert_eq!(crate::sanitize_resource_path("(0)"), Some(vec![]));
        assert_eq!(crate::sanitize_resource_path(""), Some(vec![]));
        assert_eq!(
            crate::sanitize_resource_path("a/b/c"),
            Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])
        );
    }

    #[test]
    fn test_sanitize_resource_path_rejects_traversal() {
        // 目录穿越必须被拒绝 —— 防止越权读取 Web 根之外的任意文件
        assert_eq!(crate::sanitize_resource_path("../etc"), None);
        assert_eq!(crate::sanitize_resource_path("a/../b"), None);
        assert_eq!(crate::sanitize_resource_path("/absolute"), None);
        assert_eq!(crate::sanitize_resource_path(r"back\slash"), None);
        assert_eq!(crate::sanitize_resource_path("C:drive"), None);
        assert_eq!(crate::sanitize_resource_path("bad space"), None);
    }
}