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

    #[tokio::test]
    async fn test_work_terminate() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/work/terminate/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // O10：handler 对不存在 id 正确返回 404（RowNotFound→NotFound）后，「≠404」口径
        // 会把「可达+业务 404」误判为不可达。与生成器模板/arity_probe 同口径：仅 axum
        // 兜底 404（空体）判不可达；handler 的 404 带 JSON 错误体 = 可达+业务正常。
        if response.status() == StatusCode::NOT_FOUND {
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(
                !bytes.is_empty(),
                "work_terminate route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_work_retract() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/work/retract/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // O10：handler 对不存在 id 正确返回 404（RowNotFound→NotFound）后，「≠404」口径
        // 会把「可达+业务 404」误判为不可达。与生成器模板/arity_probe 同口径：仅 axum
        // 兜底 404（空体）判不可达；handler 的 404 带 JSON 错误体 = 可达+业务正常。
        if response.status() == StatusCode::NOT_FOUND {
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(
                !bytes.is_empty(),
                "work_retract route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_work_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/work/processing/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // O10：handler 对不存在 id 正确返回 404（RowNotFound→NotFound）后，「≠404」口径
        // 会把「可达+业务 404」误判为不可达。与生成器模板/arity_probe 同口径：仅 axum
        // 兜底 404（空体）判不可达；handler 的 404 带 JSON 错误体 = 可达+业务正常。
        if response.status() == StatusCode::NOT_FOUND {
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(
                !bytes.is_empty(),
                "work_processing route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_task_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/task/processing/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // O10：handler 对不存在 id 正确返回 404（RowNotFound→NotFound）后，「≠404」口径
        // 会把「可达+业务 404」误判为不可达。与生成器模板/arity_probe 同口径：仅 axum
        // 兜底 404（空体）判不可达；handler 的 404 带 JSON 错误体 = 可达+业务正常。
        if response.status() == StatusCode::NOT_FOUND {
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(
                !bytes.is_empty(),
                "task_processing route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_work_count_with_person() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/work/count/with/person/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // O10：handler 对不存在 id 正确返回 404（RowNotFound→NotFound）后，「≠404」口径
        // 会把「可达+业务 404」误判为不可达。与生成器模板/arity_probe 同口径：仅 axum
        // 兜底 404（空体）判不可达；handler 的 404 带 JSON 错误体 = 可达+业务正常。
        if response.status() == StatusCode::NOT_FOUND {
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(
                !bytes.is_empty(),
                "work_count_with_person route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_task_count_with_person() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/task/count/with/person/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // O10：handler 对不存在 id 正确返回 404（RowNotFound→NotFound）后，「≠404」口径
        // 会把「可达+业务 404」误判为不可达。与生成器模板/arity_probe 同口径：仅 axum
        // 兜底 404（空体）判不可达；handler 的 404 带 JSON 错误体 = 可达+业务正常。
        if response.status() == StatusCode::NOT_FOUND {
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(
                !bytes.is_empty(),
                "task_count_with_person route should be registered (兜底空体 404)"
            );
        }
    }
}
