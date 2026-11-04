// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// 手写测试（O14 从 tests_generated.rs 原样迁出；本文件由人工维护，
// 生成器不会覆盖）。新增手写用例请直接加在这里。
#[cfg(test)]
mod tests_manual {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::Request;

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_im_msg_download_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/message/assemble/communicate/im/msg/download/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // Route-presence probe: an unregistered path hits the axum fallback
        // (empty 404 body); this handler may legitimately answer 404 for a
        // missing x_message_file row but always with a body, so assert the
        // body is non-empty instead of the status.
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap_or_default();
        assert!(
            !bytes.is_empty(),
            "im_msg_download_id route should be registered"
        );
    }
}
