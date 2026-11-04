// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_uuid_random() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool, shared::testing::mock_db().await);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/cms/uuid/random")
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
            assert!(!bytes.is_empty(), "uuid_random route should be registered");
        }
    }

    #[tokio::test]
    async fn test_template_form_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool, shared::testing::mock_db().await);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/cms/templateform/list")
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
                "template_form_list route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_view_list_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool, shared::testing::mock_db().await);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/cms/view/list/all")
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
                "view_list_all route should be registered"
            );
        }
    }

    // SKIPPED: view_publish requires Session parameter
    // SKIPPED: view_unpublish requires Session parameter
}
