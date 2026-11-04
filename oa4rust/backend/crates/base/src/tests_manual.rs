// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// 手写测试（O14 从 tests_generated.rs 原样迁出；本文件由人工维护，
// 生成器不会覆盖）。新增手写用例请直接加在这里。
#[cfg(test)]
mod tests_manual {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;

    use tower::util::ServiceExt;

    // ── plan002 U2：新增 5 条端点的路由注册测试 ──

    #[tokio::test]
    async fn test_cache_receive() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/base/cache")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "cache receive route should be registered"
        );
    }
    #[tokio::test]
    async fn test_cache_config_flush() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/base/cache/config/flush")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "cache config/flush route should be registered"
        );
    }
    #[tokio::test]
    async fn test_cache_commonscript_flush() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/base/cache/commonscript/flush")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "cache commonscript/flush route should be registered"
        );
    }
    #[tokio::test]
    async fn test_fireschedule_execute() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/base/fireschedule/classname/com.x.processplatform.service.processing.ScheduleApplication")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "fireschedule execute route should be registered"
        );
    }
    #[tokio::test]
    async fn test_sysresource_list() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/base/sysresource/filePath/(0)")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "sysresource listResource route should be registered"
        );
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
