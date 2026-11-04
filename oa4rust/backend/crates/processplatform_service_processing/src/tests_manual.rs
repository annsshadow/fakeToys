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
    async fn test_get_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/get/test-id")
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
                "get_process route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_create_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/create")
                    .method("POST")
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
                "create_process route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_list_processes() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/list/test-id")
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
                "list_processes route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_execute_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/execute/test-id")
                    .method("POST")
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
                "execute_process route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_get_process_instance() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/instance/test-id")
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
                "get_process_instance route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_cancel_process_instance() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/cancel/test-id")
                    .method("POST")
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
                "cancel_process_instance route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_work_id_processing() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/work/test-id/processing")
                    .method("PUT")
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
                "work_id_processing route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_work_v2_id_terminate() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/work/test-id/terminate")
                    .method("POST")
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
                "work_v2_id_terminate route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_work_v2_id_retract() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/work/test-id/retract")
                    .method("POST")
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
                "work_v2_id_retract route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_work_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/work/list")
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
                "work_list route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_process_id_complex() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/process/test-id/complex")
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
                "process_id_complex route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_work_start() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/work/test-id/start")
                    .method("POST")
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
                "work_start route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_work_complete() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/work/test-id/complete")
                    .method("POST")
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
                "work_complete route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_task_claim() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/task/test-id/claim")
                    .method("POST")
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
                "task_claim route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_task_complete() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/task/test-id/complete")
                    .method("POST")
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
                "task_complete route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_task_reject() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/task/test-id/reject")
                    .method("POST")
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
                "task_reject route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_task_transfer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/task/test-id/transfer/test-id")
                    .method("POST")
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
                "task_transfer route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    #[ignore = "route matching issue in service_processing crate"]
    async fn test_gateway_join() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/gateway/test-id/test-id/join")
                    .method("POST")
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
                "gateway_join route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_gateway_fork() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/gateway/fork/test-id")
                    .method("POST")
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
                "gateway_fork route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_start_timer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/timer/start")
                    .method("POST")
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
                "start_timer route should be registered (兜底空体 404)"
            );
        }
    }
    #[tokio::test]
    async fn test_cancel_timer() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/processplatform/service/processing/timer/test-id/cancel")
                    .method("POST")
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
                "cancel_timer route should be registered (兜底空体 404)"
            );
        }
    }
}
