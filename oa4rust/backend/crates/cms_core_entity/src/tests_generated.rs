// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_category_list() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db, shared::testing::mock_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/cms/category/list")
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
                "category_list route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_category_get() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db, shared::testing::mock_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/cms/category/test-id")
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
            assert!(!bytes.is_empty(), "category_get route should be registered");
        }
    }

    // SKIPPED: category_create requires Session parameter
    #[tokio::test]
    async fn test_article_list() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db, shared::testing::mock_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/cms/article/list")
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
            assert!(!bytes.is_empty(), "article_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_article_get() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db, shared::testing::mock_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/cms/article/test-id")
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
            assert!(!bytes.is_empty(), "article_get route should be registered");
        }
    }

    // SKIPPED: article_create requires Session parameter
    #[tokio::test]
    async fn test_cms_entity_note_list() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db, shared::testing::mock_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/cms/core/entity/note/list")
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
                "cms_entity_note_list route should be registered"
            );
        }
    }

    // SKIPPED: cms_entity_column_list not accessible
    // SKIPPED: cms_entity_column_manager_list not accessible
    // SKIPPED: cms_entity_index_list not accessible
    // SKIPPED: cms_entity_module_list not accessible
    // SKIPPED: cms_entity_column_create requires Session parameter
    // SKIPPED: cms_entity_column_save requires Session parameter
    // SKIPPED: cms_entity_column_delete requires Session parameter
    // SKIPPED: cms_entity_column_manager_create requires Session parameter
    // SKIPPED: cms_entity_column_manager_save requires Session parameter
    // SKIPPED: cms_entity_column_manager_delete requires Session parameter
    // SKIPPED: cms_entity_index_create requires Session parameter
    // SKIPPED: cms_entity_index_save requires Session parameter
    // SKIPPED: cms_entity_index_delete requires Session parameter
    // SKIPPED: cms_entity_module_create requires Session parameter
    // SKIPPED: cms_entity_module_save requires Session parameter
    // SKIPPED: cms_entity_module_delete requires Session parameter
    // SKIPPED: cms_entity_note_create requires Session parameter
    // SKIPPED: cms_entity_note_save requires Session parameter
    // SKIPPED: cms_entity_note_delete requires Session parameter
}
