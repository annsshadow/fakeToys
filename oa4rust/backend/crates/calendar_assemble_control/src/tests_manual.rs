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

    // plan002 U2: route-registration tests for the 7 newly added endpoints

    #[tokio::test]
    async fn test_u2_calendar_list_my() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/calendar_assemble_control/calendar/list/my")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 calendar list/my route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_calendar_list_public() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/calendar_assemble_control/calendar/list/public")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 calendar list/public route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_calendar_get() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/calendar_assemble_control/calendar/some-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 calendar {{id}} route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_calendar_ismanager() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/calendar_assemble_control/calendar/ismanager")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 calendar ismanager route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_event_get() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/calendar_assemble_control/event/some-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 event {{id}} route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_setting_list_all() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/calendar_assemble_control/setting/list/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 setting list/all route should be registered"
        );
    }
    #[tokio::test]
    async fn test_u2_setting_ismanager() {
        let app = crate::router(shared::testing::test_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/calendar_assemble_control/setting/ismanager")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "u2 setting ismanager route should be registered"
        );
    }
}
