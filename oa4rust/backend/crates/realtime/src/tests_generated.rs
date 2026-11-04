// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_ws_handler() {
        let app = crate::ws_route();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/ws/realtime")
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
            assert!(!bytes.is_empty(), "ws_handler route should be registered");
        }
    }

    #[tokio::test]
    async fn test_ws_room_handler() {
        let app = crate::ws_route();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/ws/realtime/room/test-id")
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
                "ws_room_handler route should be registered"
            );
        }
    }

    #[tokio::test]
    async fn test_ws_stats() {
        let app = crate::ws_route();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/ws/realtime/room/test-id/stats")
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
            assert!(!bytes.is_empty(), "ws_stats route should be registered");
        }
    }

    // SKIPPED: join not accessible
    // SKIPPED: leave not accessible
    // SKIPPED: broadcast not accessible
    // SKIPPED: broadcast_im not accessible
    // SKIPPED: register_sender not accessible
    // SKIPPED: presence not accessible
    // SKIPPED: room_connections not accessible
    // SKIPPED: list_rooms not accessible
}
