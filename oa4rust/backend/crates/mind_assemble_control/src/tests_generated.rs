// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_get_control_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/mind/assemble/control/config")
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
                "get_control_config route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_get_folder() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/mind/assemble/control/folder/test-id")
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
            assert!(!bytes.is_empty(), "get_folder route should be registered");
        }
    }

    // SKIPPED: save_folder requires Session parameter
    // SKIPPED: update_control_config requires Session parameter
    // SKIPPED: list_folders not accessible
    // SKIPPED: update_folder requires Session parameter
    // SKIPPED: folder_move_folderId requires Session parameter
    // SKIPPED: folder_id not accessible
    // SKIPPED: folder_id_force requires Session parameter
    // SKIPPED: folder_delete requires Session parameter
    // SKIPPED: mind_filter_list not accessible
    // SKIPPED: mind_filter_received requires Session parameter
    // SKIPPED: mind_filter_recycle requires Session parameter
    // SKIPPED: mind_filter_shared requires Session parameter
    // SKIPPED: mind_share_records not accessible
    // SKIPPED: mind_version_list not accessible
    // SKIPPED: mind_recycle requires Session parameter
    // SKIPPED: mind_restore requires Session parameter
    // SKIPPED: mind_save requires Session parameter
    // SKIPPED: mind_share requires Session parameter
    // SKIPPED: mind_share_cancel requires Session parameter
    // SKIPPED: mind_version_get not accessible
    // SKIPPED: mind_view not accessible
    // SKIPPED: mind_get not accessible
    // SKIPPED: mind_destroy requires Session parameter
    // SKIPPED: mind_destroy_recycle requires Session parameter
    // SKIPPED: mind_icon_get not accessible
    // SKIPPED: mind_icon_set requires Session parameter
}
