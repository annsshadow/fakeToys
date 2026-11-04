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
    async fn test_anonymous_file_id_download() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/anonymous/file/id/download")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let __st = response.status();
        let __body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert!(
            __st != StatusCode::NOT_FOUND || !__body.is_empty(),
            "anonymous_file_id_download route should be registered (handler 返回体表明已路由; 空体 404 才是未注册)"
        );
    }
    #[tokio::test]
    #[ignore = "DB schema mismatch: create_time deserialization"]
    async fn test_attachment_list_top() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/attachment/list/top")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "attachment_list_top route should be registered"
        );
    }
    #[tokio::test]
    async fn test_attachment_id_download() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/attachment/id/download")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let __st = response.status();
        let __body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert!(
            __st != StatusCode::NOT_FOUND || !__body.is_empty(),
            "attachment_id_download route should be registered (handler 返回体表明已路由; 空体 404 才是未注册)"
        );
    }
    #[tokio::test]
    #[ignore = "DB schema mismatch: create_time deserialization"]
    async fn test_attachment2_list_top() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/attachment2/list/top")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(
            response.status(),
            StatusCode::NOT_FOUND,
            "attachment2_list_top route should be registered"
        );
    }
    #[tokio::test]
    async fn test_attachment2_id_download() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/attachment2/id/download")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let __st = response.status();
        let __body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert!(
            __st != StatusCode::NOT_FOUND || !__body.is_empty(),
            "attachment2_id_download route should be registered (handler 返回体表明已路由; 空体 404 才是未注册)"
        );
    }
    #[tokio::test]
    async fn test_attachment2_id_download_image_width_width_height_height() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/attachment2/id/download/image/width/width/height/height")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let __st = response.status();
        let __body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert!(
            __st != StatusCode::NOT_FOUND || !__body.is_empty(),
            "attachment2_id_download_image_width_width_height_height route should be registered (handler 返回体表明已路由; 空体 404 才是未注册)"
        );
    }
    #[tokio::test]
    async fn test_attachment2_id_download_stream() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/attachment2/id/download/stream")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let __st = response.status();
        let __body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert!(
            __st != StatusCode::NOT_FOUND || !__body.is_empty(),
            "attachment2_id_download_stream route should be registered (handler 返回体表明已路由; 空体 404 才是未注册)"
        );
    }
    #[tokio::test]
    async fn test_folder2_id_download() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/folder2/id/download")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        // handler 已真实现：路由在（非空 body），对不存在的文件夹有意返回 404 错误信封。
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert!(
            !bytes.is_empty(),
            "folder2_id_download route should be registered and answer with an envelope"
        );
    }
    #[tokio::test]
    async fn test_share_download_share_shareId_file_fileId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/file/share/download/share/shareId/file/fileId")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let __st = response.status();
        let __body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert!(
            __st != StatusCode::NOT_FOUND || !__body.is_empty(),
            "share_download_share_shareId_file_fileId route should be registered (handler 返回体表明已路由; 空体 404 才是未注册)"
        );
    }
}
