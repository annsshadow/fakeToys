// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_process_query() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/service/processing/process")
                    .method("POST")
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
                "process_query route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_batch_process() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/query/service/processing/batch")
                    .method("POST")
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
                "batch_process route should be registered"
            );
        }
    }

    // SKIPPED: reset_service requires Session parameter
    // SKIPPED: get_service_status not accessible
    // SKIPPED: design_search not accessible
    // SKIPPED: index_directory_document_count not accessible
    // SKIPPED: index_update_extra_document requires Session parameter
    // SKIPPED: optimize_index_touch requires Session parameter
    // SKIPPED: table_insert requires Session parameter
    // SKIPPED: table_update_with_bundle requires Session parameter
    // SKIPPED: table_reload_dynamic requires Session parameter
    // SKIPPED: neural_generate requires Session parameter
    // SKIPPED: neural_stop_generating requires Session parameter
    // SKIPPED: neural_learn requires Session parameter
    // SKIPPED: neural_stop_learning requires Session parameter
    // SKIPPED: neural_list_calculate_with_work not accessible
}
