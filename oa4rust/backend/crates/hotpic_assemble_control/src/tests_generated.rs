// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_list_hotpics() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/hotpic/list/hotpics")
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
            assert!(!bytes.is_empty(), "list_hotpics route should be registered");
        }
    }

    #[tokio::test]
    async fn test_get_hotpic() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/hotpic/get/hotpic/test-id")
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
            assert!(!bytes.is_empty(), "get_hotpic route should be registered");
        }
    }

    // SKIPPED: create_hotpic requires Session parameter
    // SKIPPED: get_control_config not accessible
    // SKIPPED: list_control_panels not accessible
    // SKIPPED: update_control_config requires Session parameter
    // SKIPPED: list_control_applications not accessible
    // SKIPPED: save_hotpic requires Session parameter
    // SKIPPED: delete_hotpic requires Session parameter
    // SKIPPED: cipher_hotpic_bbs_id not accessible
    // SKIPPED: cipher_hotpic_cms_id not accessible
    // SKIPPED: cipher_hotpic_filter_list_page_page_count_count not accessible
    // SKIPPED: cipher_hotpic_id not accessible
    // SKIPPED: user_hotpic_changeTitle requires Session parameter
    // SKIPPED: user_hotpic_exists_check not accessible
    // SKIPPED: user_hotpic_filter_list_page_page_count_count not accessible
    // SKIPPED: user_hotpic_application_infoId not accessible
    // SKIPPED: user_hotpic_id requires Session parameter
    // SKIPPED: user_hotpic_delete_by_ids requires Session parameter
}
