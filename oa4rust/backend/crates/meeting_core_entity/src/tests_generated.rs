// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_room_list() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db, shared::testing::mock_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/core/entity/room/list")
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
            assert!(!bytes.is_empty(), "room_list route should be registered");
        }
    }

    // SKIPPED: create_room requires Session parameter
    #[tokio::test]
    async fn test_get_room() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db, shared::testing::mock_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/core/entity/room/test-id")
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
            assert!(!bytes.is_empty(), "get_room route should be registered");
        }
    }

    // SKIPPED: update_room requires Session parameter
    #[tokio::test]
    async fn test_meeting_list() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db, shared::testing::mock_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/core/entity/meeting/list")
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
            assert!(!bytes.is_empty(), "meeting_list route should be registered");
        }
    }

    #[tokio::test]
    async fn test_get_meeting() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db, shared::testing::mock_pool());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/meeting/core/entity/meeting/test-id")
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
            assert!(!bytes.is_empty(), "get_meeting route should be registered");
        }
    }

    // SKIPPED: delete_room requires Session parameter
    // SKIPPED: meeting_list_by_room not accessible
    // SKIPPED: create_meeting requires Session parameter
    // SKIPPED: update_meeting requires Session parameter
    // SKIPPED: delete_meeting requires Session parameter
}
