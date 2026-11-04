// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// 手写测试（O14 从 tests_generated.rs 原样迁出；本文件由人工维护，
// 生成器不会覆盖）。新增手写用例请直接加在这里。
#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;

    use tower::util::ServiceExt;

    // plan002 U2 residual closure: route-registration tests for the 9 new endpoints

    #[tokio::test]
    async fn test_u2_config_centerserver_put() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/program_center/config/centerserver")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 config centerserver PUT route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_config_person_put() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/program_center/config/person")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 config person PUT route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_config_token_put() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/program_center/config/token")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 config token PUT route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_invoke_create() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/program_center/invoke")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 invoke create route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_invoke_get() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/program_center/invoke/some-flag")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 invoke get route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_invoke_update() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/program_center/invoke/some-flag")
                    .method("PUT")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 invoke update route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_invoke_delete() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/program_center/invoke/some-flag")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 invoke delete route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_appstyle_login_avatar_erase_get() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/program_center/appstyle/image/login/avatar/erase")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 appstyle login avatar erase GET route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_appstyle_launch_logo_erase_get() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/program_center/appstyle/image/launch/logo/erase")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 appstyle launch logo erase GET route should be registered"
        );
    }
}
