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

    #[tokio::test]
    async fn test_query_list() {
        // O9：router 改注 sea_orm DatabaseConnection（接线修复）
        let app = crate::router(shared::testing::mock_db().await);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "query_list route should be registered"
        );
    }
}
