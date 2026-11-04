// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use axum::body::Body;
    use axum::http::{Request, StatusCode};

    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_calendar_get() {
        let db = shared::testing::mock_db().await;
        let app = crate::router(db);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/calendar/core/entity/calendar/test-id")
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
            assert!(!bytes.is_empty(), "calendar_get route should be registered");
        }
    }

    // SKIPPED: event_create requires Session parameter
    // SKIPPED: event_update requires Session parameter
    // SKIPPED: event_remove requires Session parameter
    // SKIPPED: calendar_list_public not accessible
    // SKIPPED: calendar_list_my not accessible
    // SKIPPED: calendar_create requires Session parameter
    // SKIPPED: calendar_update requires Session parameter
    // SKIPPED: calendar_remove requires Session parameter
    // SKIPPED: event_list_by_calendar not accessible
}
