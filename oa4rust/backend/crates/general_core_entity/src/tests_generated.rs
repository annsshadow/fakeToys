// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_dict_list() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/dict/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "dict_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_dict_item_list() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/dict/item/list/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(
                !bytes.is_empty(),
                "dict_item_list route should be registered"
            );
        }
    }

    // SKIPPED: dict_create requires Session parameter
    #[tokio::test]
    async fn test_dict_get() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/dict/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "dict_get route should be registered");
        }
    }

    // SKIPPED: dict_update requires Session parameter
    // SKIPPED: dict_delete requires Session parameter
    // SKIPPED: dict_item_create requires Session parameter
    #[tokio::test]
    async fn test_dict_item_get() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/dict/item/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(
                !bytes.is_empty(),
                "dict_item_get route should be registered"
            );
        }
    }

    // SKIPPED: dict_item_update requires Session parameter
    // SKIPPED: dict_item_delete requires Session parameter
    #[tokio::test]
    async fn test_file_list() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/file/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "file_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_invoice_list() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/invoice/list")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "invoice_list route should be registered");
        }
    }

    // SKIPPED: file_create requires Session parameter
    #[tokio::test]
    async fn test_file_get() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/file/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "file_get route should be registered");
        }
    }

    // SKIPPED: file_update requires Session parameter
    // SKIPPED: file_delete requires Session parameter
    #[tokio::test]
    async fn test_file_download() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/file/download/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(
                !bytes.is_empty(),
                "file_download route should be registered"
            );
        }
    }

    // SKIPPED: invoice_create requires Session parameter
    #[tokio::test]
    async fn test_invoice_get() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/general/invoice/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::NOT_FOUND {
            // Route-presence probe: an unregistered path hits the axum fallback
            // (empty 404 body); a matched handler may legitimately answer 404
            // (e.g. NotFound for missing row) but always with a JSON envelope.
            let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            assert!(!bytes.is_empty(), "invoice_get route should be registered");
        }
    }

    // SKIPPED: invoice_update requires Session parameter
    // SKIPPED: invoice_delete requires Session parameter
}
